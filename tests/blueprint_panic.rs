#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
}

// struct_builder
// struct_builder / struct_builder_named
#[derive(Default)]
pub struct FoobarBuilder {
    foo: Option<u8>,
    bar: Option<String>,
}

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        Default::default()
    }
}

// impl_builder
impl FoobarBuilder {
    // impl_builder_setters
    // impl_builder_setters / impl_builder_setters_named
    pub fn foo<FOO: Into<u8>>(mut self, foo: FOO) -> Self {
        self.foo = foo.into().into();
        self
    }

    pub fn bar<BAR: Into<String>>(mut self, bar: BAR) -> Self {
        self.bar = bar.into().into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_named
    pub fn build(self) -> Foobar {
        let mut errors: Vec<String> = vec![];

        if self.foo.is_none() {
            errors.push("Field foo is missing".into());
        }
        if self.bar.is_none() {
            errors.push("Field bar is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"))
        } else {
            Foobar {
                foo: self.foo.unwrap(),
                bar: self.bar.unwrap(),
            }
        }
    }
}

// test
#[test]
fn builder_build() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
        },
        built,
    );
}
