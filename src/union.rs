use xmltree::Element;

use crate::prelude::SimpleType;

#[derive(Debug, Default, Clone)]
pub struct Union {
    pub types: Vec<String>,

    pub simple_types: Vec<SimpleType>,
}

impl Union {
    pub fn read(element: &mut Element) -> Self {
        let mut r = Union::default();

        if element.attributes.contains_key("memberTypes") {
            let types = element.attributes["memberTypes"].to_string();
            for s in types.split(" ") {
                r.types.push(s.to_string());
            }
        }

        while let Some(mut simple_type) = element.take_child("simpleType") {
            r.simple_types.push(SimpleType::read(&mut simple_type));
        }

        r
    }
}
