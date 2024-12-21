mod schema;
mod annotation;
mod import;
mod simple_type;
mod restriction;
mod union;
mod attribute_group;
mod attribute;
mod complex_type;
mod simple_content;
mod extension;
mod choice;
mod element;
mod sequence;
mod occurs_attributes;
mod group;

pub mod prelude {
    pub use crate::schema::Schema;
    pub use crate::import::Import;
    pub use crate::simple_type::SimpleType;
    pub use crate::restriction::Restriction;
    pub use crate::union::Union;
    pub use crate::attribute::Attribute;
    pub use crate::attribute_group::AttributeGroup;
    pub use crate::complex_type::ComplexType;
    pub use crate::simple_content::SimpleContent;
    pub use crate::extension::Extension;
    pub use crate::choice::Choice;
    pub use crate::element::Element;
    pub use crate::sequence::Sequence;
    pub use crate::occurs_attributes::Occurs;
    pub use crate::group::Group;

    pub use crate::annotation::Annotation;
    pub use crate::annotation::AnnotationContent;
}