use crate::prelude::{Annotation, ComplexType, Occurs};

#[derive(Debug, Default)]
pub struct Element {
    pub name: String,
    pub r#type: String,

    pub annotations: Vec<Annotation>,
    pub occurs: Vec<Occurs>,
    pub complex_types: Vec<ComplexType>,
}

impl Element {
    pub fn read(element: &mut xmltree::Element) -> Self {
        let mut r = Element::default();

        r.occurs = Occurs::read(element);

        while let Some(mut annotation) = element.take_child("annotation") {
            r.annotations.push(Annotation::read(&mut annotation));
        }

        while let Some(mut complex_type) = element.take_child("complexType") {
            r.complex_types.push(ComplexType::read(&mut complex_type));
        }

        if element.attributes.contains_key("name") {
            r.name = element.attributes["name"].to_string();
        }

        if element.attributes.contains_key("type") {
            r.r#type = element.attributes["type"].to_string();
        }

        r
    }
}
