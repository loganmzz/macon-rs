// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug)]
pub struct Foobar(
    u8,
    String,
    Option<String>,
);

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// struct_builder
#[derive(Default)]
pub struct FoobarBuilder(Option<u8>,Option<String>,Option<String>,);

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        Default::default()
    }
}

// impl_builder
impl FoobarBuilder {
    // impl_builder_setters
    pub fn set0<V0: Into<u8>>(mut self, value: V0) -> Self {
        self.0 = ::core::option::Option::Some(value.into());
        self
    }
    pub fn set1<V1: Into<String>>(mut self, value: V1) -> Self {
        self.1 = ::core::option::Option::Some(value.into());
        self
    }
    pub fn set2<V2: Into<String>>(mut self, value: V2) -> Self {
        self.2 = ::core::option::Option::Some(value.into());
        self
    }
    pub fn set2_none(mut self) -> Self {
        self.2 = ::core::option::Option::None;
        self
    }

    // impl_builder_build
    pub fn build(self) -> ::core::result::Result<Foobar, ::std::string::String> {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.0.is_none() {
            errors.push("Field 0 is missing".into());
        }
        if self.1.is_none() {
            errors.push("Field 1 is missing".into());
        }

        if !errors.is_empty() {
            Err(errors.join("\n"))
        } else {
            Ok(Foobar(
                self.0.unwrap(),
                self.1.unwrap(),
                self.2,
            ))
        }
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl ::core::convert::TryFrom<FoobarBuilder> for Foobar {
    type Error = ::std::string::String;

    fn try_from(builder: FoobarBuilder) -> ::core::result::Result<Self, Self::Error> {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_unordered_full() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
        .set2("optional")
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        )),
        built,
    );
}

#[test]
fn builder_build_unordered_partial_explicit() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
        .set2_none()
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
            None,
        )),
        built,
    );
}

#[test]
fn builder_build_unordered_partial_implicit() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
            None,
        )),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .set1("foobar")
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    );
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .set2("optional")
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            Some(String::from("optional")),
        ),
        built,
    );
}

#[test]
fn builder_into_partial() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            None,
        ),
        built,
    );
}
