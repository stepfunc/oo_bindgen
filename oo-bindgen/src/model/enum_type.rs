use std::rc::Rc;

use crate::model::*;

#[derive(Debug, Clone)]
pub(crate) struct EnumVariant<T>
where
    T: DocReference,
{
    pub(crate) name: Name,
    pub(crate) value: i32,
    pub(crate) doc: Doc<T>,
}

impl EnumVariant<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<EnumVariant<Validated>> {
        Ok(EnumVariant {
            name: self.name.clone(),
            value: self.value,
            doc: self.doc.validate(&self.name, lib)?,
        })
    }
}

pub type EnumHandle = Handle<Enum<Unvalidated>>;

impl Handle<Enum<Unvalidated>> {
    pub fn value(&self, name: &'static str) -> BindResult<EnumValue> {
        EnumValue::new(self.clone(), name)
    }
}

#[derive(Debug)]
pub struct Enum<T>
where
    T: DocReference,
{
    pub(crate) name: Name,
    pub(crate) settings: Rc<LibrarySettings>,
    pub(crate) variants: Vec<EnumVariant<T>>,
    pub(crate) doc: Doc<T>,
}

impl Enum<Unvalidated> {
    pub(crate) fn validate(&self, lib: &LibraryFields) -> BindResult<Handle<Enum<Validated>>> {
        let variants: BindResult<Vec<EnumVariant<Validated>>> =
            self.variants.iter().map(|x| x.validate(lib)).collect();

        Ok(Handle::new(Enum {
            name: self.name.clone(),
            settings: self.settings.clone(),
            variants: variants?,
            doc: self.doc.validate(&self.name, lib)?,
        }))
    }
}

impl<T> Enum<T>
where
    T: DocReference,
{
    pub(crate) fn find_variant_by_name<S: AsRef<str>>(
        &self,
        variant_name: S,
    ) -> Option<&EnumVariant<T>> {
        self.variants
            .iter()
            .find(|variant| variant.name.as_ref() == variant_name.as_ref())
    }

    pub(crate) fn validate_contains_variant_name(
        &self,
        variant_name: &str,
    ) -> BindResult<&EnumVariant<T>> {
        match self.find_variant_by_name(variant_name) {
            None => Err(BindingErrorVariant::UnknownEnumVariant {
                name: self.name.clone(),
                variant_name: variant_name.to_string(),
            }
            .into()),
            Some(x) => Ok(x),
        }
    }
}

impl From<EnumHandle> for BasicType {
    fn from(x: EnumHandle) -> Self {
        BasicType::Enum(x)
    }
}
