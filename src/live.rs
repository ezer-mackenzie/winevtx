use pyo3::prelude::*;
use crate::error::{WinevtxError, Result};
use crate::record::EventRecord;

#[cfg(windows)]
mod imp {
    use super::*;
    use std::ptr::null_mut;
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::System::EventLog::*;

    pub struct EvtHandle(pub EVT_HANDLE);

    impl Drop for EvtHandle {
        fn drop(&mut self) {
            if self.0 != 0 {
                unsafe {
                    EvtClose(self.0);
                }
                self.0 = 0;
            }
        }
    }

    unsafe impl Send for EvtHandle {}
    unsafe impl Sync for EvtHandle {}

    pub fn render_event_xml(event_handle: EVT_HANDLE) -> Result<String> {
        unsafe {
            let mut buffer_used = 0;
            let mut property_count = 0;

            // Probe buffer size
            EvtRender(
                0,
                event_handle,
                EvtRenderEventXml,
                0,
                null_mut(),
                &mut buffer_used,
                &mut property_count,
            );

            let err = GetLastError();
            if err != ERROR_INSUFFICIENT_BUFFER {
                return Err(WinevtxError::WindowsApi(
                    "Failed to probe XML buffer size".to_string(),
                    err,
                ));
            }

            let wchar_count = (buffer_used / 2) as usize;
            let mut buffer: Vec<u16> = vec![0; wchar_count];

            let res = EvtRender(
                0,
                event_handle,
                EvtRenderEventXml,
                buffer_used,
                buffer.as_mut_ptr() as _,
                &mut buffer_used,
                &mut property_count,
            );

            if res == 0 {
                let err = GetLastError();
                return Err(WinevtxError::WindowsApi(
                    "Failed to render event XML".to_string(),
                    err,
                ));
            }

            let len = if wchar_count > 0 && buffer[wchar_count - 1] == 0 {
                wchar_count - 1
            } else {
                wchar_count
            };

            Ok(String::from_utf16_lossy(&buffer[..len]))
        }
    }

    pub fn query_channel(
        channel: &str,
        query: &str,
        reverse: bool,
    ) -> Result<EvtHandle> {
        let channel_w: Vec<u16> = channel.encode_utf16().chain(std::iter::once(0)).collect();
        let query_w: Vec<u16> = query.encode_utf16().chain(std::iter::once(0)).collect();

        let mut flags = EvtQueryChannelPath;
        if reverse {
            flags |= EvtQueryReverseDirection;
        } else {
            flags |= EvtQueryForwardDirection;
        }

        unsafe {
            let handle = EvtQuery(0, channel_w.as_ptr(), query_w.as_ptr(), flags);
            if handle == 0 {
                let err = GetLastError();
                return Err(WinevtxError::WindowsApi(
                    format!("Failed to query channel '{}' with query '{}'", channel, query),
                    err,
                ));
            }
            Ok(EvtHandle(handle))
        }
    }

    pub fn fetch_next_events(query_handle: EVT_HANDLE, count: u32, timeout_ms: u32) -> Result<Vec<EvtHandle>> {
        let mut raw_handles = vec![0 as EVT_HANDLE; count as usize];
        let mut returned = 0;

        unsafe {
            let res = EvtNext(
                query_handle,
                count,
                raw_handles.as_mut_ptr(),
                timeout_ms,
                0,
                &mut returned,
            );

            if res == 0 {
                let err = GetLastError();
                if err == ERROR_NO_MORE_ITEMS {
                    return Ok(Vec::new());
                }
                return Err(WinevtxError::WindowsApi(
                    "EvtNext failed".to_string(),
                    err,
                ));
            }

            let mut out = Vec::with_capacity(returned as usize);
            for i in 0..returned as usize {
                out.push(EvtHandle(raw_handles[i]));
            }
            Ok(out)
        }
    }
}

#[pyclass(name = "LiveEventLog")]
pub struct LiveEventLog {
    channel: String,
    query: String,
    reverse: bool,
}

#[pymethods]
impl LiveEventLog {
    #[new]
    #[pyo3(signature = (channel="System", query="*", reverse=true))]
    pub fn new(channel: &str, query: &str, reverse: bool) -> Self {
        Self {
            channel: channel.to_string(),
            query: query.to_string(),
            reverse,
        }
    }

    #[pyo3(signature = (limit=None, format="record"))]
    pub fn read<'py>(
        &self,
        py: Python<'py>,
        limit: Option<usize>,
        format: &str,
    ) -> PyResult<Vec<Bound<'py, PyAny>>> {
        let iter = self.iter(format)?;
        let mut results = Vec::new();
        let max_events = limit.unwrap_or(usize::MAX);

        let iter_bound = Bound::new(py, iter)?.into_any();
        while results.len() < max_events {
            match iter_bound.call_method0("__next__") {
                Ok(item) => results.push(item),
                Err(err) if err.is_instance_of::<pyo3::exceptions::PyStopIteration>(py) => break,
                Err(err) => return Err(err),
            }
        }
        Ok(results)
    }

    #[pyo3(signature = (format="record"))]
    pub fn iter(&self, format: &str) -> PyResult<LiveEventIterator> {
        #[cfg(windows)]
        {
            let handle = imp::query_channel(&self.channel, &self.query, self.reverse)?;
            Ok(LiveEventIterator {
                handle: Some(handle),
                format: format.to_string(),
                buffer: std::collections::VecDeque::new(),
            })
        }
        #[cfg(not(windows))]
        {
            let _ = format;
            Err(pyo3::exceptions::PyRuntimeError::new_err(
                "Live Windows Event Log API is only supported on Windows",
            ))
        }
    }

    pub fn __iter__(&self) -> PyResult<LiveEventIterator> {
        self.iter("record")
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<LiveEventLog channel={:?} query={:?} reverse={:?}>",
            self.channel, self.query, self.reverse
        )
    }
}

#[pyclass(name = "LiveEventIterator")]
pub struct LiveEventIterator {
    #[cfg(windows)]
    handle: Option<imp::EvtHandle>,
    format: String,
    buffer: std::collections::VecDeque<String>,
}

#[pymethods]
impl LiveEventIterator {
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        #[cfg(windows)]
        {
            if let Some(xml) = self.buffer.pop_front() {
                return format_xml(py, xml, &self.format).map(Some);
            }

            let handle = match &self.handle {
                Some(h) => h.0,
                None => return Ok(None),
            };

            // Fetch a batch of 16 events with GIL released
            let next_xmls: Result<Vec<String>> = py.detach(|| {
                let handles = imp::fetch_next_events(handle, 16, 500)?;
                let mut xmls = Vec::with_capacity(handles.len());
                for h in handles {
                    let xml = imp::render_event_xml(h.0)?;
                    xmls.push(xml);
                }
                Ok(xmls)
            });

            match next_xmls {
                Ok(xmls) => {
                    if xmls.is_empty() {
                        self.handle = None;
                        return Ok(None);
                    }
                    for xml in xmls {
                        self.buffer.push_back(xml);
                    }
                    if let Some(xml) = self.buffer.pop_front() {
                        format_xml(py, xml, &self.format).map(Some)
                    } else {
                        Ok(None)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        #[cfg(not(windows))]
        {
            let _ = py;
            Ok(None)
        }
    }
}

pub fn format_xml<'py>(py: Python<'py>, xml: String, format: &str) -> PyResult<Bound<'py, PyAny>> {
    match format.to_ascii_lowercase().as_str() {
        "xml" => {
            let py_str = pyo3::types::PyString::new(py, &xml);
            Ok(py_str.into_any())
        }
        "json" => {
            let val = crate::xml_parse::xml_to_value(&xml).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("XML parse error: {}", e))
            })?;
            let json_str = serde_json::to_string(&val).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("JSON serialization error: {}", e))
            })?;
            let py_str = pyo3::types::PyString::new(py, &json_str);
            Ok(py_str.into_any())
        }
        "dict" => {
            let val = crate::xml_parse::xml_to_value(&xml).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("XML parse error: {}", e))
            })?;
            pythonize::pythonize(py, &val).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("Dict conversion error: {}", e))
            })
        }
        "record" | "" => {
            let record = EventRecord::from_xml(xml);
            let py_record = Bound::new(py, record)?;
            Ok(py_record.into_any())
        }
        other => Err(WinevtxError::InvalidFormat(format!(
            "Unsupported format: '{}'. Expected 'record', 'dict', 'xml', or 'json'",
            other
        ))
        .into()),
    }
}
