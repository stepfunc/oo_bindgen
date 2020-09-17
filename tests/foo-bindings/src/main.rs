use std::path::Path;

fn main() {
    let builder_settings = ci_script::BindingBuilderSettings {
        ffi_name: "foo_ffi",
        destination_path: Path::new("tests/bindings"),
        library: &foo_schema::build_lib().unwrap(),
    };

    ci_script::run(builder_settings);
}
