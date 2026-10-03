use pyo3::prelude::*;
use serde_json::Value;

#[pyclass(name = "EventRecord", from_py_object)]
#[derive(Clone, Debug)]
pub struct EventRecord {
    #[pyo3(get)]
    pub xml: String,
    pub(crate) value: Value,
    #[pyo3(get)]
    pub event_id: Option<u64>,
    #[pyo3(get)]
    pub record_id: Option<u64>,
    #[pyo3(get)]
    pub channel: Option<String>,
    #[pyo3(get)]
    pub provider: Option<String>,
    #[pyo3(get)]
    pub level: Option<u64>,
    #[pyo3(get)]
    pub time_created: Option<String>,
    #[pyo3(get)]
    pub computer: Option<String>,
}

impl EventRecord {
    pub fn from_xml(xml: String) -> Self {
        let value = crate::xml_parse::xml_to_value(&xml).unwrap_or(Value::Null);
        Self::from_value_and_xml(value, xml)
    }

    pub fn from_value_and_xml(value: Value, xml: String) -> Self {
        let mut event_id = None;
        let mut record_id = None;
        let mut channel = None;
        let mut provider = None;
        let mut level = None;
        let mut time_created = None;
        let mut computer = None;

        let event_obj = value.get("Event").unwrap_or(&value);
        if let Some(sys) = event_obj.get("System") {
            if let Some(id_val) = sys.get("EventID") {
                event_id = if let Some(n) = id_val.as_u64() {
                    Some(n)
                } else if let Some(s) = id_val.as_str() {
                    s.parse().ok()
                } else if let Some(obj) = id_val.as_object() {
                    obj.get("#text")
                        .and_then(|t| t.as_u64().or_else(|| t.as_str().and_then(|s| s.parse().ok())))
                } else {
                    None
                };
            }

            if let Some(rec_val) = sys.get("EventRecordID") {
                record_id = if let Some(n) = rec_val.as_u64() {
                    Some(n)
                } else if let Some(s) = rec_val.as_str() {
                    s.parse().ok()
                } else {
                    None
                };
            }

            if let Some(ch_val) = sys.get("Channel") {
                channel = ch_val.as_str().map(|s| s.to_string());
            }

            if let Some(comp_val) = sys.get("Computer") {
                computer = comp_val.as_str().map(|s| s.to_string());
            }

            if let Some(prov_val) = sys.get("Provider") {
                if let Some(name) = prov_val.get("Name").and_then(|n| n.as_str()) {
                    provider = Some(name.to_string());
                } else if let Some(name) = prov_val.get("@Name").and_then(|n| n.as_str()) {
                    provider = Some(name.to_string());
                }
            }

            if let Some(lvl_val) = sys.get("Level") {
                level = if let Some(n) = lvl_val.as_u64() {
                    Some(n)
                } else if let Some(s) = lvl_val.as_str() {
                    s.parse().ok()
                } else {
                    None
                };
            }

            if let Some(tc_val) = sys.get("TimeCreated") {
                if let Some(st) = tc_val.get("SystemTime").and_then(|s| s.as_str()) {
                    time_created = Some(st.to_string());
                } else if let Some(st) = tc_val.get("@SystemTime").and_then(|s| s.as_str()) {
                    time_created = Some(st.to_string())
                } else if let Some(st) = tc_val.as_str() {
                    time_created = Some(st.to_string());
                }
            }
        }

        Self {
            xml,
            value,
            event_id,
            record_id,
            channel,
            provider,
            level,
            time_created,
            computer,
        }
    }
}

#[pymethods]
impl EventRecord {
    /// Convert the event into a native Python dictionary.
    pub fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        pythonize::pythonize(py, &self.value).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("Serialization error: {}", e))
        })
    }

    /// Convert the event into a JSON string.
    pub fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.value).map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!("JSON serialization error: {}", e))
        })
    }

    /// Access the System dictionary.
    #[getter]
    pub fn system<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let event_obj = self.value.get("Event").unwrap_or(&self.value);
        if let Some(sys) = event_obj.get("System") {
            let py_val = pythonize::pythonize(py, sys)?;
            Ok(Some(py_val))
        } else {
            Ok(None)
        }
    }

    /// Access the EventData dictionary or list.
    #[getter]
    pub fn event_data<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyAny>>> {
        let event_obj = self.value.get("Event").unwrap_or(&self.value);
        if let Some(ed) = event_obj.get("EventData") {
            let py_val = pythonize::pythonize(py, ed)?;
            Ok(Some(py_val))
        } else if let Some(ud) = event_obj.get("UserData") {
            let py_val = pythonize::pythonize(py, ud)?;
            Ok(Some(py_val))
        } else {
            Ok(None)
        }
    }

    pub fn get<'py>(
        &self,
        py: Python<'py>,
        key: &str,
        default: Option<Bound<'py, PyAny>>,
    ) -> PyResult<Option<Bound<'py, PyAny>>> {
        let event_obj = self.value.get("Event").unwrap_or(&self.value);
        if let Some(val) = event_obj.get(key) {
            let py_val = pythonize::pythonize(py, val)?;
            Ok(Some(py_val))
        } else if let Some(val) = self.value.get(key) {
            let py_val = pythonize::pythonize(py, val)?;
            Ok(Some(py_val))
        } else {
            Ok(default)
        }
    }

    pub fn __getitem__<'py>(&self, py: Python<'py>, key: &str) -> PyResult<Bound<'py, PyAny>> {
        let event_obj = self.value.get("Event").unwrap_or(&self.value);
        if let Some(val) = event_obj.get(key) {
            pythonize::pythonize(py, val).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("Conversion error: {}", e))
            })
        } else if let Some(val) = self.value.get(key) {
            pythonize::pythonize(py, val).map_err(|e| {
                pyo3::exceptions::PyValueError::new_err(format!("Conversion error: {}", e))
            })
        } else {
            Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Key '{}' not found in event",
                key
            )))
        }
    }

    pub fn __contains__(&self, key: &str) -> bool {
        let event_obj = self.value.get("Event").unwrap_or(&self.value);
        event_obj.get(key).is_some() || self.value.get(key).is_some()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "<EventRecord id={:?} channel={:?} provider={:?} record_id={:?}>",
            self.event_id, self.channel, self.provider, self.record_id
        )
    }

    pub fn __str__(&self) -> String {
        self.xml.clone()
    }
}
