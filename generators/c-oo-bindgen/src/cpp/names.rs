use heck::{CamelCase, SnakeCase};
use oo_bindgen::class::{
    AsyncMethod, ClassDeclarationHandle, ClassHandle, Method, StaticClassHandle,
};
use oo_bindgen::constants::Constant;
use oo_bindgen::enum_type::{EnumHandle, EnumVariant};
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::interface::{CallbackFunction, InterfaceHandle};
use oo_bindgen::structs::common::StructDeclaration;
use oo_bindgen::structs::function_return_struct::RStructHandle;
use oo_bindgen::types::Arg;
use oo_bindgen::StructType;

pub(crate) trait CppName {
    fn cpp_name(&self) -> String;
}

impl CppName for RStructHandle {
    fn cpp_name(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CppName for StructDeclaration {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for StructType {
    fn cpp_name(&self) -> String {
        self.declaration().cpp_name()
    }
}

impl CppName for EnumHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for EnumVariant {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for ErrorType {
    fn cpp_name(&self) -> String {
        self.exception_name.to_camel_case()
    }
}

impl CppName for InterfaceHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl<T> CppName for Arg<T>
{
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for CallbackFunction {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for Constant {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for ClassDeclarationHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for ClassHandle {
    fn cpp_name(&self) -> String {
        self.name().to_camel_case()
    }
}

impl CppName for Method {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for AsyncMethod {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for StaticClassHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}
