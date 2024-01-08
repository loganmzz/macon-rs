// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(PartialEq,Debug)]
struct Named {
    mandatory: PathBuf,
    option: Option<PathBuf>,
}

#[derive(PartialEq,Debug)]
struct Tuple(
    PathBuf,
    Option<PathBuf>,
);

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl Named {
    pub fn builder() -> NamedBuilder {
        <NamedBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct NamedBuilder<MANDATORY=(),OPTION=(),> {
    mandatory: MANDATORY,
    option: Option<PathBuf>,
    __typestate_markers: ::core::marker::PhantomData<(OPTION,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<OPTION,> NamedBuilder<(),OPTION,> {

    pub fn mandatory<MANDATORY: ::core::convert::Into<PathBuf>>(self, mandatory: MANDATORY) -> NamedBuilder<PathBuf,OPTION,> {
        NamedBuilder {
            mandatory: mandatory.into(),
            option: self.option,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}
// impl_builder / impl_builder_setter
impl<MANDATORY,> NamedBuilder<MANDATORY,(),> {

    pub fn option<OPTION: ::core::convert::Into<PathBuf>>(self, option: OPTION) -> NamedBuilder<MANDATORY,Option<PathBuf>,> {
        NamedBuilder {
            mandatory: self.mandatory,
            option: ::core::option::Option::Some(option.into()),
            __typestate_markers: ::core::default::Default::default(),
        }
    }

    pub fn option_none(self) -> NamedBuilder<MANDATORY,Option<PathBuf>,> {
        NamedBuilder {
            mandatory: self.mandatory,
            option: ::core::option::Option::None,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_build
impl<OPTION,> NamedBuilder<PathBuf,OPTION,> {
    pub fn build(self) -> Named {
        Named {
            mandatory: self.mandatory,
            option: self.option,
        }
    }
}

// impl_builder / impl_builder_from
impl<OPTION,> ::core::convert::From<NamedBuilder<PathBuf,OPTION,>> for Named {
    fn from(builder: NamedBuilder<PathBuf,OPTION,>) -> Self {
        builder.build()
    }
}

// impl_target
impl Tuple {
    pub fn builder() -> TupleBuilder {
        <TupleBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct TupleBuilder<V0=(),V1=(),>(
    V0,
    Option<PathBuf>,
    ::core::marker::PhantomData<(V1,)>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,> TupleBuilder<(),V1,> {
    pub fn set0<V0: ::core::convert::Into<PathBuf>>(self, v0: V0) -> TupleBuilder<PathBuf,V1,> {
        TupleBuilder(
            v0.into(),
            self.1,
            ::core::default::Default::default(),
        )
    }
}
impl TupleBuilder<(),()> {
    pub fn set<V0: ::core::convert::Into<PathBuf>>(self, v0: V0) -> TupleBuilder<PathBuf,(),> {
        self.set0(v0)
    }
}
// impl_builder / impl_builder_setter
impl<V0,> TupleBuilder<V0,(),> {

    pub fn set1<V1: ::core::convert::Into<PathBuf>>(self, v1: V1) -> TupleBuilder<V0,Option<PathBuf>,> {
        TupleBuilder(
            self.0,
            ::core::option::Option::Some(v1.into()),
            ::core::default::Default::default(),
        )
    }

    pub fn set1_none(self) -> TupleBuilder<V0,Option<PathBuf>,> {
        TupleBuilder(
            self.0,
            ::core::option::Option::None,
            ::core::default::Default::default(),
        )
    }
}
impl TupleBuilder<PathBuf,()> {
    pub fn set<V1: ::core::convert::Into<PathBuf>>(self, v1: V1) -> TupleBuilder<PathBuf,Option<PathBuf>,> {
        self.set1(v1)
    }
    pub fn none(self) -> TupleBuilder<PathBuf,Option<PathBuf>,> {
        self.set1_none()
    }
}

// impl_builder / impl_builder_build
impl<V1,> TupleBuilder<PathBuf,V1,> {
    pub fn build(self) -> Tuple {
        Tuple(
            self.0,
            self.1,
        )
    }
}

// impl_builder / impl_builder_from
impl<V1,> ::core::convert::From<TupleBuilder<PathBuf,V1,>> for Tuple {
    fn from(builder: TupleBuilder<PathBuf,V1,>) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_builder_build_full() {
    let built = Named::builder()
        .option("/tmp/builder_build_full/option")
        .mandatory("/tmp/builder_build_full/mandatory")
        .build();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_build_full/mandatory"),
            option: Some(PathBuf::from("/tmp/builder_build_full/option")),
        },
        built,
    );
}

#[test]
fn named_builder_build_partial_implicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_build_partial_implicit/mandatory")
        .build();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_build_partial_implicit/mandatory"),
            option: None,
        },
        built,
    );
}

#[test]
fn named_builder_build_partial_explicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_build_partial_explicit/mandatory")
        .option_none()
        .build();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_build_partial_explicit/mandatory"),
            option: None,
        },
        built,
    );
}
#[test]
fn named_builder_into_full() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_full/mandatory")
        .option("/tmp/builder_into_full/option")
        .into();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_into_full/mandatory"),
            option: Some(PathBuf::from("/tmp/builder_into_full/option")),
        },
        built,
    );
}

#[test]
fn named_builder_into_partial_implicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_partial_implicit/mandatory")
        .into();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_into_partial_implicit/mandatory"),
            option: None,
        },
        built,
    );
}

#[test]
fn named_builder_into_partial_explicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_partial_explicit/mandatory")
        .option_none()
        .into();
    assert_eq!(
        Named {
            mandatory: PathBuf::from("/tmp/builder_into_partial_explicit/mandatory"),
            option: None,
        },
        built,
    );
}

#[test]
fn tuple_builder_build_unordered_full() {
    let built = Tuple::builder()
        .set1("/tmp/builder_build_full/option")
        .set0("/tmp/builder_build_full/mandatory")
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_full/mandatory"),
            Some(PathBuf::from("/tmp/builder_build_full/option")),
        ),
        built,
    );
}

#[test]
fn tuple_builder_build_unordered_partial_implicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_build_partial_implicit/mandatory")
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_partial_implicit/mandatory"),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_builder_build_unordered_partial_explicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_build_partial_explicit/mandatory")
        .set1_none()
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_partial_explicit/mandatory"),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_builder_build_ordered_full() {
    let built = Tuple::builder()
        .set("/tmp/builder_build_full/mandatory")
        .set("/tmp/builder_build_full/option")
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_full/mandatory"),
            Some(PathBuf::from("/tmp/builder_build_full/option")),
        ),
        built,
    );
}

#[test]
fn tuple_builder_build_ordered_partial_implicit() {
    let built = Tuple::builder()
        .set("/tmp/builder_build_partial_implicit/mandatory")
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_partial_implicit/mandatory"),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_builder_build_ordered_partial_explicit() {
    let built = Tuple::builder()
        .set("/tmp/builder_build_partial_explicit/mandatory")
        .none()
        .build();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_build_partial_explicit/mandatory"),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_builder_into_full() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_full/mandatory")
        .set1("/tmp/builder_into_full/option")
        .into();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_into_full/mandatory"),
            Some(PathBuf::from("/tmp/builder_into_full/option")),
        ),
        built,
    );
}

#[test]
fn tuple_builder_into_partial_implicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_partial_implicit/mandatory")
        .into();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_into_partial_implicit/mandatory"),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_builder_into_partial_explicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_partial_explicit/mandatory")
        .set1_none()
        .into();
    assert_eq!(
        Tuple(
            PathBuf::from("/tmp/builder_into_partial_explicit/mandatory"),
            None,
        ),
        built,
    );
}
