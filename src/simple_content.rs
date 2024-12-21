use xmltree::Element;
use crate::prelude::{Annotation, Attribute, AttributeGroup, Extension};

#[derive(Debug, Default)]
pub struct SimpleContent {
    pub base : String,

    pub annotations : Vec<Annotation>,
    pub attributes : Vec<Attribute>,
    pub attribute_groups : Vec<AttributeGroup>,
    pub extensions : Vec<Extension>,
}

impl SimpleContent {
    pub fn read(element : &mut Element) -> Self {
        let mut r = SimpleContent::default();

        if element.attributes.contains_key("base") {
            r.base = element.attributes["base"].to_string();
        }

        while let Some(mut annotation) = element.take_child("annotation") {
            r.annotations.push(Annotation::read(&mut annotation));
        }

        while let Some(mut attribute) = element.take_child("attribute") {
            r.attributes.push(Attribute::read(&mut attribute));
        }

        while let Some(mut attribute_group) = element.take_child("attributeGroup") {
            r.attribute_groups.push(AttributeGroup::read(&mut attribute_group));
        }

        while let Some(mut extension) = element.take_child("extension") {
            r.extensions.push(Extension::read(&mut extension));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use xmltree::Element;
    use crate::schema::Schema;

    #[test]
    fn simple_content_1() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:complexType name="accidental-text">
		<xs:annotation>
			<xs:documentation>The accidental-text type represents an element with an accidental value and text-formatting attributes.</xs:documentation>
		</xs:annotation>
		<xs:simpleContent>
			<xs:extension base="accidental-value">
				<xs:attributeGroup ref="text-formatting"/>
				<xs:attribute name="smufl" type="smufl-accidental-glyph-name"/>
			</xs:extension>
		</xs:simpleContent>
	</xs:complexType>
</xs:schema>        
"#;
        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.complex_types.len(), 1);

        let type_1 = &item.complex_types[0];
        assert_eq!(type_1.annotations.len(), 1);
        assert_eq!(type_1.simple_content.len(), 1);
        assert_eq!(type_1.simple_content[0].extensions.len(), 1);
        assert_eq!(type_1.simple_content[0].extensions[0].base, "accidental-value".to_string());
        assert_eq!(type_1.simple_content[0].extensions[0].attribute_groups.len(), 1);
        assert_eq!(type_1.simple_content[0].extensions[0].attributes.len(), 1);


    }
}