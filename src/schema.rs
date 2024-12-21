use std::path::PathBuf;

use xmltree::Element;
use crate::{import::Import, prelude::{Annotation, AttributeGroup, ComplexType}, simple_type::SimpleType};


#[derive(Debug, Default)]
pub struct Schema {
    pub annotations : Vec<Annotation>,
    pub imports : Vec<Import>,
    pub simple_types : Vec<SimpleType>,
    pub attribute_groups : Vec<AttributeGroup>,
    pub complex_types : Vec<ComplexType>,
}

impl Schema {
    /// Download and parse a schema definition from the internet. 
    pub fn download(_url : &str) -> Result<Self, String> {
        Err("Not implemented yet. Now would be the time to change it.".to_string())
    }

    /// Load and parse a schema definition from disk.
    pub fn load(path : &PathBuf) -> Result<Self, String> {
        match std::fs::read_to_string(path) {
            Ok(xml) => {
                Self::parse(&xml)
            },
            
            Err(e) => {
                let err = format!("Could open xml file. Error: {:?}", e);
                Err(err)
            },
        }
    }

    /// Parse the given string into a schema.
    pub fn parse(xml : &str) -> Result<Self, String> {
        match xmltree::Element::parse(xml.as_bytes()) {
            Ok(mut element) => {
                let item = Schema::read(&mut element);
                Ok(item)
            },

            Err(e) => {
                let err = format!("Could not parse xml file. Error: {:?}", e);
                Err(err)
            },
        }
    }

    /// Read an element after it has been parsed. 
    pub fn read(element : &mut Element) -> Self {
        let mut r = Schema::default();

        while let Some(mut annotation) = element.take_child("annotation") {
            r.annotations.push(Annotation::read(&mut annotation));
        }

        while let Some(mut import) = element.take_child("import") {
            r.imports.push(Import::read(&mut import));
        }

        while let Some(mut simple_type) = element.take_child("simpleType") {
            r.simple_types.push(SimpleType::read(&mut simple_type));
        }

        while let Some(mut simple_type) = element.take_child("attributeGroup") {
            r.attribute_groups.push(AttributeGroup::read(&mut simple_type));
        }

        while let Some(mut complex_type) = element.take_child("complexType") {
            r.complex_types.push(ComplexType::read(&mut complex_type));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use xmltree::Element;
    use crate::prelude::AnnotationContent;
    use super::*;

    #[test]
    fn annotation() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:annotation>
        <xs:documentation>The MusicXML 4.1 DTD has no namespace, so for compatibility
            the MusicXML 4.1 XSD has no namespace either. Those who need to import the
            MusicXML XSD into another schema are advised to create a new version
            that uses "http://www.musicxml.org/xsd/MusicXML" as the namespace.
        </xs:documentation>
    </xs:annotation>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.annotations.len(), 1);
        assert_eq!(item.annotations[0].content.len(), 1);
        match &item.annotations[0].content[0] {
            AnnotationContent::Documentation(s) => { assert!(s.starts_with("The MusicXML 4.1 DTD has no namespace")); },
            _ => { panic!("Wrong annotation type!"); }
        }
    }

    #[test]
    fn import() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:import namespace="http://www.w3.org/XML/1998/namespace" schemaLocation="http://www.musicxml.org/xsd/xml.xsd"/>
	<xs:import namespace="http://www.w3.org/1999/xlink" schemaLocation="http://www.musicxml.org/xsd/xlink.xsd"/>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.imports.len(), 2);
        assert_eq!(item.imports[0].namespace, "http://www.w3.org/XML/1998/namespace".to_string());
        assert_eq!(item.imports[1].namespace, "http://www.w3.org/1999/xlink".to_string());

        assert_eq!(item.imports[0].schema_location, "http://www.musicxml.org/xsd/xml.xsd".to_string());
        assert_eq!(item.imports[1].schema_location, "http://www.musicxml.org/xsd/xlink.xsd".to_string());
    }
}