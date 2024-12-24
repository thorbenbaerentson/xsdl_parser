use xmltree::Element;

use crate::prelude::{Annotation, Attribute};

#[derive(Debug, Clone, Default)]
pub struct AttributeGroup {
    pub name: String,
    pub reference: Option<String>,
    pub annotations: Vec<Annotation>,
    pub attributes: Vec<Attribute>,
    pub attribute_groups: Vec<AttributeGroup>,
}

impl AttributeGroup {
    pub fn read(element: &mut Element) -> Self {
        let mut r = AttributeGroup::default();

        if element.attributes.contains_key("name") {
            r.name = element.attributes["name"].clone();
        }

        if element.attributes.contains_key("ref") {
            r.reference = Some(element.attributes["ref"].clone());
        }

        // Read annotations values.
        while let Some(mut annotation) = element.take_child("annotation") {
            r.annotations.push(Annotation::read(&mut annotation));
        }

        while let Some(mut attribute) = element.take_child("attribute") {
            r.attributes.push(Attribute::read(&mut attribute));
        }

        while let Some(mut grp) = element.take_child("attributeGroup") {
            r.attribute_groups.push(AttributeGroup::read(&mut grp));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;
    use xmltree::Element;

    #[test]
    fn annotation() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:attributeGroup name="dashed-formatting">
		<xs:annotation>
			<xs:documentation>The dashed-formatting entity represents the length of dashes and spaces in a dashed line. Both the dash-length and space-length attributes are represented in tenths. These attributes are ignored if the corresponding line-type attribute is not dashed.</xs:documentation>
		</xs:annotation>
		<xs:attribute name="dash-length" type="tenths"/>
		<xs:attribute name="space-length" type="tenths"/>
	</xs:attributeGroup>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.attribute_groups.len(), 1);
        assert_eq!(item.attribute_groups[0].annotations.len(), 1);
        assert_eq!(item.attribute_groups[0].attributes.len(), 2);
    }
    #[test]
    fn attribute_group() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:attributeGroup name="text-formatting">
		<xs:annotation>
			<xs:documentation>The text-formatting attribute group collects the common formatting attributes for text elements. Default values may differ across the elements that use this group.</xs:documentation>
		</xs:annotation>
		<xs:attributeGroup ref="justify"/>
		<xs:attributeGroup ref="print-style-align"/>
		<xs:attributeGroup ref="text-decoration"/>
		<xs:attributeGroup ref="text-rotation"/>
		<xs:attributeGroup ref="letter-spacing"/>
		<xs:attributeGroup ref="line-height"/>
		<xs:attribute ref="xml:lang"/>
		<xs:attribute ref="xml:space"/>
		<xs:attributeGroup ref="text-direction"/>
		<xs:attributeGroup ref="enclosure"/>
        <xs:attribute name="underline" type="number-of-lines"/>
		<xs:attribute name="overline" type="number-of-lines"/>
		<xs:attribute name="line-through" type="number-of-lines"/>
	</xs:attributeGroup>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.attribute_groups.len(), 1);
        assert_eq!(item.attribute_groups[0].annotations.len(), 1);
        assert_eq!(item.attribute_groups[0].attributes.len(), 5);
        assert_eq!(item.attribute_groups[0].attribute_groups.len(), 8);
    }
}
