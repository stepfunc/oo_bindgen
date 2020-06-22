use oo_bindgen::native_enum::NativeEnumHandle;
use oo_bindgen::native_function::*;
use oo_bindgen::class::ClassHandle;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(ClassHandle, NativeEnumHandle), BindingError> {
    let variation_enum = lib
        .define_native_enum("Variation")?
        .push("Group1Var0")?
        .push("Group1Var1")?
        .push("Group1Var2")?
        .push("Group2Var0")?
        .push("Group2Var1")?
        .push("Group2Var2")?
        .push("Group2Var3")?
        .push("Group3Var0")?
        .push("Group3Var1")?
        .push("Group3Var2")?
        .push("Group4Var0")?
        .push("Group4Var1")?
        .push("Group4Var2")?
        .push("Group4Var3")?
        .push("Group10Var0")?
        .push("Group10Var1")?
        .push("Group10Var2")?
        .push("Group11Var0")?
        .push("Group11Var1")?
        .push("Group11Var2")?
        .push("Group12Var0")?
        .push("Group12Var1")?
        .push("Group13Var1")?
        .push("Group13Var2")?
        .push("Group20Var0")?
        .push("Group20Var1")?
        .push("Group20Var2")?
        .push("Group20Var5")?
        .push("Group20Var6")?
        .push("Group21Var0")?
        .push("Group21Var1")?
        .push("Group21Var2")?
        .push("Group21Var5")?
        .push("Group21Var6")?
        .push("Group21Var9")?
        .push("Group21Var10")?
        .push("Group22Var0")?
        .push("Group22Var1")?
        .push("Group22Var2")?
        .push("Group22Var5")?
        .push("Group22Var6")?
        .push("Group23Var0")?
        .push("Group23Var1")?
        .push("Group23Var2")?
        .push("Group23Var5")?
        .push("Group23Var6")?
        .push("Group30Var0")?
        .push("Group30Var1")?
        .push("Group30Var2")?
        .push("Group30Var3")?
        .push("Group30Var4")?
        .push("Group30Var5")?
        .push("Group30Var6")?
        .push("Group32Var0")?
        .push("Group32Var1")?
        .push("Group32Var2")?
        .push("Group32Var3")?
        .push("Group32Var4")?
        .push("Group32Var5")?
        .push("Group32Var6")?
        .push("Group32Var7")?
        .push("Group32Var8")?
        .push("Group40Var0")?
        .push("Group40Var1")?
        .push("Group40Var2")?
        .push("Group40Var3")?
        .push("Group40Var4")?
        .push("Group41Var0")?
        .push("Group41Var1")?
        .push("Group41Var2")?
        .push("Group41Var3")?
        .push("Group41Var4")?
        .push("Group42Var0")?
        .push("Group42Var1")?
        .push("Group42Var2")?
        .push("Group42Var3")?
        .push("Group42Var4")?
        .push("Group42Var5")?
        .push("Group42Var6")?
        .push("Group42Var7")?
        .push("Group42Var8")?
        .push("Group43Var1")?
        .push("Group43Var2")?
        .push("Group43Var3")?
        .push("Group43Var4")?
        .push("Group43Var5")?
        .push("Group43Var6")?
        .push("Group43Var7")?
        .push("Group43Var8")?
        .push("Group50Var1")?
        .push("Group50Var3")?
        .push("Group50Var4")?
        .push("Group51Var1")?
        .push("Group51Var2")?
        .push("Group52Var1")?
        .push("Group52Var2")?
        .push("Group60Var1")?
        .push("Group60Var2")?
        .push("Group60Var3")?
        .push("Group60Var4")?
        .push("Group80Var1")?
        .push("Group110")?
        .push("Group111")?
        .push("Group112")?
        .push("Group113")?
        .build();

    let request = lib.declare_class("Request")?;

    let request_new_fn = lib.declare_native_function("request_new")?
        .return_type(ReturnType::Type(Type::ClassRef(request.clone())))?
        .build()?;

    let request_new_class_fn = lib.declare_native_function("request_new_class")?
        .param("class0", Type::Bool)?
        .param("class1", Type::Bool)?
        .param("class2", Type::Bool)?
        .param("class3", Type::Bool)?
        .return_type(ReturnType::Type(Type::ClassRef(request.clone())))?
        .build()?;

    let request_destroy_fn = lib.declare_native_function("request_destroy")?
        .param("request", Type::ClassRef(request.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let request_add_one_byte_header_fn = lib.declare_native_function("request_add_one_byte_header")?
        .param("request", Type::ClassRef(request.clone()))?
        .param("variation", Type::Enum(variation_enum.clone()))?
        .param("start", Type::Uint8)?
        .param("stop", Type::Uint8)?
        .return_type(ReturnType::Void)?
        .build()?;

    let request_add_two_byte_header_fn = lib.declare_native_function("request_add_two_byte_header")?
        .param("request", Type::ClassRef(request.clone()))?
        .param("variation", Type::Enum(variation_enum.clone()))?
        .param("start", Type::Uint16)?
        .param("stop", Type::Uint16)?
        .return_type(ReturnType::Void)?
        .build()?;

    let request_add_all_objects_header_fn = lib.declare_native_function("request_add_all_objects_header")?
        .param("request", Type::ClassRef(request.clone()))?
        .param("variation", Type::Enum(variation_enum.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let request = lib.define_class(&request)?
        .constructor(&request_new_fn)?
        .destructor(&request_destroy_fn)?
        .static_method("ClassRequest", &request_new_class_fn)?
        .method("AddOneByteHeader", &request_add_one_byte_header_fn)?
        .method("AddTwoByteHeader", &request_add_two_byte_header_fn)?
        .method("AddAllObjectsHeader", &request_add_all_objects_header_fn)?
        .build();

    Ok((request, variation_enum))
}
