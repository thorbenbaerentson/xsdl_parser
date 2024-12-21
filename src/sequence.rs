use crate::{element::Element, group::Group};

#[derive(Debug, Default)]
pub struct Sequence {
    pub elements: Vec<Element>,
    pub groups: Vec<Group>,
}

impl Sequence {
    pub fn read(element: &mut xmltree::Element) -> Self {
        let mut r = Sequence::default();

        while let Some(mut element) = element.take_child("element") {
            r.elements.push(Element::read(&mut element));
        }

        while let Some(mut group) = element.take_child("group") {
            r.groups.push(Group::read(&mut group));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;

    #[test]
    fn sequence_1() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:complexType name="midi-instrument">
		<xs:annotation>
			<xs:documentation>The midi-instrument type defines MIDI 1.0 instrument playback.</xs:documentation>
		</xs:annotation>
		<xs:sequence>
            <xs:group ref="editorial"/>

			<xs:element name="midi-channel" type="midi-16" minOccurs="0">
				<xs:annotation>
					<xs:documentation>The midi-channel element specifies a MIDI 1.0 channel numbers ranging from 1 to 16.</xs:documentation>
				</xs:annotation>
			</xs:element>
			<xs:element name="midi-name" type="xs:string" minOccurs="0">
				<xs:annotation>
					<xs:documentation>The midi-name element corresponds to a ProgramName meta-event within a Standard MIDI File.</xs:documentation>
				</xs:annotation>
			</xs:element>
			<xs:element name="midi-bank" type="midi-16384" minOccurs="0">
				<xs:annotation>
					<xs:documentation>The midi-bank element specifies a MIDI 1.0 bank number ranging from 1 to 16,384.</xs:documentation>
				</xs:annotation>
			</xs:element>

            <xs:element name="directive" minOccurs="0" maxOccurs="unbounded">
				<xs:annotation>
					<xs:documentation>Directives are like directions, but can be grouped together with attributes for convenience. This is typically used for tempo markings at the beginning of a piece of music. This element was deprecated in Version 2.0 in favor of the direction element's directive attribute. Language names come from ISO 639, with optional country subcodes from ISO 3166.</xs:documentation>
				</xs:annotation>
				<xs:complexType>
					<xs:simpleContent>
						<xs:extension base="xs:string">
							<xs:attributeGroup ref="print-style"/>
							<xs:attribute ref="xml:lang"/>
						</xs:extension>
					</xs:simpleContent>
				</xs:complexType>
			</xs:element>

		</xs:sequence>
		<xs:attribute name="id" type="xs:IDREF" use="required"/>
	</xs:complexType>
</xs:schema>        
        "#;
        let mut element = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.complex_types.len(), 1);
        assert_eq!(item.complex_types[0].name, "midi-instrument".to_string());

        let type_1 = &item.complex_types[0];
        assert_eq!(type_1.annotations.len(), 1);

        assert_eq!(item.complex_types[0].sequences.len(), 1);

        let seq = &item.complex_types[0].sequences[0];
        assert_eq!(seq.elements.len(), 4);
        assert_eq!(seq.groups[0].reference, "editorial".to_string());

        assert_eq!(seq.elements[0].annotations.len(), 1);
        assert_eq!(seq.elements[0].occurs.len(), 1);
        assert_eq!(seq.elements[0].name, "midi-channel".to_string());
        assert_eq!(seq.elements[0].r#type, "midi-16".to_string());

        assert_eq!(seq.elements[1].annotations.len(), 1);
        assert_eq!(seq.elements[1].occurs.len(), 1);
        assert_eq!(seq.elements[1].name, "midi-name".to_string());
        assert_eq!(seq.elements[1].r#type, "xs:string".to_string());

        assert_eq!(seq.elements[2].annotations.len(), 1);
        assert_eq!(seq.elements[2].occurs.len(), 1);
        assert_eq!(seq.elements[2].name, "midi-bank".to_string());
        assert_eq!(seq.elements[2].r#type, "midi-16384".to_string());

        assert_eq!(seq.elements[3].annotations.len(), 1);
        assert_eq!(seq.elements[3].occurs.len(), 2);
        assert_eq!(seq.elements[3].name, "directive".to_string());
        assert_eq!(seq.elements[3].complex_types.len(), 1);
    }
}
