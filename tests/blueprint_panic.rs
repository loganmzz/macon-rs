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
    pub fn foo(&mut self, foo: u8) -> &mut Self {
        self.foo = foo.into();
        self
    }

    pub fn bar(&mut self, bar: String) -> &mut Self {
        self.bar = bar.into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_named
    pub fn build(&mut self) -> Foobar {
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
                foo: self.foo.take().unwrap(),
                bar: self.bar.take().unwrap(),
            }
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
