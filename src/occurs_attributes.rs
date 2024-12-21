#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Occurs {
    MinOccurs(String),
    MaxOccurs(String),
}

impl Occurs {
    pub fn read(element : &mut xmltree::Element) -> Vec<Self> {
        let mut r : Vec<Occurs> = Vec::new();

        if element.attributes.contains_key("minOccurs") {
            r.push(Self::MinOccurs(element.attributes["minOccurs"].to_string()));
        }

        if element.attributes.contains_key("maxOccurs") {
            r.push(Self::MaxOccurs(element.attributes["maxOccurs"].to_string()));
        }

        r
    }
}