use xmltree::Element;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum RestrictionContent {
    /// A single enumeration option. The only item is the attribute value.
    Enumeration(String),

    /// Min value simple integer.
    MinInclusive(i32),

    /// Max value simple integer.
    MaxInclusive(i32),

    /// A regex pattern
    Pattern(String),
}

#[derive(Debug, Default, Clone)]
pub struct Restriction {
    pub base: String,

    pub content: Vec<RestrictionContent>,
}

impl Restriction {
    pub fn read(element: &mut Element) -> Self {
        let mut r = Restriction::default();

        if element.attributes.contains_key("base") {
            r.base = element.attributes["base"].clone();
        }

        // Read enum values.
        while let Some(annotation) = element.take_child("enumeration") {
            if annotation.attributes.contains_key("value") {
                let option = annotation.attributes["value"].clone();
                r.content.push(RestrictionContent::Enumeration(option));
            }
        }

        // Min values
        while let Some(min) = element.take_child("minInclusive") {
            if min.attributes.contains_key("value") {
                let option = min.attributes["value"].clone();
                match option.parse::<i32>() {
                    Ok(v) => {
                        r.content.push(RestrictionContent::MinInclusive(v));
                    }
                    Err(_) => {}
                };
            }
        }

        // Max values
        while let Some(max) = element.take_child("maxInclusive") {
            if max.attributes.contains_key("value") {
                let option = max.attributes["value"].clone();
                match option.parse::<i32>() {
                    Ok(v) => {
                        r.content.push(RestrictionContent::MaxInclusive(v));
                    }
                    Err(_) => {}
                };
            }
        }

        // Pattern
        while let Some(min) = element.take_child("pattern") {
            if min.attributes.contains_key("value") {
                let option = min.attributes["value"].clone();
                r.content.push(RestrictionContent::Pattern(option));
            }
        }

        r
    }
}
