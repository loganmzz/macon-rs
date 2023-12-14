#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
}

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        Default::default()
    }
}

// struct_builder
// struct_builder / struct_builder_named
pub struct FoobarBuilder<FOO=(), BAR=()> {
    foo: FOO,
    bar: BAR,
}

// default_builder
// default_builder / default_builder_named
impl core::default::Default for FoobarBuilder {
    fn default() -> Self {
        Self {
            foo: core::default::Default::default(),
            bar: core::default::Default::default(),
        }
    }
}

// impl_builder
// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_named
impl<BAR> FoobarBuilder<(),BAR> {
    pub fn foo(self, foo: u8) -> FoobarBuilder<u8,BAR> {
        FoobarBuilder {
            foo,
            bar: self.bar,
        }
    }
}

// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_named
impl<FOO> FoobarBuilder<FOO,()> {
    pub fn bar(self, bar: String) -> FoobarBuilder<FOO,String> {
        FoobarBuilder {
            bar,
            foo: self.foo,
        }
    }
}

// impl_builder
// impl_builder / impl_builder_build
// impl_builder / impl_builder_build / impl_builder_build_named
impl FoobarBuilder<u8,String> {
    pub fn build(self) -> Foobar {
        Foobar {
            foo: self.foo,
            bar: self.bar,
        }
    }
}

// test
#[test]
fn builder_build() {
    let built = Foobar::builder()
        .foo(2)
        .bar(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
        },
        built,
    );
}
