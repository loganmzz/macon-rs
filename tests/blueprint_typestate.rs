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
pub struct FoobarBuilder<FOO=(), BAR=()> {
    marker: core::marker::PhantomData<(FOO,BAR)>,
    foo: FOO,
    bar: BAR,
}

// default_builder
impl core::default::Default for FoobarBuilder {
    fn default() -> Self {
        Self {
            marker: core::marker::PhantomData,
            foo: core::default::Default::default(),
            bar: core::default::Default::default(),
        }
    }
}

// impl_builder
// impl_builder / impl_builder_setters
impl<BAR> FoobarBuilder<(),BAR> {
    pub fn foo(self, foo: u8) -> FoobarBuilder<u8,BAR> {
        FoobarBuilder {
            marker: core::marker::PhantomData,
            foo,
            bar: self.bar,
        }
    }
}

impl<FOO> FoobarBuilder<FOO,()> {
    pub fn bar(self, bar: String) -> FoobarBuilder<FOO,String> {
        FoobarBuilder {
            marker: core::marker::PhantomData,
            bar,
            foo: self.foo,
        }
    }
}

// impl_builder / impl_builder_build
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
