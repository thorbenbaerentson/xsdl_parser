use xmltree::Element;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum AttributeMeta {
    Reference(String),
    Use(String),
    Fixed(String),
    Default(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Attribute {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub additional_attributes: Vec<AttributeMeta>,
}

impl Attribute {
    pub fn read(element: &mut Element) -> Self {
        let mut r = Attribute::default();
        if element.attributes.contains_key("name") {
            r.name = Some(element.attributes["name"].clone());
        }

        if element.attributes.contains_key("type") {
            r.r#type = Some(element.attributes["type"].clone());
        }

        if element.attributes.contains_key("ref") {
            r.additional_attributes
                .push(AttributeMeta::Reference(element.attributes["ref"].clone()));
        }

        if element.attributes.contains_key("use") {
            r.additional_attributes
                .push(AttributeMeta::Use(element.attributes["use"].clone()));
        }

        if element.attributes.contains_key("fixed") {
            r.additional_attributes
                .push(AttributeMeta::Fixed(element.attributes["fixed"].clone()));
        }

        if element.attributes.contains_key("default") {
            r.additional_attributes.push(AttributeMeta::Default(
                element.attributes["default"].clone(),
            ));
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use crate::{attribute::AttributeMeta, schema::Schema};
    use xmltree::Element;

    #[test]
    fn annotation() {
        let xml = r#"
<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xlink="http://www.w3.org/1999/xlink" elementFormDefault="qualified" attributeFormDefault="unqualified">
    <xs:attributeGroup name="link-attributes">
		<xs:annotation>
			<xs:documentation>The link-attributes group includes all the simple XLink attributes supported in the MusicXML format. It is also used to connect a MusicXML score with MusicXML parts or a MusicXML opus.</xs:documentation>
		</xs:annotation>
		<!--<xs:attribute ref="xmnls:xlink" fixed="http://www.w3.org/1999/xlink"/>-->
		<xs:attribute ref="xlink:href" use="required"/>
		<xs:attribute ref="xlink:type" fixed="simple"/>
		<xs:attribute ref="xlink:role"/>
		<xs:attribute ref="xlink:title"/>
		<xs:attribute ref="xlink:show" default="replace"/>
		<xs:attribute ref="xlink:actuate" default="onRequest"/>
	</xs:attributeGroup>
</xs:schema>
"#;

        let mut element = Element::parse(xml.as_bytes()).unwrap();
        let item = Schema::read(&mut element);

        assert_eq!(item.attribute_groups.len(), 1);
        assert_eq!(item.attribute_groups[0].annotations.len(), 1);
        assert_eq!(item.attribute_groups[0].attributes.len(), 6);

        let att_1 = item.attribute_groups[0].attributes[0].clone();
        assert_eq!(att_1.additional_attributes.len(), 2);
        assert_eq!(
            att_1.additional_attributes[0],
            AttributeMeta::Reference("xlink:href".to_string())
        );
        assert_eq!(
            att_1.additional_attributes[1],
            AttributeMeta::Use("required".to_string())
        );

        let att_2 = item.attribute_groups[0].attributes[1].clone();
        assert_eq!(att_2.additional_attributes.len(), 2);
        assert_eq!(
            att_2.additional_attributes[0],
            AttributeMeta::Reference("xlink:type".to_string())
        );
        assert_eq!(
            att_2.additional_attributes[1],
            AttributeMeta::Fixed("simple".to_string())
        );

        let att_3 = item.attribute_groups[0].attributes[2].clone();
        assert_eq!(att_3.additional_attributes.len(), 1);
        assert_eq!(
            att_3.additional_attributes[0],
            AttributeMeta::Reference("xlink:role".to_string())
        );

        let att_4 = item.attribute_groups[0].attributes[3].clone();
        assert_eq!(att_4.additional_attributes.len(), 1);
        assert_eq!(
            att_4.additional_attributes[0],
            AttributeMeta::Reference("xlink:title".to_string())
        );

        let att_5 = item.attribute_groups[0].attributes[4].clone();
        assert_eq!(att_5.additional_attributes.len(), 2);
        assert_eq!(
            att_5.additional_attributes[0],
            AttributeMeta::Reference("xlink:show".to_string())
        );
        assert_eq!(
            att_5.additional_attributes[1],
            AttributeMeta::Default("replace".to_string())
        );

        let att_6 = item.attribute_groups[0].attributes[5].clone();
        assert_eq!(att_6.additional_attributes.len(), 2);
        assert_eq!(
            att_6.additional_attributes[0],
            AttributeMeta::Reference("xlink:actuate".to_string())
        );
        assert_eq!(
            att_6.additional_attributes[1],
            AttributeMeta::Default("onRequest".to_string())
        );
    }
}
