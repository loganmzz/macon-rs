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
// struct_builder / struct_builder_tuple
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
    // impl_builder_setters / impl_builder_setters_tuple
    pub fn set0<V0: Into<u8>>(mut self, value: V0) -> Self {
        self.0 = value.into().into();
        self
    }
    pub fn set1<V1: Into<String>>(mut self, value: V1) -> Self {
        self.1 = value.into().into();
        self
    }
    pub fn set2<V2: Into<String>>(mut self, value: V2) -> Self {
        self.2 = value.into().into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_tuple
    pub fn build(self) -> Result<Foobar, String> {
        let mut errors: Vec<String> = vec![];

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
fn builder_build_partial_ok() {
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
fn builder_into() {
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
