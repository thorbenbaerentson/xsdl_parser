use xmltree::Element;


#[derive(Debug, Clone, Default)]
pub enum AnnotationContent {
    #[default]
    None, 
    Documentation(String),
}

#[derive(Debug, Clone, Default)]
pub struct Annotation {
    pub content : Vec<AnnotationContent>,
}

impl Annotation {
    pub fn new() -> Self {
        Annotation {
            content : Vec::new(),
        }
    }

    pub fn read(element : &mut Element) -> Self {
        let mut r  = Annotation::new();

        while let Some(s) = element.take_child("documentation") {
            match s.get_text() {
                Some(doc) => r.content.push(AnnotationContent::Documentation(doc.to_string())),
                None => { },
            };
        }

        r
    }
}