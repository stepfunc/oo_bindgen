use oo_bindgen::*;
use oo_bindgen::class::ClassDeclarationHandle;
use oo_bindgen::interface::InterfaceHandle;
use oo_bindgen::native_function::*;

pub fn define(lib: &mut LibraryBuilder, master_class: ClassDeclarationHandle, read_handler: InterfaceHandle) -> Result<ClassDeclarationHandle, BindingError> {
    let destroy_fn = lib.declare_native_function("master_destroy")?
        .param("master", Type::ClassRef(master_class.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    // Association creation
    let association_class = lib.declare_class("Association")?;

    let event_classes = lib.declare_native_struct("EventClasses")?;
    let event_classes = lib.define_native_struct(&event_classes)?
        .add("class1", Type::Bool)?
        .add("class2", Type::Bool)?
        .add("class3", Type::Bool)?
        .build();

    let auto_time_sync_enum = lib.define_native_enum("AutoTimeSync")?
        .push("None")?
        .push("LAN")?
        .push("NonLAN")?
        .build();

    let association_configuration = lib.declare_native_struct("AssociationConfiguration")?;
    let association_configuration = lib.define_native_struct(&association_configuration)?
        .add("disable_unsol_classes", Type::Struct(event_classes.clone()))?
        .add("enable_unsol_classes", Type::Struct(event_classes))?
        .add("auto_time_sync", Type::Enum(auto_time_sync_enum))?
        .build();

    let association_handlers = lib.declare_native_struct("AssociationHandlers")?;
    let association_handlers = lib.define_native_struct(&association_handlers)?
        .add("integrity_handler", Type::Interface(read_handler.clone()))?
        .add("unsolicited_handler", Type::Interface(read_handler.clone()))?
        .add("default_poll_handler", Type::Interface(read_handler))?
        .build();

    let add_association_fn = lib.declare_native_function("master_add_association")?
        .param("master", Type::ClassRef(master_class.clone()))?
        .param("address", Type::Uint16)?
        .param("config", Type::Struct(association_configuration))?
        .param("handlers", Type::Struct(association_handlers))?
        .return_type(ReturnType::Type(Type::ClassRef(association_class.clone())))?
        .build()?;

    lib.define_class(&master_class)?
        .method("AddAssociation", &add_association_fn)?
        .destructor(&destroy_fn)?
        .build();

    Ok(association_class)
}
