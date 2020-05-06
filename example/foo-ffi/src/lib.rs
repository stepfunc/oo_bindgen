use foo::Foo;

#[repr(C)]
pub struct FooConfig {
    pub a: u8,
}

#[no_mangle]
pub extern "C" fn foo_foo_new(config: FooConfig) -> *mut Foo {
    println!("Constructor called");
    let config = foo::FooConfig {
        a: config.a,
    };
    let foo = Box::new(Foo::new(&config));
    Box::into_raw(foo)
}

#[no_mangle]
pub extern "C" fn foo_foo_free(this: *mut Foo) {
    println!("Destructor called");
    unsafe {
        Box::from_raw(this);
    }
}
