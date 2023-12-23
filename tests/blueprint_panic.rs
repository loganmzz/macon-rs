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
// struct_builder / struct_builder_named
#[derive(Default)]
pub struct FoobarBuilder {
    foo: ::core::option::Option<u8>,
    bar: ::core::option::Option<String>,
    option: ::core::option::Option<String>,

}

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        <FoobarBuilder as ::core::default::Default>::default()
    }
}

// impl_builder
impl FoobarBuilder {
    // impl_builder_setters
    // impl_builder_setters / impl_builder_setters_named
    pub fn foo<FOO: ::core::convert::Into<u8>>(mut self, foo: FOO) -> Self {
        self.foo = foo.into().into();
        self
    }

    pub fn bar<BAR: ::core::convert::Into<String>>(mut self, bar: BAR) -> Self {
        self.bar = bar.into().into();
        self
    }

    pub fn option<OPTION: ::core::convert::Into<String>>(mut self, option: OPTION) -> Self {
        self.option = option.into().into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_named
    pub fn build(self) -> Foobar {
        let mut errors: ::std::vec::Vec<String> = vec![];

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
                option: self.option,
            }
        }
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl From<FoobarBuilder> for Foobar {
    fn from(builder: FoobarBuilder) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .option("optional")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: Some(String::from("optional")),
        },
        built,
    );
}

#[test]
fn builder_build_partial() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: None,
        },
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .option("optional")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: Some(String::from("optional")),
        },
        built,
    );
}
