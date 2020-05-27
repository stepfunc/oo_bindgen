use oo_bindgen::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder, association_class: ClassDeclarationHandle) -> Result<(), BindingError> {
    let destroy_fn = lib.declare_native_function("association_destroy")?
        .param("association", Type::ClassRef(association_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    lib.define_class(&association_class)?
        .destructor(&destroy_fn)?
        .build();

    Ok(())
}
