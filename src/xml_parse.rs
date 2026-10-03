use roxmltree::Node;
use serde_json::{Map, Value};

/// Converts a Windows Event XML string into a `serde_json::Value`.
pub fn xml_to_value(xml: &str) -> Result<Value, String> {
    let doc = roxmltree::Document::parse(xml).map_err(|e| e.to_string())?;
    let root = doc.root_element();
    let mut map = Map::new();
    map.insert(root.tag_name().name().to_string(), node_to_value(&root));
    Ok(Value::Object(map))
}

fn node_to_value(node: &Node) -> Value {
    let has_children = node.children().any(|c| c.is_element());
    let text = node.text().map(|s| s.trim()).filter(|s| !s.is_empty());
    let has_attrs = node.attributes().count() > 0;

    // Special handling for EventData with <Data Name="..."> elements
    if node.tag_name().name() == "EventData" && has_children {
        return parse_event_data(node);
    }

    // Leaf node without attributes: return simple value
    if !has_children && !has_attrs {
        if let Some(t) = text {
            return parse_primitive(t);
        } else {
            return Value::Null;
        }
    }

    let mut map = Map::new();

    // Attributes
    for attr in node.attributes() {
        // e.g. Name="foo" -> "Name": "foo"
        map.insert(attr.name().to_string(), Value::String(attr.value().to_string()));
    }

    // Direct text in an element that has attributes
    if let Some(t) = text {
        if !has_children {
            // Leaf element with attributes, e.g. <EventID Qualifiers="16384">100</EventID>
            // or <TimeCreated SystemTime="2026-..." />
            // If the map already has attributes, store text as "#text" or "Value"
            map.insert("#text".to_string(), parse_primitive(t));
            return Value::Object(map);
        } else {
            map.insert("#text".to_string(), Value::String(t.to_string()));
        }
    }

    // Child elements grouped by tag name
    if has_children {
        let mut child_map: std::collections::BTreeMap<String, Vec<Value>> =
            std::collections::BTreeMap::new();

        for child in node.children().filter(|c| c.is_element()) {
            let tag = child.tag_name().name().to_string();
            let val = node_to_value(&child);
            child_map.entry(tag).or_default().push(val);
        }

        for (tag, values) in child_map {
            if values.len() == 1 {
                map.insert(tag, values.into_iter().next().unwrap());
            } else {
                map.insert(tag, Value::Array(values));
            }
        }
    }

    Value::Object(map)
}

/// Specialized parser for <EventData> so that:
/// <Data Name="MajorVersion">10</Data> -> "MajorVersion": 10
/// and <Data>param1</Data><Data>param2</Data> -> "Data": ["param1", "param2"]
fn parse_event_data(node: &Node) -> Value {
    let mut map = Map::new();
    let mut unnamed_data = Vec::new();
    let mut has_named = false;

    for child in node.children().filter(|c| c.is_element()) {
        if child.tag_name().name() == "Data" {
            let text = child.text().unwrap_or("").trim();
            let val = parse_primitive(text);

            if let Some(name) = child.attribute("Name") {
                has_named = true;
                map.insert(name.to_string(), val);
            } else {
                unnamed_data.push(val);
            }
        } else {
            // Any other element inside EventData (e.g. Binary, etc.)
            let tag = child.tag_name().name().to_string();
            let val = node_to_value(&child);
            map.insert(tag, val);
        }
    }

    if !unnamed_data.is_empty() {
        if !has_named && map.is_empty() {
            // If only unnamed data items, return {"Data": [...]}
            let mut res = Map::new();
            res.insert("Data".to_string(), Value::Array(unnamed_data));
            return Value::Object(res);
        } else {
            map.insert("Data".to_string(), Value::Array(unnamed_data));
        }
    }

    Value::Object(map)
}

fn parse_primitive(s: &str) -> Value {
    if let Ok(i) = s.parse::<i64>() {
        return Value::Number(i.into());
    }
    if let Ok(u) = s.parse::<u64>() {
        return Value::Number(u.into());
    }
    if let Ok(f) = s.parse::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return Value::Number(n);
        }
    }
    if s == "true" {
        return Value::Bool(true);
    }
    if s == "false" {
        return Value::Bool(false);
    }
    Value::String(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_event_xml() {
        let xml = r#"<Event xmlns='http://schemas.microsoft.com/win/2004/08/events/event'>
            <System>
                <Provider Name='Microsoft-Windows-Kernel-General' Guid='{a68ca8b7}'/>
                <EventID>11</EventID>
                <Level>4</Level>
                <TimeCreated SystemTime='2026-10-02T08:30:39.0606220Z'/>
                <EventRecordID>13759</EventRecordID>
                <Channel>System</Channel>
                <Computer>LAPTOP-6EBV93J2</Computer>
            </System>
            <EventData>
                <Data Name='ExtraStringLength'>43</Data>
                <Data Name='ExtraString'>\SystemRoot\System32\AppLocker\AppCache.dat</Data>
                <Data Name='Status'>0xc0000022</Data>
            </EventData>
        </Event>"#;

        let val = xml_to_value(xml).expect("parse xml");
        assert!(val.get("Event").is_some());
        let event = &val["Event"];
        assert_eq!(event["System"]["EventID"], 11);
        assert_eq!(event["System"]["Channel"], "System");
        assert_eq!(event["System"]["Computer"], "LAPTOP-6EBV93J2");
        assert_eq!(event["EventData"]["ExtraStringLength"], 43);
        assert_eq!(
            event["EventData"]["ExtraString"],
            r"\SystemRoot\System32\AppLocker\AppCache.dat"
        );
    }
}
