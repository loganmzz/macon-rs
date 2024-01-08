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
struct FoobarBuilder<PATH1=(),PATH2=()> {
    path1: PATH1,
    path2: PATH2,
    __typestate_markers: ::core::marker::PhantomData<()>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<PATH2> FoobarBuilder<(),PATH2,> {
    pub fn path1<PATH1: ::core::convert::Into<PathBuf>>(self, path1: PATH1) -> FoobarBuilder<PathBuf,PATH2,> {
        FoobarBuilder {
            path1: path1.into(),
            path2: self.path2,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}
// impl_builder / impl_builder_setter
impl<PATH1> FoobarBuilder<PATH1,(),> {
    pub fn path2<PATH2: ::core::convert::Into<PathBuf>>(self, path2: PATH2) -> FoobarBuilder<PATH1,PathBuf,> {
        FoobarBuilder {
            path1: self.path1,
            path2: path2.into(),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_build
impl<> FoobarBuilder<PathBuf,PathBuf,> {
    pub fn build(self) -> Foobar {
        Foobar {
            path1: self.path1,
            path2: self.path2,
        }
    }
}

// impl_builder / impl_builder_from
impl<> ::core::convert::From<FoobarBuilder<PathBuf,PathBuf,>> for Foobar {
    fn from(builder: FoobarBuilder<PathBuf,PathBuf,>) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build() {
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
fn builder_into() {
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
