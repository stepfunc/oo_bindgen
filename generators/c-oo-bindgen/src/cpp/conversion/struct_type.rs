use crate::cpp::by_unique_ptr;
use crate::cpp::conversion::CoreType;
use oo_bindgen::collection::CollectionHandle;
use oo_bindgen::function::FunctionArgument;
use oo_bindgen::structs::{
    CallbackArgStructField, FunctionArgStructField, FunctionReturnStructField, UniversalStructField,
};
use oo_bindgen::types::{BasicType, StringType};

pub(crate) trait CppStructType {
    fn struct_member_type(&self) -> String;
}

impl CppStructType for BasicType {
    fn struct_member_type(&self) -> String {
        self.core_type()
    }
}

impl CppStructType for StringType {
    fn struct_member_type(&self) -> String {
        self.core_type()
    }
}

impl CppStructType for CollectionHandle {
    fn struct_member_type(&self) -> String {
        format!("std::vector<{}>", self.item_type.struct_member_type())
    }
}

impl CppStructType for FunctionArgument {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionArgument::Basic(x) => x.struct_member_type(),
            FunctionArgument::String(x) => x.struct_member_type(),
            FunctionArgument::Collection(x) => x.struct_member_type(),
            FunctionArgument::Struct(_) => unimplemented!(),
            FunctionArgument::StructRef(_) => unimplemented!(),
            FunctionArgument::ClassRef(_) => unimplemented!(),
            FunctionArgument::Interface(_) => unimplemented!(),
        }
    }
}

impl CppStructType for FunctionArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionArgStructField::Basic(x) => x.struct_member_type(),
            FunctionArgStructField::String(x) => x.struct_member_type(),
            FunctionArgStructField::Interface(x) => by_unique_ptr(x.core_type()),
            FunctionArgStructField::Collection(x) => x.struct_member_type(),
            FunctionArgStructField::Struct(x) => x.core_type(),
        }
    }
}

impl CppStructType for FunctionReturnStructField {
    fn struct_member_type(&self) -> String {
        match self {
            FunctionReturnStructField::Basic(x) => x.struct_member_type(),
            FunctionReturnStructField::ClassRef(x) => x.core_type(),
            FunctionReturnStructField::Iterator(x) => x.core_type(),
            FunctionReturnStructField::Struct(x) => x.core_type(),
        }
    }
}

impl CppStructType for CallbackArgStructField {
    fn struct_member_type(&self) -> String {
        match self {
            CallbackArgStructField::Basic(x) => x.struct_member_type(),
            CallbackArgStructField::Iterator(x) => format!("{}&", x.core_type()),
            CallbackArgStructField::Struct(x) => x.core_type(),
        }
    }
}

impl CppStructType for UniversalStructField {
    fn struct_member_type(&self) -> String {
        match self {
            UniversalStructField::Basic(x) => x.struct_member_type(),
            UniversalStructField::Struct(x) => x.core_type(),
        }
    }
}
