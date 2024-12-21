use xmltree::Element;
use crate::prelude::{Attribute, AttributeGroup};

#[derive(Debug, Default)]
pub struct Extension {
    pub base : String,

    pub attributes : Vec<Attribute>,
    pub attribute_groups : Vec<AttributeGroup>,
}

impl Extension {
    pub fn read(element : &mut Element) -> Self {
        let mut r = Extension::default();

        if element.attributes.contains_key("base") {
            r.base = element.attributes["base"].to_string();
        }

        while let Some(mut attribute) = element.take_child("attribute") {
            r.attributes.push(Attribute::read(&mut attribute));
        }

        while let Some(mut attribute_group) = element.take_child("attributeGroup") {
            r.attribute_groups.push(AttributeGroup::read(&mut attribute_group));
        }

        r
    }
}