pub struct FooConfig {
    pub a: u8,
}

pub struct Foo {
    a: u8,
}

impl Foo {
    pub fn new(config: &FooConfig) -> Self {
        Self {
            a: config.a,
        }
    }
}
