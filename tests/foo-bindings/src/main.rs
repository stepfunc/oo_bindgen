use std::path::Path;
use std::rc::Rc;

fn main() {
    let builder_settings = ci_script::BindingBuilderSettings {
        ffi_target_name: "foo-ffi",
        ffi_name: "foo_ffi",
        ffi_path: Path::new("tests/foo-ffi").into(),
        java_group_id: "io.stepfunc",
        destination_path: Path::new("tests/bindings").into(),
        library: Rc::new(foo_schema::build_lib().unwrap()),
    };

    ci_script::run(builder_settings);
}
