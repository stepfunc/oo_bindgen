use std::path::Path;

use crate::backend::c::cpp::conversion::*;
use crate::backend::c::cpp::doc::*;
use crate::backend::*;
use crate::model::*;

use crate::backend::c::cpp::formatting::*;
use crate::backend::c::formatting::*;

pub(crate) fn generate_header(lib: &Library, path: &Path) -> FormattingResult<()> {
    // Open the file
    logged::create_dir_all(&path)?;
    let filename = path.join(format!("{}.hpp", lib.settings.name));
    let mut f = FilePrinter::new(filename)?;

    print_license(&mut f, lib)?;

    // include guard
    f.writeln("#pragma once")?;
    f.newline()?;
    f.writeln("#include <cstdint>")?;
    f.writeln("#include <stdexcept>")?;
    f.writeln("#include <chrono>")?;
    f.writeln("#include <memory>")?;
    f.writeln("#include <vector>")?;
    f.writeln("#include <string>")?;
    f.newline()?;

    doxygen(&mut f, |f| {
        // Doxygen needs the @file tag
        f.writeln(&format!(
            "@file {}.hpp C++ API for the {} library",
            lib.settings.name, lib.settings.name,
        ))?;

        f.newline()?;

        // Doxygen main page
        f.writeln("@mainpage")?;
        f.newline()?;
        f.writeln(&lib.info.description)?;
        f.newline()?;
        f.writeln(&format!(
            "For complete documentation, see @ref {} namespace",
            lib.settings.name
        ))?;
        f.newline()?;
        f.writeln("@section license License")?;
        f.newline()?;
        for line in &lib.info.license_description {
            f.writeln(line)?;
        }

        Ok(())
    })?;
    f.newline()?;

    f.writeln(&format!(
        "/// main namespace for the {} library",
        lib.settings.name
    ))?;
    namespace(&mut f, &lib.settings.c_ffi_prefix, |f| {
        print_header_namespace_contents(lib, f)
    })?;

    Ok(())
}

fn print_header_namespace_contents(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    print_version(lib, f)?;
    f.newline()?;

    let mut documented_functional_ns = false;

    for statement in lib.statements() {
        match &statement {
            Statement::Constants(x) => print_constants(f, x)?,
            Statement::EnumDefinition(x) => print_enum(f, x)?,
            Statement::ErrorType(x) => print_exception(f, x)?,
            Statement::StructDeclaration(x) => print_struct_decl(f, x)?,
            Statement::StructDefinition(x) => match x {
                StructType::FunctionArg(x) => print_struct_definition(f, x)?,
                StructType::FunctionReturn(x) => print_struct_definition(f, x)?,
                StructType::CallbackArg(x) => print_struct_definition(f, x)?,
                StructType::Universal(x) => print_struct_definition(f, x)?,
            },
            Statement::InterfaceDefinition(x) => {
                print_interface(f, x.untyped())?;

                if let Some(callback) = x.untyped().get_functional_callback() {
                    if !documented_functional_ns {
                        documented_functional_ns = true;
                        f.writeln("/// helpers functions to create interface implementations using lambdas")?;
                    }
                    namespace(f, "functional", |f| {
                        write_functional_interface_helpers(f, x.untyped(), callback)
                    })?;
                    f.newline()?;
                }
            }
            Statement::ClassDeclaration(x) => {
                match x.class_type {
                    ClassType::Normal => print_class_decl(f, x)?,
                    ClassType::Iterator => print_class_decl(f, x)?,
                    // collections are mapped to Vec<T> in C++ and therefore
                    // have no opaque declaration in the header
                    ClassType::Collection => {}
                }
            }
            Statement::ClassDefinition(x) => print_class_definition(f, x)?,
            Statement::StaticClassDefinition(x) => print_static_class(f, x)?,
            Statement::IteratorDeclaration(x) => print_iterator_definition(f, x)?,
            Statement::CollectionDeclaration(_) => {
                // collections are just vectors in C++
            }
            Statement::FunctionDefinition(_) => {
                // not used in C++
            }
        }
    }

    Ok(())
}

fn write_functional_interface_helpers(
    f: &mut dyn Printer,
    interface: &Handle<Interface<Validated>>,
    callback: &CallbackFunction<Validated>,
) -> FormattingResult<()> {
    let interface_name = interface.core_cpp_type();

    doxygen(f, |f| {
        f.writeln(&format!(
            "@brief class that implements @ref {} in terms of a lambda expression",
            interface_name
        ))?;
        f.writeln(&format!(
            "@note this class can only be constructed using @ref {}() helper function",
            interface.name
        ))?;
        Ok(())
    })?;
    let class_name = format!("{}Lambda", interface_name);
    f.writeln("template <class T>")?;
    f.writeln(&format!(
        "class {} final : public {}",
        class_name, interface_name
    ))?;
    f.writeln("{")?;
    indented(f, |f| {
        f.writeln("static_assert(std::is_copy_constructible<T>::value, \"Lambda expression must be copy constructible. Does it contain something that is move-only?\");")?;
        f.newline()?;
        f.writeln("T lambda;")
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        doxygen(f, |f| {
            f.writeln("@brief constructor")?;
            f.writeln("@param lambda functor value on which to base the interface implementation")?;
            Ok(())
        })?;
        f.writeln(&format!(
            "{}(const T& lambda) : lambda(lambda) {{}}",
            class_name
        ))?;
        f.newline()?;

        let return_type = callback.return_type.get_cpp_callback_return_type();
        let args = callback
            .arguments
            .iter()
            .map(|x| format!("{} {}", x.arg_type.get_cpp_callback_arg_type(), x.name))
            .collect::<Vec<String>>()
            .join(", ");

        f.writeln("/// @brief implement virtual method from base class")?;
        f.writeln(&format!(
            "{} {}({}) override",
            return_type, callback.name, args
        ))?;
        let args = callback
            .arguments
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let invocation = &format!("lambda({});", args);
        if callback.return_type.is_none() {
            blocked(f, |f| f.writeln(invocation))?;
        } else {
            blocked(f, |f| f.writeln(&format!("return {}", invocation)))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;

    f.newline()?;

    let is_synchronous = match interface.mode {
        InterfaceCategory::Synchronous => true,
        InterfaceCategory::Asynchronous => false,
        InterfaceCategory::Future => false,
    };

    let return_type = if is_synchronous {
        format!("{}<T>", class_name)
    } else {
        format!("std::unique_ptr<{}>", interface_name)
    };

    let return_expr = if is_synchronous {
        format!("{}<T>(lambda)", class_name)
    } else {
        format!("std::make_unique<{}<T>>(lambda); ", class_name)
    };

    doxygen(f, |f| {
        f.writeln(&format!(
            "@brief construct an implementation of @ref {} based on a lambda expression",
            interface_name
        ))?;
        f.writeln("@note T must be copy-constructible to use this function")?;
        f.writeln("@param lambda functor value on which to base the interface implementation")?;
        if is_synchronous {
            f.writeln("@return concrete implementation of the interface")?;
        } else {
            f.writeln("@return abstract implementation of the interface in a unique_ptr")?;
        }
        Ok(())
    })?;
    f.writeln("template <class T>")?;
    f.writeln(&format!(
        "{} {}(const T& lambda)",
        return_type, interface.name
    ))?;
    blocked(f, |f| f.writeln(&format!("return {}; ", return_expr)))?;

    f.newline()
}

fn print_iterator_definition(
    f: &mut dyn Printer,
    iter: &Handle<AbstractIterator<Validated>>,
) -> FormattingResult<()> {
    let iterator = include_str!("snippet/iterator.hpp");
    for line in iterator.lines() {
        let substituted = line
            .replace("<name>", &iter.core_cpp_type())
            .replace("<snake_name>", &iter.core_cpp_type())
            .replace("<iter_type>", &iter.item_type.core_cpp_type());
        f.writeln(&substituted)?;
    }
    f.newline()
}

fn print_class_decl(f: &mut dyn Printer, handle: &ClassDeclarationHandle) -> FormattingResult<()> {
    f.writeln(&format!("class {};", handle.core_cpp_type()))?;
    f.newline()
}

fn print_version(lib: &Library, f: &mut dyn Printer) -> FormattingResult<()> {
    let name = lib.settings.c_ffi_prefix.clone();

    f.writeln("/// major version number")?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_major = {};",
        name, lib.version.major
    ))?;

    f.writeln("/// minor version number")?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_minor = {};",
        name, lib.version.minor
    ))?;

    f.writeln("/// patch version number")?;
    f.writeln(&format!(
        "constexpr uint64_t {}_version_patch = {};",
        name, lib.version.patch
    ))?;

    f.writeln("/// version number as the string major.minor.patch")?;
    f.writeln(&format!(
        "constexpr char const* {}_version_string = \"{}\";",
        name, lib.version
    ))?;
    f.newline()
}

fn print_constants(
    f: &mut dyn Printer,
    set: &Handle<ConstantSet<Validated>>,
) -> FormattingResult<()> {
    fn get_value(v: ConstantValue) -> String {
        match v {
            ConstantValue::U8(v, Representation::Hex) => format!("0x{:02X}", v),
        }
    }

    fn get_type(v: ConstantValue) -> &'static str {
        match v {
            ConstantValue::U8(_, _) => "uint8_t",
        }
    }

    print_commented_cpp_doc(f, &set.doc)?;
    f.writeln(&format!("namespace {} {{", set.name))?;
    indented(f, |f| {
        for value in &set.values {
            print_commented_cpp_doc(f, &value.doc)?;
            f.writeln(&format!(
                "constexpr {} {} = {};",
                get_type(value.value),
                value.core_cpp_type(),
                get_value(value.value)
            ))?;
        }
        Ok(())
    })?;
    f.writeln("}")?;
    f.newline()
}

fn print_enum(f: &mut dyn Printer, e: &Handle<Enum<Validated>>) -> FormattingResult<()> {
    print_commented_cpp_doc(f, &e.doc)?;
    f.writeln(&format!("enum class {} {{", e.core_cpp_type()))?;
    indented(f, |f| {
        for v in &e.variants {
            print_commented_cpp_doc(f, &v.doc)?;
            f.writeln(&format!("{} = {},", v.core_cpp_type(), v.value))?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()?;
    doxygen(f, |f| {
        f.writeln(&format!(
            "@brief convert an instance of enum {} into a C-style string",
            e.core_cpp_type()
        ))?;
        f.writeln("@param value enum value")?;
        f.writeln("@return C-style string constant")
    })?;
    f.writeln(&format!(
        "const char* to_string({} value);",
        e.core_cpp_type()
    ))?;
    f.newline()
}

fn print_exception(f: &mut dyn Printer, e: &ErrorType<Validated>) -> FormattingResult<()> {
    f.writeln(&format!(
        "/// @brief Exception type corresponding to the underlying error enum #{}",
        e.inner.core_cpp_type()
    ))?;
    f.writeln(&format!(
        "class {} : public std::logic_error {{",
        e.core_cpp_type()
    ))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln("/// @brief underlying error enum")?;
        f.writeln(&format!("{} error;", e.inner.core_cpp_type()))?;
        f.writeln("/// @brief construct the exception with an instance of the enum")?;
        f.writeln(&format!(
            "{}({} error) : std::logic_error(to_string(error)), error(error) {{}}",
            e.core_cpp_type(),
            e.inner.core_cpp_type()
        ))?;
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_struct_decl(f: &mut dyn Printer, s: &StructDeclaration) -> FormattingResult<()> {
    f.writeln(&format!("struct {};", s.core_cpp_type()))?;
    f.newline()
}

fn print_struct_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T, Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppStructType + CppFunctionArgType,
{
    print_commented_cpp_doc(f, &handle.doc)?;
    f.writeln(&format!("struct {} {{", handle.core_cpp_type()))?;

    indented(f, |f| {
        f.writeln(&format!("friend class {};", handle.friend_class()))?;
        f.newline()
    })?;

    if let Visibility::Private = handle.visibility {
        f.writeln("private:")?;
        f.newline()?
    }

    if !handle.has_full_initializer() {
        if handle.visibility == Visibility::Public {
            f.writeln("private:")?;
        }
        indented(f, |f| {
            // write a default constructor
            let constructor = Handle::new(Initializer::full(
                InitializerType::Normal,
                brief("Fully initialize the struct"),
            ));
            print_initializer_definition(f, handle, &constructor)
        })?;
        if handle.visibility == Visibility::Public {
            f.writeln("public:")?;
        }
    }

    indented(f, |f| {
        // delete the default constructor unless the struct has one
        if !handle.has_default_initializer() {
            f.writeln(&format!("{}() = delete;", handle.core_cpp_type()))?;
        }

        // write the constructors
        for c in &handle.initializers {
            f.newline()?;
            print_initializer_definition(f, handle, c)?;
        }

        f.newline()?;
        for field in &handle.fields {
            print_commented_cpp_doc(f, &field.doc)?;
            f.writeln(&format!(
                "{} {};",
                field.field_type.struct_member_type(),
                field.name
            ))?;
        }

        Ok(())
    })?;

    f.writeln("};")?;
    f.newline()
}

trait ToConstantCpp {
    fn to_constant_cpp(&self) -> String;
}

impl ToConstantCpp for PrimitiveValue {
    fn to_constant_cpp(&self) -> String {
        match self {
            PrimitiveValue::Bool(x) => x.to_string(),
            PrimitiveValue::U8(x) => x.to_string(),
            PrimitiveValue::S8(x) => x.to_string(),
            PrimitiveValue::U16(x) => x.to_string(),
            PrimitiveValue::S16(x) => x.to_string(),
            PrimitiveValue::U32(x) => x.to_string(),
            PrimitiveValue::S32(x) => x.to_string(),
            PrimitiveValue::U64(x) => x.to_string(),
            PrimitiveValue::S64(x) => x.to_string(),
            PrimitiveValue::Float(x) => x.to_string(),
            PrimitiveValue::Double(x) => x.to_string(),
        }
    }
}

impl ToConstantCpp for DurationValue {
    fn to_constant_cpp(&self) -> String {
        match self {
            DurationValue::Milliseconds(x) => format!("std::chrono::milliseconds({})", x),
            DurationValue::Seconds(x) => format!("std::chrono::Duration::seconds({})", x),
        }
    }
}

impl ToConstantCpp for EnumValue {
    fn to_constant_cpp(&self) -> String {
        format!("{}::{}", self.handle.core_cpp_type(), self.variant.name)
    }
}

impl ToConstantCpp for BasicValue {
    fn to_constant_cpp(&self) -> String {
        match self {
            BasicValue::Primitive(x) => x.to_constant_cpp(),
            BasicValue::Duration(x) => x.to_constant_cpp(),
            BasicValue::Enum(x) => x.to_constant_cpp(),
        }
    }
}

impl ToConstantCpp for ZeroParameterStructInitializer {
    fn to_constant_cpp(&self) -> String {
        match self.initializer.initializer_type {
            InitializerType::Normal => format!("{}()", self.handle.core_cpp_type()),
            InitializerType::Static => format!(
                "{}::{}()",
                self.handle.core_cpp_type(),
                self.initializer.name
            ),
        }
    }
}

trait TryToConstantCpp {
    fn try_to_constant_cpp(&self) -> Option<String>;
}

impl<T> TryToConstantCpp for T
where
    T: ToConstantCpp,
{
    fn try_to_constant_cpp(&self) -> Option<String> {
        Some(self.to_constant_cpp())
    }
}

impl TryToConstantCpp for DefaultCallbackReturnValue {
    fn try_to_constant_cpp(&self) -> Option<String> {
        match self {
            DefaultCallbackReturnValue::Void => None,
            DefaultCallbackReturnValue::Basic(x) => x.try_to_constant_cpp(),
            DefaultCallbackReturnValue::InitializedStruct(x) => x.try_to_constant_cpp(),
        }
    }
}

fn print_interface(
    f: &mut dyn Printer,
    handle: &Handle<Interface<Validated>>,
) -> FormattingResult<()> {
    doxygen(f, |f| {
        print_cpp_doc(f, &handle.doc)?;
        f.newline()?;
        f.writeln("@note this class is an \"interface\" and only has virtual methods, some of which may have default implementations.")
    })?;
    f.writeln(&format!("class {} {{", handle.core_cpp_type()))?;
    f.writeln("public:")?;
    indented(f, |f| {
        f.writeln(&format!("virtual ~{}() = default;", handle.core_cpp_type()))?;
        f.newline()?;
        for cb in &handle.callbacks {
            let args: String = cb
                .arguments
                .iter()
                .map(|arg| {
                    format!(
                        "{} {}",
                        arg.arg_type.get_cpp_callback_arg_type(),
                        arg.core_cpp_type()
                    )
                })
                .collect::<Vec<String>>()
                .join(", ");

            let default_value = cb
                .default_implementation
                .as_ref()
                .map(|v| v.try_to_constant_cpp());

            doxygen(f, |f| {
                print_cpp_doc(f, &cb.doc)?;
                f.newline()?;
                for arg in cb.arguments.iter() {
                    f.newline()?;
                    print_cpp_argument_doc(f, arg)?;
                }
                print_cpp_return_type_doc(f, &cb.return_type)?;
                if let Some(x) = &default_value {
                    match x {
                        None => {
                            f.newline()?;
                            f.writeln(
                                "@note This method has a default implementation that does nothing",
                            )?;
                        }
                        Some(value) => {
                            f.newline()?;
                            f.writeln(&format!(
                                "@note This method has a default implementation that returns '{}'",
                                value
                            ))?;
                        }
                    }
                }
                Ok(())
            })?;

            match &default_value {
                None => {
                    f.writeln(&format!(
                        "virtual {} {}({}) = 0;",
                        cb.return_type.get_cpp_callback_return_type(),
                        cb.core_cpp_type(),
                        args
                    ))?;
                }
                Some(None) => {
                    f.writeln(&format!(
                        "virtual {} {}({}) {{}};",
                        cb.return_type.get_cpp_callback_return_type(),
                        cb.core_cpp_type(),
                        args
                    ))?;
                }
                Some(Some(value)) => {
                    f.writeln(&format!(
                        "virtual {} {}({}) {{",
                        cb.return_type.get_cpp_callback_return_type(),
                        cb.core_cpp_type(),
                        args
                    ))?;
                    indented(f, |f| f.writeln(&format!("return {};", value)))?;
                    f.writeln("}")?;
                }
            }
            f.newline()?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn get_default_value_doc(default: &ValidatedDefaultValue) -> String {
    match default {
        ValidatedDefaultValue::Bool(x) => format!("@p {}", x),
        ValidatedDefaultValue::Number(x) => x.to_string(),
        ValidatedDefaultValue::Duration(DurationType::Milliseconds, x) => {
            format!("{}ms", x.as_millis())
        }
        ValidatedDefaultValue::Duration(DurationType::Seconds, x) => format!("{}s", x.as_secs()),
        ValidatedDefaultValue::Enum(x, variant) => format!("{}::{}", x.core_cpp_type(), variant),
        ValidatedDefaultValue::String(x) => format!("\"{}\"", x),
        ValidatedDefaultValue::DefaultStruct(handle, _, _) => {
            format!("Default @ref {}", handle.core_cpp_type())
        }
    }
}

fn print_initializer_definition<T>(
    f: &mut dyn Printer,
    handle: &Handle<Struct<T, Validated>>,
    initializer: &Handle<Initializer<Validated>>,
) -> FormattingResult<()>
where
    T: StructFieldType + CppFunctionArgType,
{
    let args = handle
        .initializer_args(initializer.clone())
        .map(|x| format!("{} {}", x.field_type.get_cpp_function_arg_type(), x.name))
        .collect::<Vec<String>>()
        .join(", ");

    doxygen(f, |f| {
        print_cpp_doc(f, &initializer.doc)?;

        if !initializer.values.is_empty() {
            f.newline()?;
            f.writeln("@note Values are initialized to:")?;
            for value in initializer.values.iter() {
                f.writeln(&format!(
                    "- @ref {}.{} : {}",
                    handle.core_cpp_type(),
                    value.name,
                    get_default_value_doc(&value.value)
                ))?;
            }
            f.newline()?;
        }

        for field in handle.initializer_args(initializer.clone()) {
            f.writeln(&format!("@param {} ", field.name))?;
            print_cpp_docstring(f, &field.doc.brief)?;
        }

        f.writeln(&format!(
            "@returns New instance of @ref {}",
            handle.core_cpp_type()
        ))?;

        Ok(())
    })?;
    match initializer.initializer_type {
        InitializerType::Normal => f.writeln(&format!("{}({});", handle.core_cpp_type(), args))?,
        InitializerType::Static => f.writeln(&format!(
            "static {} {}({});",
            handle.core_cpp_type(),
            initializer.name,
            args
        ))?,
    }

    f.newline()
}

fn print_class_definition(
    f: &mut dyn Printer,
    handle: &Handle<Class<Validated>>,
) -> FormattingResult<()> {
    let class_name = handle.core_cpp_type();

    print_commented_cpp_doc(f, &handle.doc)?;
    f.writeln(&format!("class {} {{", class_name))?;
    indented(f, |f| {
        f.writeln(&format!("friend class {};", handle.friend_class()))?;
        f.writeln("// pointer to the underlying C type")?;
        f.writeln("void* self;")?;
        f.writeln("// constructor only accessible internally")?;
        f.writeln(&format!(
            "{}(void* self): self(self) {{}}",
            handle.core_cpp_type()
        ))?;
        print_deleted_class_functions(f, &class_name)
    })?;
    f.newline()?;
    f.writeln("public:")?;
    indented(f, |f| {
        doxygen(f, |f| {
            f.writeln("@brief Transfer ownership of the underlying C-type to this instance and invalidate the other instance.")?;
            f.writeln("@note the moved class will now throw an exception if any method is called")?;
            f.writeln("@param other Class from which ownership will be transfer to this instance")
        })?;
        f.writeln(&format!(
            "{}({}&& other) noexcept : self(other.self) {{ other.self = nullptr; }}",
            class_name, class_name
        ))?;

        if let Some(x) = &handle.constructor {
            let args = cpp_arguments(x.function.arguments.iter());

            f.newline()?;
            print_cpp_constructor_docs(f, x)?;
            f.writeln(&format!("{}({});", class_name, args))?;
        };
        if let Some(x) = &handle.destructor {
            f.newline()?;
            print_commented_cpp_doc(f, &x.function.doc)?;
            f.writeln(&format!("~{}();", class_name))?;
        };

        for method in &handle.methods {
            f.newline()?;
            print_method(f, method)?;
        }

        for method in &handle.static_methods {
            f.newline()?;
            print_static_method(f, method)?;
        }

        for method in &handle.future_methods {
            f.newline()?;
            print_future_method(f, method)?;
        }

        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_method(f: &mut dyn Printer, method: &Method<Validated>) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.arguments.iter().skip(1));

    print_cpp_method_docs(f, method)?;
    f.writeln(&format!(
        "{} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name,
        args
    ))
}

fn print_static_method(
    f: &mut dyn Printer,
    method: &StaticMethod<Validated>,
) -> FormattingResult<()> {
    let args = cpp_arguments(method.native_function.arguments.iter());

    print_cpp_static_method_docs(f, method)?;
    f.writeln(&format!(
        "static {} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name,
        args
    ))
}

fn print_future_method(
    f: &mut dyn Printer,
    method: &FutureMethod<Validated>,
) -> FormattingResult<()> {
    let args: String = cpp_arguments(method.native_function.arguments.iter().skip(1));

    print_cpp_future_method_docs(f, method)?;
    f.writeln(&format!(
        "{} {}({});",
        method
            .native_function
            .return_type
            .get_cpp_function_return_type(),
        method.name,
        args
    ))?;

    f.newline()
}

fn print_static_class(
    f: &mut dyn Printer,
    handle: &Handle<StaticClass<Validated>>,
) -> FormattingResult<()> {
    print_commented_cpp_doc(f, &handle.doc)?;
    f.writeln(&format!("class {} {{", handle.core_cpp_type()))?;
    indented(f, |f| {
        f.writeln(&format!("{}() = delete;", handle.core_cpp_type()))
    })?;
    f.writeln("public:")?;
    indented(f, |f| {
        for method in &handle.static_methods {
            print_static_method(f, method)?;
        }
        Ok(())
    })?;
    f.writeln("};")?;
    f.newline()
}

fn print_deleted_class_functions(f: &mut dyn Printer, name: &str) -> FormattingResult<()> {
    f.writeln("// non-copyable")?;
    f.writeln(&format!("{}(const {}&) = delete;", name, name))?;
    f.writeln(&format!("{}& operator=(const {}&) = delete;", name, name))?;

    f.writeln("// no move assignment")?;
    f.writeln(&format!("{}& operator=({}&& other) = delete;", name, name))
}

fn cpp_arguments<'a, T>(iter: T) -> String
where
    T: Iterator<Item = &'a Arg<FunctionArgument, Validated>>,
{
    iter.map(|p| {
        format!(
            "{} {}",
            p.arg_type.get_cpp_function_arg_type(),
            p.core_cpp_type(),
        )
    })
    .collect::<Vec<String>>()
    .join(", ")
}
