use heck::{CamelCase, SnakeCase};
use oo_bindgen::callback::{CallbackFunction, InterfaceHandle};
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::constants::Constant;
use oo_bindgen::error_type::ErrorType;
use oo_bindgen::iterator::IteratorHandle;
use oo_bindgen::native_enum::{EnumVariant, NativeEnumHandle};
use oo_bindgen::native_function::Parameter;
use oo_bindgen::native_struct::{NativeStructDeclaration, NativeStructElement, NativeStructHandle};

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

impl CppName for NativeEnumHandle {
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
        self.name.to_camel_case()
    }
}

impl CppName for InterfaceHandle {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for Parameter {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
    }
}

impl CppName for CallbackFunction {
    fn cpp_name(&self) -> String {
        self.name.to_camel_case()
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

impl CppName for IteratorHandle {
    fn cpp_name(&self) -> String {
        self.name().to_camel_case()
    }
}
