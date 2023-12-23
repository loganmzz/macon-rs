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

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        <FoobarBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
// struct_builder / struct_builder_named
pub struct FoobarBuilder<FOO=(), BAR=(), OPTION=()> {
    foo: FOO,
    bar: BAR,
    option: Option<String>,
    __optional_set: ::core::marker::PhantomData<OPTION>,
}

// default_builder
// default_builder / default_builder_named
impl ::core::default::Default for FoobarBuilder {
    fn default() -> Self {
        Self {
            foo: ::core::default::Default::default(),
            bar: ::core::default::Default::default(),
            option: ::core::default::Default::default(),
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder
// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_named
impl<BAR,OPTION,> FoobarBuilder<(),BAR,OPTION,> {
    pub fn foo<FOO: ::core::convert::Into<u8>>(self, foo: FOO) -> FoobarBuilder<u8,BAR,OPTION,> {
        FoobarBuilder {
            foo: foo.into(),
            bar: self.bar,
            option: self.option,
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_named
impl<FOO,OPTION,> FoobarBuilder<FOO,(),OPTION,> {
    pub fn bar<BAR: ::core::convert::Into<String>>(self, bar: BAR) -> FoobarBuilder<FOO,String,OPTION,> {
        FoobarBuilder {
            foo: self.foo,
            bar: bar.into(),
            option: self.option,
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_named
impl<FOO,BAR,> FoobarBuilder<FOO,BAR,(),> {
    pub fn option<OPTION: ::core::convert::Into<String>>(self, option: OPTION) -> FoobarBuilder<FOO,BAR,Option<String>,> {
        FoobarBuilder {
            foo: self.foo,
            bar: self.bar,
            option: option.into().into(),
            __optional_set: ::core::default::Default::default(),
        }
    }

    pub fn option_none(self) -> FoobarBuilder<FOO,BAR,Option<String>,> {
        FoobarBuilder {
            foo: self.foo,
            bar: self.bar,
            option: ::core::option::Option::None,
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder
// impl_builder / impl_builder_build
// impl_builder / impl_builder_build / impl_builder_build_named
impl<OPTION> FoobarBuilder<u8,String,OPTION> {
    pub fn build(self) -> Foobar {
        Foobar {
            foo: self.foo,
            bar: self.bar,
            option: self.option,
        }
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl<OPTION> ::core::convert::From<FoobarBuilder<u8,String,OPTION,>> for Foobar {
    fn from(builder: FoobarBuilder<u8,String,OPTION,>) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let built = Foobar::builder()
        .foo(2)
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
    let built = Foobar::builder()
        .foo(2)
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
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .option("into_option")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: Some(String::from("into_option")),
        },
        built,
    );
}

#[test]
fn builder_into_partial() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: None,
        },
        built,
    );
}
