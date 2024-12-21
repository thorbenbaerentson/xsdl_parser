use xmltree::Element;


#[derive(Debug, Default)]
pub struct Import {
    pub namespace : String,
    pub schema_location : String,
}

impl Import {
    pub fn read(element : &mut Element) -> Self {
        let mut r = Import::default();
        if element.attributes.contains_key("namespace") {
            r.namespace = element.attributes["namespace"].clone();
        }

        if element.attributes.contains_key("schemaLocation") {
            r.schema_location = element.attributes["schemaLocation"].clone();
        }

        r
    }
}