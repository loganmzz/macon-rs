// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use ::std::path::PathBuf;

#[derive(PartialEq,Debug,)]
struct StructNamed {
    id: i32,
    value: String,
    optional: Option<String>,
    mandatory: PathBuf,
}

#[derive(PartialEq,Debug,)]
struct StructTuple(
    i32,
    String,
    Option<String>,
    PathBuf,
);

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl StructNamed {
    pub fn builder() -> StructNamedBuilder {
        <StructNamedBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct StructNamedBuilder<ID=(),VALUE=(),OPTIONAL=(),MANDATORY=(),> {
    id: ::macon::Defaulting<i32>,
    value: ::macon::Defaulting<String>,
    optional: ::macon::Defaulting<Option<String>>,
    mandatory: MANDATORY,
    __typestate_markers: ::core::marker::PhantomData<(ID,VALUE,OPTIONAL,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<VALUE,OPTIONAL,MANDATORY,> StructNamedBuilder<(),VALUE,OPTIONAL,MANDATORY,> {
    pub fn id<ID: ::core::convert::Into<i32>>(self, id: ID) -> StructNamedBuilder<i32,VALUE,OPTIONAL,MANDATORY,> {
        StructNamedBuilder {
            id: ::macon::Defaulting::Set(id.into()),
            value: self.value,
            optional: self.optional,
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }

    pub fn id_default(self) -> StructNamedBuilder<i32,VALUE,OPTIONAL,MANDATORY,> {
        StructNamedBuilder {
            id: ::macon::Defaulting::Default,
            value: self.value,
            optional: self.optional,
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}
// impl_builder / impl_builder_setter
impl<ID,OPTIONAL,MANDATORY,> StructNamedBuilder<ID,(),OPTIONAL,MANDATORY,> {
    pub fn value<VALUE: ::core::convert::Into<String>>(self, value: VALUE) -> StructNamedBuilder<ID,String,OPTIONAL,MANDATORY,> {
        StructNamedBuilder {
            id: self.id,
            value: ::macon::Defaulting::Set(value.into()),
            optional: self.optional,
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }

    pub fn value_default(self) -> StructNamedBuilder<ID,String,OPTIONAL,MANDATORY,> {
        StructNamedBuilder {
            id: self.id,
            value: ::macon::Defaulting::Default,
            optional: self.optional,
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}
// impl_builder / impl_builder_setter
impl<ID,VALUE,MANDATORY,> StructNamedBuilder<ID,VALUE,(),MANDATORY,> {
    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(self, optional: OPTIONAL) -> StructNamedBuilder<ID,VALUE,Option<String>,MANDATORY,> {
        StructNamedBuilder {
            id: self.id,
            value: self.value,
            optional: ::macon::Defaulting::Set(::core::option::Option::Some(optional.into())),
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn optional_none(self) -> StructNamedBuilder<ID,VALUE,Option<String>,MANDATORY,> {
        StructNamedBuilder {
            id: self.id,
            value: self.value,
            optional: ::macon::Defaulting::Set(::core::option::Option::None),
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn optional_default(self) -> StructNamedBuilder<ID,VALUE,Option<String>,MANDATORY,> {
        StructNamedBuilder {
            id: self.id,
            value: self.value,
            optional: ::macon::Defaulting::Default,
            mandatory: self.mandatory,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_setter
impl<ID,VALUE,OPTIONAL,> StructNamedBuilder<ID,VALUE,OPTIONAL,(),> {
    pub fn mandatory<MANDATORY: ::core::convert::Into<PathBuf>>(self, mandatory: MANDATORY) -> StructNamedBuilder<ID,VALUE,OPTIONAL,PathBuf,> {
        StructNamedBuilder {
            id: self.id,
            value: self.value,
            optional: self.optional,
            mandatory: mandatory.into(),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_build
impl<ID,VALUE,OPTIONAL,> StructNamedBuilder<ID,VALUE,OPTIONAL,PathBuf,> {
    pub fn build(self) -> StructNamed {
        StructNamed {
            id: self.id.unwrap(),
            value: self.value.unwrap(),
            optional: self.optional.unwrap(),
            mandatory: self.mandatory,
        }
    }
}

// impl_builder / impl_builder_from
impl<ID,VALUE,OPTIONAL,> ::core::convert::From<StructNamedBuilder<ID,VALUE,OPTIONAL,PathBuf,>> for StructNamed {
    fn from(builder: StructNamedBuilder<ID,VALUE,OPTIONAL,PathBuf,>) -> Self {
        builder.build()
    }
}

// impl_target
impl StructTuple {
    pub fn builder() -> StructTupleBuilder {
        <StructTupleBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct StructTupleBuilder<V0=(),V1=(),V2=(),V3=(),>(
    ::macon::Defaulting<i32>,
    ::macon::Defaulting<String>,
    ::macon::Defaulting<Option<String>>,
    V3,
    ::core::marker::PhantomData<(V0,V1,V2,)>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,V2,V3,> StructTupleBuilder<(),V1,V2,V3,> {
    pub fn set0<V0: ::core::convert::Into<i32>>(self, v0: V0) -> StructTupleBuilder<i32,V1,V2,V3,> {
        StructTupleBuilder(
            ::macon::Defaulting::Set(v0.into()),
            self.1,
            self.2,
            self.3,
            ::core::default::Default::default(),
        )
    }
    pub fn set0_default(self) -> StructTupleBuilder<i32,V1,V2,V3,> {
        StructTupleBuilder(
            ::macon::Defaulting::Default,
            self.1,
            self.2,
            self.3,
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<(),(),(),()> {
    pub fn set<V0: ::core::convert::Into<i32>>(self, v0: V0) -> StructTupleBuilder<i32,(),(),()> {
        self.set0(v0)
    }
    pub fn default(self) -> StructTupleBuilder<i32,(),(),()> {
        self.set0_default()
    }
}

// impl_builder / impl_builder_setter
impl<V0,V2,V3,> StructTupleBuilder<V0,(),V2,V3,> {
    pub fn set1<V1: ::core::convert::Into<String>>(self, v1: V1) -> StructTupleBuilder<V0,String,V2,V3,> {
        StructTupleBuilder(
            self.0,
            ::macon::Defaulting::Set(v1.into()),
            self.2,
            self.3,
            ::core::default::Default::default(),
        )
    }
    pub fn set1_default(self) -> StructTupleBuilder<V0,String,V2,V3,> {
        StructTupleBuilder(
            self.0,
            ::macon::Defaulting::Default,
            self.2,
            self.3,
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<i32,(),(),(),> {
    pub fn set<V1: ::core::convert::Into<String>>(self, v1: V1) -> StructTupleBuilder<i32,String,(),(),> {
        self.set1(v1)
    }
    pub fn default(self) -> StructTupleBuilder<i32,String,(),(),> {
        self.set1_default()
    }
}

// impl_builder / impl_builder_setter
impl<V0,V1,V3,> StructTupleBuilder<V0,V1,(),V3,> {
    pub fn set2<V2: ::core::convert::Into<String>>(self, v2: V2) -> StructTupleBuilder<V0,V1,Option<String>,V3,> {
        StructTupleBuilder(
            self.0,
            self.1,
            ::macon::Defaulting::Set(::core::option::Option::Some(v2.into())),
            self.3,
            ::core::default::Default::default(),
        )
    }
    pub fn set2_none(self) -> StructTupleBuilder<V0,V1,Option<String>,V3,> {
        StructTupleBuilder(
            self.0,
            self.1,
            ::macon::Defaulting::Set(::core::option::Option::None),
            self.3,
            ::core::default::Default::default(),
        )
    }
    pub fn set2_default(self) -> StructTupleBuilder<V0,V1,Option<String>,V3,> {
        StructTupleBuilder(
            self.0,
            self.1,
            ::macon::Defaulting::Default,
            self.3,
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<i32,String,(),(),> {
    pub fn set<V2: ::core::convert::Into<String>>(self, v2: V2) -> StructTupleBuilder<i32,String,Option<String>,(),> {
        self.set2(v2)
    }
    pub fn none(self) -> StructTupleBuilder<i32,String,Option<String>,(),> {
        self.set2_none()
    }
    pub fn default(self) -> StructTupleBuilder<i32,String,Option<String>,(),> {
        self.set2_default()
    }
}

// impl_builder / impl_builder_setter
impl<V0,V1,V2,> StructTupleBuilder<V0,V1,V2,(),> {
    pub fn set3<V3: ::core::convert::Into<PathBuf>>(self, v3: V3) -> StructTupleBuilder<V0,V1,V2,PathBuf,> {
        StructTupleBuilder(
            self.0,
            self.1,
            self.2,
            v3.into(),
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<i32,String,Option<String>,(),> {
    pub fn set<V3: ::core::convert::Into<PathBuf>>(self, v3: V3) -> StructTupleBuilder<i32,String,Option<String>,PathBuf,> {
        self.set3(v3)
    }
}

// impl_builder / impl_builder_build
impl<V0,V1,V2,> StructTupleBuilder<V0,V1,V2,PathBuf,> {
    pub fn build(self) -> StructTuple {
        StructTuple(
            self.0.unwrap(),
            self.1.unwrap(),
            self.2.unwrap(),
            self.3,
        )
    }
}

// impl_builder / impl_builder_from
impl<V0,V1,V2,> ::core::convert::From<StructTupleBuilder<V0,V1,V2,PathBuf,>> for StructTuple {
    fn from(builder: StructTupleBuilder<V0,V1,V2,PathBuf,>) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_build_default_implicit() {
    let built = StructNamed::builder()
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn named_build_default_explicit() {
    let built = StructNamed::builder()
        .id_default()
        .value_default()
        .optional_default()
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn named_build_default_explicit_none() {
    let built = StructNamed::builder()
        .id_default()
        .value_default()
        .optional_none()
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn named_build_full() {
    let built = StructNamed::builder()
        .id(42)
        .value("any value")
        .optional("optional")
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 42,
            value: String::from("any value"),
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn tuple_build_default_implicit() {
    let built = StructTuple::builder()
        .set3("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_default_explicit() {
    let built = StructTuple::builder()
        .set0_default()
        .set1_default()
        .set2_default()
        .set3("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_default_explicit_none() {
    let built = StructTuple::builder()
        .set0_default()
        .set1_default()
        .set2_none()
        .set3("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_full() {
    let built = StructTuple::builder()
        .set0(42)
        .set1("any value")
        .set2("optional")
        .set3("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            42,
            String::from("any value"),
            Some(String::from("optional")),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_default_explicit() {
    let built = StructTuple::builder()
        .default()
        .default()
        .default()
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_default_explicit_none() {
    let built = StructTuple::builder()
        .default()
        .default()
        .none()
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_ordered_build_full() {
    let built = StructTuple::builder()
        .set(42)
        .set("any value")
        .set("optional")
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            42,
            String::from("any value"),
            Some(String::from("optional")),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}
