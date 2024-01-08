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
struct FoobarBuilder<V0=(),V1=(),>(
    V0,
    V1,
    ::core::marker::PhantomData<()>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,> FoobarBuilder<(),V1,> {
    pub fn set0<V0: ::core::convert::Into<PathBuf>>(self, v0: V0) -> FoobarBuilder<PathBuf,V1,> {
        FoobarBuilder(
            v0.into(),
            self.1,
            ::core::default::Default::default(),
        )
    }
}

impl FoobarBuilder<(),(),> {
    pub fn set<V0: ::core::convert::Into<PathBuf>>(self, v0: V0) -> FoobarBuilder<PathBuf,(),> {
        self.set0(v0)
    }
}

// impl_builder / impl_builder_setter
impl<V0,> FoobarBuilder<V0,(),> {
    pub fn set1<V1: ::core::convert::Into<PathBuf>>(self, v1: V1) -> FoobarBuilder<V0,PathBuf,> {
        FoobarBuilder(
            self.0,
            v1.into(),
            ::core::default::Default::default(),
        )
    }
}

impl FoobarBuilder<PathBuf,(),> {
    pub fn set<V1: ::core::convert::Into<PathBuf>>(self, v1: V1) -> FoobarBuilder<PathBuf,PathBuf,> {
        self.set1(v1)
    }
}

// impl_builder / impl_builder_build
impl<> FoobarBuilder<PathBuf,PathBuf,> {
    pub fn build(self) -> Foobar {
        Foobar(
            self.0,
            self.1,
        )
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
fn builder_build_unordered() {
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
fn builder_build_ordered() {
    let built = Foobar::builder()
        .set("/tmp/builder_build.0")
        .set("/tmp/builder_build.1")
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
fn builder_into() {
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
