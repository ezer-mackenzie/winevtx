pub mod error;
pub mod live;
pub mod offline;
pub mod record;
pub mod xml_parse;

use pyo3::prelude::*;
use live::{LiveEventIterator, LiveEventLog};
use offline::{EvtxFile, EvtxRecordIterator};
use record::EventRecord;

/// Query live events from a Windows Event Log channel (e.g. "System", "Application", "Security").
///
/// Args:
///     channel (str): Name of the event log channel (default: "System").
///     query (str): XPath query filter (default: "*").
///     limit (int, optional): Maximum number of events to return.
///     reverse (bool): If True, returns newest events first (default: True).
///     format (str): Output format: 'record' (default), 'dict', 'xml', or 'json'.
///
/// Returns:
///     list: List of events formatted as requested.
#[pyfunction]
#[pyo3(signature = (channel="System", query="*", limit=None, reverse=true, format="record"))]
fn query_events<'py>(
    py: Python<'py>,
    channel: &str,
    query: &str,
    limit: Option<usize>,
    reverse: bool,
    format: &str,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let log = LiveEventLog::new(channel, query, reverse);
    log.read(py, limit, format)
}

/// Create an iterator over live Windows Event Log events.
///
/// Args:
///     channel (str): Name of the event log channel (default: "System").
///     query (str): XPath query filter (default: "*").
///     reverse (bool): If True, reads newest events first (default: True).
///     format (str): Output format: 'record' (default), 'dict', 'xml', or 'json'.
///
/// Returns:
///     LiveEventIterator: An iterator yielding events.
#[pyfunction]
#[pyo3(signature = (channel="System", query="*", reverse=true, format="record"))]
fn iter_events(
    channel: &str,
    query: &str,
    reverse: bool,
    format: &str,
) -> PyResult<LiveEventIterator> {
    let log = LiveEventLog::new(channel, query, reverse);
    log.iter(format)
}

/// Read events from an offline .evtx file.
///
/// Args:
///     path (str): Path to the .evtx file.
///     limit (int, optional): Maximum number of events to return.
///     format (str): Output format: 'record' (default), 'dict', 'xml', or 'json'.
///
/// Returns:
///     list: List of parsed events.
#[pyfunction]
#[pyo3(signature = (path, limit=None, format="record"))]
fn read_evtx<'py>(
    py: Python<'py>,
    path: &str,
    limit: Option<usize>,
    format: &str,
) -> PyResult<Vec<Bound<'py, PyAny>>> {
    let evtx_file = EvtxFile::new(path)?;
    evtx_file.read(py, limit, format)
}

/// Create an iterator over an offline .evtx file.
///
/// Args:
///     path (str): Path to the .evtx file.
///     format (str): Output format: 'record' (default), 'dict', 'xml', or 'json'.
///
/// Returns:
///     EvtxRecordIterator: An iterator yielding parsed events.
#[pyfunction]
#[pyo3(signature = (path, format="record"))]
fn iter_evtx(path: &str, format: &str) -> PyResult<EvtxRecordIterator> {
    let evtx_file = EvtxFile::new(path)?;
    evtx_file.iter(format)
}

/// Parse a raw Windows Event XML string into a native Python dictionary.
#[pyfunction]
fn xml_to_dict<'py>(py: Python<'py>, xml: &str) -> PyResult<Bound<'py, PyAny>> {
    let val = xml_parse::xml_to_value(xml)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("XML parse error: {}", e)))?;
    pythonize::pythonize(py, &val).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Dictionary conversion error: {}", e))
    })
}

/// Parse a raw Windows Event XML string into a JSON string.
#[pyfunction]
fn xml_to_json(xml: &str) -> PyResult<String> {
    let val = xml_parse::xml_to_value(xml)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("XML parse error: {}", e)))?;
    serde_json::to_string(&val).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("JSON serialization error: {}", e))
    })
}

/// Fast and safe Windows Event Log and EVTX reader for Python.
#[pymodule]
fn _winevtx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<EventRecord>()?;
    m.add_class::<LiveEventLog>()?;
    m.add_class::<LiveEventIterator>()?;
    m.add_class::<EvtxFile>()?;
    m.add_class::<EvtxRecordIterator>()?;

    m.add_function(wrap_pyfunction!(query_events, m)?)?;
    m.add_function(wrap_pyfunction!(iter_events, m)?)?;
    m.add_function(wrap_pyfunction!(read_evtx, m)?)?;
    m.add_function(wrap_pyfunction!(iter_evtx, m)?)?;
    m.add_function(wrap_pyfunction!(xml_to_dict, m)?)?;
    m.add_function(wrap_pyfunction!(xml_to_json, m)?)?;

    Ok(())
}
