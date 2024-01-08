// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(PartialEq,Debug)]
struct Foobar {
    path1: PathBuf,
    path2: PathBuf,
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
#[derive(Default,)]
struct FoobarBuilder {
    path1: ::macon::Building<PathBuf>,
    path2: ::macon::Building<PathBuf>,
}

// impl_builder
impl FoobarBuilder {
    // impl_builder / impl_builder_setters
    pub fn path1<PATH1: ::core::convert::Into<PathBuf>>(mut self, path1: PATH1) -> FoobarBuilder {
        self.path1 = ::macon::Building::Set(path1.into());
        self
    }

    pub fn path2<PATH2: ::core::convert::Into<PathBuf>>(mut self, path2: PATH2) -> FoobarBuilder {
        self.path2 = ::macon::Building::Set(path2.into());
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> Foobar {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.path1.is_undefined() {
            errors.push("Field path1 is missing".into());
        }
        if self.path2.is_undefined() {
            errors.push("Field path2 is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"));
        } else {
            Foobar {
                path1: self.path1.unwrap(),
                path2: self.path2.unwrap(),
            }
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
        .path1("/tmp/builder_build.001")
        .path2("/tmp/builder_build.002")
        .build();
    assert_eq!(
        Foobar {
            path1: PathBuf::from("/tmp/builder_build.001"),
            path2: PathBuf::from("/tmp/builder_build.002"),
        },
        built,
    );
}

#[test]
#[should_panic(expected = "Field path1 is missing")]
fn builder_build_missing() {
    Foobar::builder()
        .path2("/tmp/builder_build.002")
        .build();
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
    .path1("/tmp/builder_into.001")
    .path2("/tmp/builder_into.002")
        .into();
    assert_eq!(
        Foobar {
            path1: PathBuf::from("/tmp/builder_into.001"),
            path2: PathBuf::from("/tmp/builder_into.002"),
        },
        built,
    );
}

#[test]
#[should_panic(expected = "Field path2 is missing")]
fn builder_into_missing() {
    let _built: Foobar = Foobar::builder()
        .path1("/tmp/builder_into.001")
        .into();
}
