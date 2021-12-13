use oo_bindgen::model::*;

pub(crate) trait GuardType {
    fn guard_type(&self) -> Option<String>;
}

impl GuardType for BasicType {
    fn guard_type(&self) -> Option<String> {
        None
    }
}

impl GuardType for StringType {
    fn guard_type(&self) -> Option<String> {
        Some("jni::strings::JavaStr<'a, 'a>".to_string())
    }
}

impl GuardType for AsynchronousInterface {
    fn guard_type(&self) -> Option<String> {
        None
    }
}

impl GuardType for UniversalStructHandle {
    fn guard_type(&self) -> Option<String> {
        Some(format!("{}Guard<'a>", self.name().camel_case()))
    }
}

impl GuardType for UniversalOr<FunctionArgStructField> {
    fn guard_type(&self) -> Option<String> {
        Some(format!("{}Guard<'a>", self.name().camel_case()))
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
}

impl GuardType for UniversalStructField {
    fn guard_type(&self) -> Option<String> {
        match self {
            UniversalStructField::Basic(x) => x.guard_type(),
            UniversalStructField::Struct(x) => x.guard_type(),
        }
    }
}
