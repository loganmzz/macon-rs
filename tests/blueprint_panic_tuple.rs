// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(PartialEq,Debug)]
struct Foobar(
    PathBuf,
    PathBuf,
);

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
#[derive(Default,)]
struct FoobarBuilder(
    ::macon::Building<PathBuf>,
    ::macon::Building<PathBuf>,
);

// impl_builder
impl FoobarBuilder {
    // impl_builder / impl_builder_setters
    pub fn set0<V0: Into<PathBuf>>(mut self, v0: V0) -> Self {
        self.0 = ::macon::Building::Set(v0.into());
        self
    }

    pub fn set1<V1: Into<PathBuf>>(mut self, v1: V1) -> Self {
        self.1 = ::macon::Building::Set(v1.into());
        self
    }

    // impl_builder / impl_builder_build
    // impl_builder / impl_builder_build / impl_builder_build_from_scratch
    pub fn build(self) -> Foobar {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.0.is_undefined() {
            errors.push("Field 0 is missing".into());
        }
        if self.1.is_undefined() {
            errors.push("Field 1 is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"))
        } else {
            Foobar(
                self.0.unwrap(),
                self.1.unwrap(),
            )
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<FoobarBuilder> for Foobar {
    fn from(builder: FoobarBuilder) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let built = Foobar::builder()
        .set1("/tmp/builder_build.1")
        .set0("/tmp/builder_build.0")
        .build();
    assert_eq!(
        Foobar(
            PathBuf::from("/tmp/builder_build.0"),
            PathBuf::from("/tmp/builder_build.1"),
        ),
        built,
    );
}

#[test]
#[should_panic(expected = "Field 0 is missing")]
fn builder_build_missing() {
    Foobar::builder()
        .set1("/tmp/builder_build.002")
        .build();
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .set0("/tmp/builder_into.0")
        .set1("/tmp/builder_into.1")
        .into();
    assert_eq!(
        Foobar(
            PathBuf::from("/tmp/builder_into.0"),
            PathBuf::from("/tmp/builder_into.1"),
        ),
        built,
    );
}

#[test]
#[should_panic(expected = "Field 1 is missing")]
fn builder_into_missing() {
    Foobar::builder()
        .set0("/tmp/builder_into.001")
        .build();
}
