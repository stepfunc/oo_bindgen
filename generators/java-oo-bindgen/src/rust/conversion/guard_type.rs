use oo_bindgen::model::*;

pub(crate) trait GuardType {
    fn guard_type(&self) -> Option<String>;
    fn guard_transform(&self, expr: &str) -> Option<String>;
}

impl GuardType for BasicType {
    fn guard_type(&self) -> Option<String> {
        None
    }

    fn guard_transform(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl GuardType for StringType {
    fn guard_type(&self) -> Option<String> {
        Some("jni::strings::JavaStr<'a, 'a>".to_string())
    }

    fn guard_transform(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl GuardType for AsynchronousInterface {
    fn guard_type(&self) -> Option<String> {
        None
    }

    fn guard_transform(&self, _expr: &str) -> Option<String> {
        None
    }
}

impl<T> GuardType for Handle<Struct<T, Unvalidated>>
where
    T: StructFieldType,
{
    fn guard_type(&self) -> Option<String> {
        Some(format!("{}Guard<'a>", self.name().camel_case()))
    }

    fn guard_transform(&self, expr: &str) -> Option<String> {
        Some(format!("{}.0", expr))
    }
}

impl GuardType for UniversalOr<FunctionArgStructField> {
    fn guard_type(&self) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.guard_type(),
            UniversalOr::Universal(x) => x.guard_type(),
        }
    }

    fn guard_transform(&self, expr: &str) -> Option<String> {
        match self {
            UniversalOr::Specific(x) => x.guard_transform(expr),
            UniversalOr::Universal(x) => x.guard_transform(expr),
        }
    }
}

impl GuardType for FunctionArgStructField {
    fn guard_type(&self) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.guard_type(),
            FunctionArgStructField::String(x) => x.guard_type(),
            FunctionArgStructField::Interface(x) => x.guard_type(),
            FunctionArgStructField::Struct(x) => x.guard_type(),
        }
    }

    fn guard_transform(&self, expr: &str) -> Option<String> {
        match self {
            FunctionArgStructField::Basic(x) => x.guard_transform(expr),
            FunctionArgStructField::String(x) => x.guard_transform(expr),
            FunctionArgStructField::Interface(x) => x.guard_transform(expr),
            FunctionArgStructField::Struct(x) => x.guard_transform(expr),
        }
    }
}

impl GuardType for UniversalStructField {
    fn guard_type(&self) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.guard_type(),
            UniversalStructField::Struct(x) => x.guard_type(),
        }
    }

    fn guard_transform(&self, expr: &str) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.guard_transform(expr),
            UniversalStructField::Struct(x) => x.guard_transform(expr),
        }
    }
}
