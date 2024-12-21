use xmltree::Element;
use crate::{choice::Choice, group::Group, prelude::{Annotation, Attribute, AttributeGroup, Sequence, SimpleContent}};

#[derive(Debug, Default)]
pub struct ComplexType {
    pub name : String,

    pub annotations : Vec<Annotation>,
    pub attributes : Vec<Attribute>,
    pub attribute_groups : Vec<AttributeGroup>,
    pub simple_content : Vec<SimpleContent>,
    pub choices : Vec<Choice>,
    pub sequences : Vec<Sequence>,

    pub groups : Vec<Group>,
}

impl ComplexType {
    pub fn read(element : &mut Element) -> Self {
        let mut r = ComplexType::default();

        if element.attributes.contains_key("name") {
            r.name = element.attributes["name"].to_string();
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

        while let Some(mut simple_content) = element.take_child("simpleContent") {
            r.simple_content.push(SimpleContent::read(&mut simple_content));
        }

        while let Some(mut choice) = element.take_child("choice") {
            r.choices.push(Choice::read(&mut choice));
        }

        while let Some(mut sequence) = element.take_child("sequence") {
            r.sequences.push(Sequence::read(&mut sequence));
        }

        while let Some(mut group) = element.take_child("group") {
            r.groups.push(Group::read(&mut group));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use xmltree::Element;
    use crate::{prelude::Occurs, schema::Schema};

    #[test]
    fn complext_type_1() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:complexType name="accidental-text">
        <xs:group ref="slash" minOccurs="0"/>

		<xs:annotation>
			<xs:documentation>The accidental-text type represents an element with an accidental value and text-formatting attributes.</xs:documentation>
		</xs:annotation>
		<xs:simpleContent>
			<xs:extension base="accidental-value">
				<xs:attributeGroup ref="text-formatting"/>
				<xs:attribute name="smufl" type="smufl-accidental-glyph-name"/>
			</xs:extension>
		</xs:simpleContent>
        <xs:group ref="staff" minOccurs="0"/>
	</xs:complexType>
</xs:schema>        
        "#;
        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.complex_types.len(), 1);
        assert_eq!(item.complex_types[0].name, "accidental-text".to_string());

        let type_1 = &item.complex_types[0];
        assert_eq!(type_1.annotations.len(), 1);
        assert_eq!(type_1.simple_content.len(), 1);

        let grp_1 = type_1.groups[0].clone();
        assert_eq!(grp_1.reference, "slash".to_string());
        assert_eq!(grp_1.occurs[0], Occurs::MinOccurs("0".to_string()));

        let grp_2 = type_1.groups[1].clone();
        assert_eq!(grp_2.reference, "staff".to_string());
        assert_eq!(grp_2.occurs[0], Occurs::MinOccurs("0".to_string()));

    }
}