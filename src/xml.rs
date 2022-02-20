use libheif_rs::HeifContext;
use std::io::{Cursor, Read, Result};
use xml::reader::{EventReader, XmlEvent};

pub fn decode_xml(metadata: Vec<u8>, filter: &str) -> Result<(String)> {
    let xml = EventReader::from_str(std::str::from_utf8(&metadata).unwrap());
    let mut depth = 0;
    let mut value = String::new();
    for e in xml {
        match e {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace,
            }) => {
                // println!("Name: {:?}", name);
                // println!("Attributes: {:?}", attributes);
                // println!("Namespaces: {:?}", namespace);

                for attribute in attributes {
                    if attribute.name.prefix == Some(filter.to_string()) {
                        println!("{}", attribute.value);
                        value = attribute.value;
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    return Ok(value);
}
