use std::path::Path;
use std::rc::Rc;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    let builder_settings = oo_bindgen::cli::BindingBuilderSettings {
        ffi_target_name: "foo-ffi",
        jni_target_name: "foo-ffi-api",
        ffi_name: "foo_ffi",
        ffi_path: Path::new("tests/foo-ffi").into(),
        java_group_id: "io.stepfunc",
        destination_path: Path::new("tests/bindings").into(),
        library: Rc::new(foo_schema::build_lib().unwrap()),
    };

    oo_bindgen::cli::run(builder_settings);
}
