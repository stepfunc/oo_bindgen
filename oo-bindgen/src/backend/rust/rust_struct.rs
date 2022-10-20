use crate::backend::rust::rust_type::RustType;
use crate::model::*;

pub(crate) trait RustStruct {
    fn annotate_rust_with_lifetime(&self) -> bool;
    fn annotate_c_with_lifetime(&self) -> bool;
    fn has_conversion(&self) -> bool;
}

impl<T> RustStruct for Handle<Struct<T, Validated>>
where
    T: StructFieldType + RustType,
{
    fn annotate_rust_with_lifetime(&self) -> bool {
        self.fields
            .iter()
            .any(|f| f.field_type.rust_requires_lifetime())
    }

    fn annotate_c_with_lifetime(&self) -> bool {
        self.fields
            .iter()
            .any(|f| f.field_type.c_requires_lifetime())
    }

    fn has_conversion(&self) -> bool {
        self.fields.iter().any(|f| f.field_type.has_conversion())
    }
}
