use crate::prelude::Occurs;

#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
pub struct Group {
    pub reference: String,
    pub occurs: Vec<Occurs>,
}

impl Group {
    pub fn read(element: &mut xmltree::Element) -> Self {
        let mut r = Group::default();

        if element.attributes.contains_key("ref") {
            r.reference = element.attributes["ref"].to_string();
        }

        r.occurs = Occurs::read(element);

        r
    }
}
