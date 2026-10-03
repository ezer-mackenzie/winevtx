use pyo3::prelude::*;
use std::path::PathBuf;
use crate::error::WinevtxError;
use crate::live::format_xml;
use evtx::EvtxParser;

#[pyclass(name = "EvtxFile")]
pub struct EvtxFile {
    path: PathBuf,
}

#[pymethods]
impl EvtxFile {
    #[new]
    pub fn new(path: &str) -> PyResult<Self> {
        let p = PathBuf::from(path);
        if !p.exists() {
            return Err(WinevtxError::FileNotFound(path.to_string()).into());
        }
        Ok(Self { path: p })
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
    pub fn iter(&self, format: &str) -> PyResult<EvtxRecordIterator> {
        // Test that file can be opened
        let _ = EvtxParser::from_path(&self.path).map_err(|e| {
            WinevtxError::EvtxParse(format!("Failed to open EVTX file: {}", e))
        })?;

        let (tx, rx) = std::sync::mpsc::sync_channel::<std::result::Result<String, String>>(128);
        let path_clone = self.path.clone();

        std::thread::spawn(move || {
            let mut parser = match EvtxParser::from_path(&path_clone) {
                Ok(p) => p,
                Err(e) => {
                    let _ = tx.send(Err(e.to_string()));
                    return;
                }
            };

            for record in parser.records() {
                match record {
                    Ok(r) => {
                        if tx.send(Ok(r.data)).is_err() {
                            return; // Consumer dropped iterator
                        }
                    }
                    Err(e) => {
                        if tx.send(Err(e.to_string())).is_err() {
                            return;
                        }
                    }
                }
            }
        });

        Ok(EvtxRecordIterator {
            receiver: std::sync::Arc::new(std::sync::Mutex::new(rx)),
            format: format.to_string(),
        })
    }

    pub fn __iter__(&self) -> PyResult<EvtxRecordIterator> {
        self.iter("record")
    }

    pub fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __exit__(
        &self,
        _exc_type: Option<&Bound<'_, PyAny>>,
        _exc_val: Option<&Bound<'_, PyAny>>,
        _exc_tb: Option<&Bound<'_, PyAny>>,
    ) -> bool {
        false
    }

    pub fn __repr__(&self) -> String {
        format!("<EvtxFile path={:?}>", self.path)
    }
}

#[pyclass(name = "EvtxRecordIterator")]
pub struct EvtxRecordIterator {
    receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<std::result::Result<String, String>>>>,
    format: String,
}

#[pymethods]
impl EvtxRecordIterator {
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__<'py>(&mut self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let rx = std::sync::Arc::clone(&self.receiver);
        let msg = py.detach(move || rx.lock().unwrap().recv());
        match msg {
            Ok(Ok(xml)) => format_xml(py, xml, &self.format).map(Some),
            Ok(Err(e)) => Err(WinevtxError::EvtxParse(e).into()),
            Err(_) => Ok(None), // channel disconnected: iteration finished
        }
    }
}
