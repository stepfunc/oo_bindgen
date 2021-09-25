use heck::{CamelCase, SnakeCase};
use oo_bindgen::callback::{CallbackFunction, InterfaceHandle};
use oo_bindgen::class::{
    AsyncMethod, ClassDeclarationHandle, ClassHandle, Method, StaticClassHandle,
};
use oo_bindgen::constants::Constant;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::native_enum::{EnumHandle, EnumVariant};
use oo_bindgen::native_function::Parameter;
use oo_bindgen::native_struct::{NativeStructElement, NativeStructHandle, StructHandle};
use oo_bindgen::struct_common::NativeStructDeclaration;

pub(crate) trait CppName {
    fn cpp_name(&self) -> String;
}

impl CppName for NativeStructDeclaration {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for NativeStructHandle {
    fn cpp_name(&self) -> String {
        self.declaration.cpp_name()
    }
}

impl CppName for &NativeStructHandle {
    fn cpp_name(&self) -> String {
        self.declaration.cpp_name()
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

impl CppName for NativeStructElement {
    fn cpp_name(&self) -> String {
        self.name.to_snake_case()
    }
}

impl CppName for InterfaceHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for Parameter {
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

impl CppName for StructHandle {
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
