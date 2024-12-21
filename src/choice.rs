use crate::{
    element::Element,
    prelude::{Occurs, Sequence},
};

#[derive(Debug, Default)]
pub enum ChoiceItems {
    #[default]
    None,

    Element(Element),
    Choice(Choice),
    Sequence(Sequence),
}

#[derive(Debug, Default)]
pub struct Choice {
    pub occurs: Vec<Occurs>,
    pub elements: Vec<ChoiceItems>,
}

impl Choice {
    pub fn read(element: &mut xmltree::Element) -> Self {
        let mut r = Choice::default();

        r.occurs = Occurs::read(element);

        loop {
            if let Some(mut element) = element.take_child("element") {
                r.elements
                    .push(ChoiceItems::Element(Element::read(&mut element)));
                continue;
            }

            if let Some(mut choice) = element.take_child("choice") {
                r.elements
                    .push(ChoiceItems::Choice(Choice::read(&mut choice)));
                continue;
            }

            if let Some(mut sequence) = element.take_child("sequence") {
                r.elements
                    .push(ChoiceItems::Sequence(Sequence::read(&mut sequence)));
                continue;
            }

            break;
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::Occurs, schema::Schema};

    #[test]
    fn choice_1() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:complexType name="dynamics">
		<xs:annotation>
			<xs:documentation>Dynamics can be associated either with a note or a general musical direction.</xs:documentation>
        </xs:annotation>
		<xs:choice minOccurs="0" maxOccurs="unbounded">
			<xs:element name="p" type="empty"/>
			<xs:element name="pp" type="empty"/>
			<xs:element name="ppp" type="empty"/>
			<xs:element name="pppp" type="empty"/>
			<xs:element name="ppppp" type="empty"/>
			<xs:element name="pppppp" type="empty"/>
			<xs:element name="f" type="empty"/>
			<xs:element name="ff" type="empty"/>
			<xs:element name="fff" type="empty"/>
			<xs:element name="ffff" type="empty"/>
			<xs:element name="fffff" type="empty"/>
			<xs:element name="ffffff" type="empty"/>
			<xs:element name="mp" type="empty"/>
			<xs:element name="mf" type="empty"/>
			<xs:element name="sf" type="empty"/>
			<xs:element name="sfp" type="empty"/>
			<xs:element name="sfpp" type="empty"/>
			<xs:element name="fp" type="empty"/>
			<xs:element name="rf" type="empty"/>
			<xs:element name="rfz" type="empty"/>
			<xs:element name="sfz" type="empty"/>
			<xs:element name="sffz" type="empty"/>
			<xs:element name="fz" type="empty"/>
			<xs:element name="n" type="empty"/>
			<xs:element name="pf" type="empty"/>
			<xs:element name="sfzp" type="empty"/>
			<xs:element name="other-dynamics" type="other-text"/>
		</xs:choice>
		<xs:attributeGroup ref="print-style-align"/>
		<xs:attributeGroup ref="placement"/>
		<xs:attributeGroup ref="text-decoration"/>
		<xs:attributeGroup ref="enclosure"/>
		<xs:attributeGroup ref="optional-unique-id"/>
	</xs:complexType>
</xs:schema>        
        "#;
        let mut element = xmltree::Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.complex_types.len(), 1);
        assert_eq!(item.complex_types[0].name, "dynamics".to_string());

        let type_1 = &item.complex_types[0];
        assert_eq!(type_1.annotations.len(), 1);
        assert_eq!(type_1.attribute_groups.len(), 5);

        assert_eq!(type_1.choices.len(), 1);
        assert_eq!(
            type_1.choices[0].occurs[0],
            Occurs::MinOccurs("0".to_string())
        );
        assert_eq!(
            type_1.choices[0].occurs[1],
            Occurs::MaxOccurs("unbounded".to_string())
        );

        assert_eq!(type_1.choices[0].elements.len(), 27);
        match &type_1.choices[0].elements[4] {
            crate::choice::ChoiceItems::Element(element) => {
                assert_eq!(element.name, "ppppp".to_string());
                assert_eq!(element.r#type, "empty".to_string());
            }
            _ => {
                panic!("Expected choice");
            }
        }
    }
}
