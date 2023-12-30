// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
    option: Option<String>,
}
// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// struct_builder
#[derive(Default)]
pub struct FoobarBuilder {
    foo: Option<u8>,
    bar: Option<String>,
    option: Option<String>,
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
    pub fn foo<FOO: Into<u8>>(mut self, foo: FOO) -> Self {
        self.foo = foo.into().into();
        self
    }

    pub fn bar<BAR: Into<String>>(mut self, bar: BAR) -> Self {
        self.bar = bar.into().into();
        self
    }

    pub fn option<OPTION: Into<String>>(mut self, option: OPTION) -> Self {
        self.option = option.into().into();
        self
    }

    // impl_builder_build
    pub fn build(self) -> Result<Foobar, String> {
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
                foo: self.foo.unwrap(),
                bar: self.bar.unwrap(),
                option: self.option,
            })
        }
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl TryFrom<FoobarBuilder> for Foobar {
    type Error = String;

    fn try_from(builder: FoobarBuilder) -> Result<Self, Self::Error> {
        builder.build()
    }
}


// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full_ok() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .option("optional")
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: Some(String::from("optional")),
        }),
        built,
    );
}

#[test]
fn builder_build_partial_ok() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: None,
        }),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .bar("foobar")
        .build();
    assert_eq!(
        Err(String::from("Field foo is missing")),
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .option("optional")
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: Some(String::from("optional")),
        },
        built,
    );
}
