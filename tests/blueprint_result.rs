#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
}

// struct_builder
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
    pub fn foo(&mut self, foo: u8) -> &mut Self {
        self.foo = foo.into();
        self
    }

    pub fn bar(&mut self, bar: String) -> &mut Self {
        self.bar = bar.into();
        self
    }

    // impl_builder_build
    pub fn build(&mut self) -> Result<Foobar, String> {
        let mut errors: Vec<String> = vec![];

        if self.foo.is_none() {
            errors.push("Field foo is missing".into());
        }
        if self.bar.is_none() {
            errors.push("Field bar is missing".into());
        }

        if !errors.is_empty() {
            Err(errors.join("\n"))
        } else {
            Ok(Foobar {
                foo: self.foo.take().unwrap(),
                bar: self.bar.take().unwrap(),
            })
        }
    }
}

// test
#[test]
fn builder_build_ok() {
    let built = Foobar::builder()
        .foo(2)
        .bar(String::from("foobar"))
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
        }),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .bar(String::from("foobar"))
        .build();
    assert_eq!(
        Err(String::from("Field foo is missing")),
        built,
    );
}
