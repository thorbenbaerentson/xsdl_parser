use xmltree::Element;

use crate::{prelude::{Annotation, Restriction}, union::Union};


#[derive(Debug, Default, Clone)]
pub struct SimpleType {
    /// The name of the type as found inside the xsd definition.
    pub name : String,

    /// Annotations, usually documentation
    pub annotations : Vec<Annotation>,
    pub restriction : Option<Restriction>,

    pub union : Option<Union>,
}

impl SimpleType {
    pub fn read(element : &mut Element) -> Self {
        let mut r = SimpleType::default();

        if element.attributes.contains_key("name") {
            r.name = element.attributes["name"].to_string();
        }

        while let Some(mut annotation) = element.take_child("annotation") {
            r.annotations.push(Annotation::read(&mut annotation));
        }

        while let Some(mut annotation) = element.take_child("restriction") {
            r.restriction = Some(Restriction::read(&mut annotation));
        }

        while let Some(mut union) = element.take_child("union") {
            r.union = Some(Union::read(&mut union));
        }

        r
    }
}


#[cfg(test)]
mod tests {
    use xmltree::Element;
    use crate::{restriction::RestrictionContent, schema::Schema};
    

    #[test]
    fn simple_type_1() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:simpleType name="above-below">
		<xs:annotation>
			<xs:documentation>The above-below type is used to indicate whether one element appears above or below another element.</xs:documentation>
		</xs:annotation>
		<xs:restriction base="xs:token">
			<xs:enumeration value="above"/>
			<xs:enumeration value="below"/>
		</xs:restriction>
	</xs:simpleType>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.simple_types.len(), 1);
        assert_eq!(item.simple_types[0].annotations.len(), 1);
        assert_eq!(item.simple_types[0].name, "above-below".to_string());

        let restriction = item.simple_types[0].restriction.clone().unwrap();
        assert_eq!(restriction.base, "xs:token".to_string());
        assert_eq!(restriction.content[0], RestrictionContent::Enumeration("above".to_string()));
        assert_eq!(restriction.content[1], RestrictionContent::Enumeration("below".to_string()));
    }

    #[test]
    fn simple_type_2() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:simpleType name="midi-16384">
		<xs:annotation>
			<xs:documentation>The midi-16384 type is used to express MIDI 1.0 values that range from 1 to 16,384.</xs:documentation>
		</xs:annotation>
		<xs:restriction base="xs:positiveInteger">
			<xs:minInclusive value="1"/>
			<xs:maxInclusive value="16384"/>
		</xs:restriction>
	</xs:simpleType>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.simple_types.len(), 1);
        assert_eq!(item.simple_types[0].annotations.len(), 1);
        assert_eq!(item.simple_types[0].name, "midi-16384".to_string());

        let restriction = item.simple_types[0].restriction.clone().unwrap();
        assert_eq!(restriction.base, "xs:positiveInteger".to_string());
        assert_eq!(restriction.content[0], RestrictionContent::MinInclusive(1));
        assert_eq!(restriction.content[1], RestrictionContent::MaxInclusive(16384));
    }

    #[test]
    fn simple_type_3() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:simpleType name="font-family">
		<xs:annotation>
			<xs:documentation>The font-family is a comma-separated list of font names. These can be specific font styles such as Maestro or Opus, or one of several generic font styles: music, engraved, handwritten, text, serif, sans-serif, handwritten, cursive, fantasy, and monospace. The music, engraved, and handwritten values refer to music fonts; the rest refer to text fonts. The fantasy style refers to decorative text such as found in older German-style printing.</xs:documentation>
		</xs:annotation>
		<xs:restriction base="comma-separated-text"/>
	</xs:simpleType>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.simple_types.len(), 1);
        assert_eq!(item.simple_types[0].annotations.len(), 1);
        assert_eq!(item.simple_types[0].name, "font-family".to_string());

        let restriction = item.simple_types[0].restriction.clone().unwrap();
        assert_eq!(restriction.base, "comma-separated-text".to_string());
        assert_eq!(restriction.content.len(), 0);
    }

    #[test]
    fn simple_type_5() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:simpleType name="font-size">
		<xs:annotation>
			<xs:documentation>The font-size can be one of the CSS font sizes (xx-small, x-small, small, medium, large, x-large, xx-large) or a numeric point size.</xs:documentation>
		</xs:annotation>
		<xs:union memberTypes="xs:decimal css-font-size"/>
	</xs:simpleType>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.simple_types.len(), 1);
        assert_eq!(item.simple_types[0].annotations.len(), 1);
        assert_eq!(item.simple_types[0].name, "font-size".to_string());

        assert!(item.simple_types[0].restriction.clone().is_none());
        assert!(item.simple_types[0].union.is_some());

        let union = item.simple_types[0].union.clone().unwrap();
        assert_eq!(union.types.len(), 2);
        assert_eq!(union.types[0], "xs:decimal".to_string());
        assert_eq!(union.types[1], "css-font-size".to_string());
    }

    #[test]
    fn simple_type_4() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:simpleType name="number-or-normal">
		<xs:annotation>
			<xs:documentation>The number-or-normal values can be either a decimal number or the string "normal". This is used by the line-height and letter-spacing attributes.</xs:documentation>
		</xs:annotation>
		<xs:union memberTypes="xs:decimal">
			<xs:simpleType>
				<xs:restriction base="xs:token">
					<xs:enumeration value="normal"/>
				</xs:restriction>
			</xs:simpleType>
		</xs:union>
	</xs:simpleType>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.simple_types.len(), 1);
        assert_eq!(item.simple_types[0].annotations.len(), 1);
        assert_eq!(item.simple_types[0].name, "number-or-normal".to_string());

        assert!(item.simple_types[0].restriction.clone().is_none());
        assert!(item.simple_types[0].union.is_some());

        let union = item.simple_types[0].union.clone().unwrap();
        assert_eq!(union.types.len(), 1);
        assert_eq!(union.types[0], "xs:decimal".to_string());
        assert_eq!(union.simple_types[0].restriction.clone().unwrap().content.len(), 1);
    }
}