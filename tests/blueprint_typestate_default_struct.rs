// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug,Default)]
struct StructNamed {
    value: String,
    optional: Option<String>,
}

#[derive(PartialEq,Debug,Default)]
struct StructTuple(
    String,
    Option<String>,
);

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl StructNamed {
    pub fn builder() -> StructNamedBuilder {
        ::core::default::Default::default()
    }
}

// struct_builder
#[derive(Default,)]
struct StructNamedBuilder<VALUE=(),OPTIONAL=(),> {
    value: ::macon::Keeping<::macon::Defaulting<String>>,
    optional: ::macon::Keeping<::macon::Defaulting<Option<String>>>,
    __typestate_markers: ::core::marker::PhantomData<(VALUE,OPTIONAL,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<OPTIONAL,> StructNamedBuilder<(),OPTIONAL,> {
    pub fn value<VALUE: ::core::convert::Into<String>>(self, value: VALUE) -> StructNamedBuilder<String,OPTIONAL,> {
        StructNamedBuilder {
            value: ::macon::Keeping::Set(::macon::Defaulting::Set(value.into())),
            optional: self.optional,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn value_keep(self) -> StructNamedBuilder<String,OPTIONAL,> {
        StructNamedBuilder {
            value: ::macon::Keeping::Keep,
            optional: self.optional,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn value_default(self) -> StructNamedBuilder<String,OPTIONAL,> {
        StructNamedBuilder {
            value: ::macon::Keeping::Set(::macon::Defaulting::Default),
            optional: self.optional,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_setter
impl<VALUE,> StructNamedBuilder<VALUE,(),> {
    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(self, optional: OPTIONAL) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::Some(optional.into()))),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn optional_none(self) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::None)),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn optional_keep(self) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::macon::Keeping::Keep,
            __typestate_markers: ::core::default::Default::default(),
        }
    }
    pub fn optional_default(self) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::macon::Keeping::Set(::macon::Defaulting::Default),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_build
impl<VALUE,OPTIONAL,> StructNamedBuilder<VALUE,OPTIONAL,> {
    pub fn build(self) -> StructNamed {
        let mut built = <StructNamed as ::core::default::Default>::default();
        if self.value.is_set() {
            built.value = self.value.unwrap().unwrap();
        }
        if self.optional.is_set() {
            built.optional = self.optional.unwrap().unwrap();
        }
        built
    }
}

// impl_builder / impl_builder_from
impl<VALUE,OPTIONAL,> ::core::convert::From<StructNamedBuilder<VALUE,OPTIONAL,>> for StructNamed {
    fn from(builder: StructNamedBuilder<VALUE,OPTIONAL,>) -> Self {
        builder.build()
    }
}

// impl_target
impl StructTuple {
    pub fn builder() -> StructTupleBuilder {
        ::core::default::Default::default()
    }
}

// struct_builder
#[derive(Default,)]
struct StructTupleBuilder<V0=(),V1=(),>(
    ::macon::Keeping<::macon::Defaulting<String>>,
    ::macon::Keeping<::macon::Defaulting<Option<String>>>,
    ::core::marker::PhantomData<(V0,V1,)>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,> StructTupleBuilder<(),V1,> {
    pub fn set0<V0: ::core::convert::Into<String>>(self, v0: V0) -> StructTupleBuilder<String,V1,> {
        StructTupleBuilder(
            ::macon::Keeping::Set(::macon::Defaulting::Set(v0.into())),
            self.1,
            ::core::default::Default::default(),
        )
    }
    pub fn set0_keep(self) -> StructTupleBuilder<String,V1,> {
        StructTupleBuilder(
            ::macon::Keeping::Keep,
            self.1,
            ::core::default::Default::default(),
        )
    }
    pub fn set0_default(self) -> StructTupleBuilder<String,V1,> {
        StructTupleBuilder(
            ::macon::Keeping::Set(::macon::Defaulting::Default),
            self.1,
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<(),(),> {
    pub fn set<V0: ::core::convert::Into<String>>(self, v0: V0) -> StructTupleBuilder<String,(),> {
        self.set0(v0)
    }
    pub fn keep(self) -> StructTupleBuilder<String,(),> {
        self.set0_keep()
    }
    pub fn default(self) -> StructTupleBuilder<String,(),> {
        self.set0_default()
    }
}

// impl_builder / impl_builder_setter
impl<V0,> StructTupleBuilder<V0,(),> {
    pub fn set1<V1: ::core::convert::Into<String>>(self, v1: V1) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::Some(v1.into()))),
            ::core::default::Default::default(),
        )
    }
    pub fn set1_none(self) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::None)),
            ::core::default::Default::default(),
        )
    }
    pub fn set1_keep(self) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::macon::Keeping::Keep,
            ::core::default::Default::default(),
        )
    }
    pub fn set1_default(self) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::macon::Keeping::Set(::macon::Defaulting::Default),
            ::core::default::Default::default(),
        )
    }
}

impl StructTupleBuilder<String,(),> {
    pub fn set<V1: ::core::convert::Into<String>>(self, v1: V1) -> StructTupleBuilder<String,Option<String>,> {
        self.set1(v1)
    }
    pub fn none(self) -> StructTupleBuilder<String,Option<String>,> {
        self.set1_none()
    }
    pub fn keep(self) -> StructTupleBuilder<String,Option<String>,> {
        self.set1_keep()
    }
    pub fn default(self) -> StructTupleBuilder<String,Option<String>,> {
        self.set1_default()
    }
}

// impl_builder / impl_builder_build
impl<V0,V1,> StructTupleBuilder<V0,V1,> {
    pub fn build(self) -> StructTuple {
        let mut built = <StructTuple as ::core::default::Default>::default();
        if self.0.is_set() {
            built.0 = self.0.unwrap().unwrap();
        }
        if self.1.is_set() {
            built.1 = self.1.unwrap().unwrap();
        }
        built
    }
}

// impl_builder / impl_builder_from
impl<V0,V1,> ::core::convert::From<StructTupleBuilder<V0,V1,>> for StructTuple {
    fn from(builder: StructTupleBuilder<V0,V1,>) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_build_keep_implicit() {
    let built = StructNamed::builder()
        .build();
    assert_eq!(
        StructNamed {
            value: String::from(""),
            optional: None,
        },
        built,
    )
}

#[test]
fn named_build_keep_explicit() {
    let built = StructNamed::builder()
        .value_keep()
        .optional_keep()
        .build();
    assert_eq!(
        StructNamed {
            value: String::from(""),
            optional: None,
        },
        built,
    )
}

#[test]
fn named_build_keep_explicit_none() {
    let built = StructNamed::builder()
        .value_keep()
        .optional_none()
        .build();
    assert_eq!(
        StructNamed {
            value: String::from(""),
            optional: None,
        },
        built,
    )
}

#[test]
fn named_build_full() {
    let built = StructNamed::builder()
        .value("any value")
        .optional("optional")
        .build();
    assert_eq!(
        StructNamed {
            value: String::from("any value"),
            optional: Some(String::from("optional")),
        },
        built,
    )
}

#[test]
fn named_build_default() {
    let built = StructNamed::builder()
        .value_default()
        .optional_default()
        .build();
    assert_eq!(
        StructNamed {
            value: String::from(""),
            optional: None,
        },
        built,
    )
}

#[test]
fn tuple_build_keep_implicit() {
    let built = StructTuple::builder()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_keep_explicit() {
    let built = StructTuple::builder()
        .set0_keep()
        .set1_keep()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_keep_explicit_none() {
    let built = StructTuple::builder()
        .set0_keep()
        .set1_none()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_full() {
    let built = StructTuple::builder()
        .set0("any value")
        .set1("optional")
        .build();
    assert_eq!(
        StructTuple(
            String::from("any value"),
            Some(String::from("optional")),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_default() {
    let built = StructTuple::builder()
        .set0_default()
        .set1_default()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_keep_explicit() {
    let built = StructTuple::builder()
        .keep()
        .keep()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_keep_explicit_none() {
    let built = StructTuple::builder()
        .keep()
        .none()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}

#[test]
fn tuple_ordered_build_full() {
    let built = StructTuple::builder()
        .set("any value")
        .set("optional")
        .build();
    assert_eq!(
        StructTuple(
            String::from("any value"),
            Some(String::from("optional")),
        ),
        built,
    )
}

#[test]
fn tuple_ordered_build_default() {
    let built = StructTuple::builder()
        .default()
        .default()
        .build();
    assert_eq!(
        StructTuple(
            String::from(""),
            None,
        ),
        built,
    )
}
