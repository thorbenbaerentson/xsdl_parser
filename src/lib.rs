mod annotation;
mod attribute;
mod attribute_group;
mod choice;
mod complex_type;
mod element;
mod extension;
mod group;
mod import;
mod occurs_attributes;
mod restriction;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod union;

pub mod prelude {
    pub use crate::attribute::Attribute;
    pub use crate::attribute_group::AttributeGroup;
    pub use crate::choice::Choice;
    pub use crate::complex_type::ComplexType;
    pub use crate::element::Element;
    pub use crate::extension::Extension;
    pub use crate::group::Group;
    pub use crate::import::Import;
    pub use crate::occurs_attributes::Occurs;
    pub use crate::restriction::Restriction;
    pub use crate::restriction::RestrictionContent;
    pub use crate::schema::Schema;
    pub use crate::sequence::Sequence;
    pub use crate::simple_content::SimpleContent;
    pub use crate::simple_type::SimpleType;
    pub use crate::union::Union;

    pub use crate::annotation::Annotation;
    pub use crate::annotation::AnnotationContent;
}
