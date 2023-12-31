// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug,Default)]
struct StructUnit;

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
impl StructUnit {
    pub fn builder() -> StructUnitBuilder {
        ::core::default::Default::default()
    }
}

// struct_builder
#[derive(Default)]
struct StructUnitBuilder {
    __optional_set: ::core::marker::PhantomData<()>,
}

// impl_builder
// impl_builder / impl_builder_build
impl StructUnitBuilder {
    pub fn build(self) -> StructUnit {
        let built = <StructUnit as ::core::default::Default>::default();
        built
    }
}

// impl_target
impl StructNamed {
    pub fn builder() -> StructNamedBuilder {
        ::core::default::Default::default()
    }
}

// struct_builder
#[derive(Default)]
struct StructNamedBuilder<VALUE=(),OPTIONAL=(),> {
    value: ::core::option::Option<String>,
    optional: ::core::option::Option<Option<String>>,
    __optional_set: ::core::marker::PhantomData<(VALUE,OPTIONAL,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<OPTIONAL,> StructNamedBuilder<(),OPTIONAL,> {
    pub fn value<VALUE: ::core::convert::Into<String>>(self, value: VALUE) -> StructNamedBuilder<String,OPTIONAL,> {
        StructNamedBuilder {
            value: ::core::option::Option::Some(value.into()),
            optional: self.optional,
            __optional_set: ::core::default::Default::default(),
        }
    }
    pub fn value_keep(self) -> StructNamedBuilder<String,OPTIONAL,> {
        StructNamedBuilder {
            value: ::core::option::Option::None,
            optional: self.optional,
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_setter
impl<VALUE,> StructNamedBuilder<VALUE,(),> {
    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(self, optional: OPTIONAL) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::core::option::Option::Some(::core::option::Option::Some(optional.into())),
            __optional_set: ::core::default::Default::default(),
        }
    }
    pub fn optional_none(self) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::core::option::Option::Some(::core::option::Option::None),
            __optional_set: ::core::default::Default::default(),
        }
    }
    pub fn optional_keep(self) -> StructNamedBuilder<VALUE,Option<String>,> {
        StructNamedBuilder {
            value: self.value,
            optional: ::core::option::Option::None,
            __optional_set: ::core::default::Default::default(),
        }
    }
}

// impl_builder
// impl_builder / impl_builder_build
impl<VALUE,OPTIONAL,> StructNamedBuilder<VALUE,OPTIONAL,> {
    pub fn build(self) -> StructNamed {
        let mut built = <StructNamed as ::core::default::Default>::default();

        if let ::core::option::Option::Some(value) = self.value {
            built.value = value;
        }

        if let ::core::option::Option::Some(optional) = self.optional {
            built.optional = optional;
        }

        built
    }
}

// impl_builder
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
#[derive(Default)]
struct StructTupleBuilder<V0=(),V1=(),>(
    ::core::option::Option<String>,
    ::core::option::Option<Option<String>>,
    ::core::marker::PhantomData<(V0,V1,)>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,> StructTupleBuilder<(),V1,> {
    pub fn set0<V0: ::core::convert::Into<String>>(self, v0: V0) -> StructTupleBuilder<String,V1,> {
        StructTupleBuilder(
            ::core::option::Option::Some(v0.into()),
            self.1,
            ::core::default::Default::default(),
        )
    }
    pub fn set0_keep(self) -> StructTupleBuilder<String,V1,> {
        StructTupleBuilder(
            ::core::option::Option::None,
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
}

// impl_builder / impl_builder_setter
impl<V0,> StructTupleBuilder<V0,(),> {
    pub fn set1<V1: ::core::convert::Into<String>>(self, v1: V1) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::core::option::Option::Some(::core::option::Option::Some(v1.into())),
            ::core::default::Default::default(),
        )
    }
    pub fn set1_none(self) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::core::option::Option::Some(::core::option::Option::None),
            ::core::default::Default::default(),
        )
    }
    pub fn set1_keep(self) -> StructTupleBuilder<V0,Option<String>,> {
        StructTupleBuilder(
            self.0,
            ::core::option::Option::None,
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
}

// impl_builder
// impl_builder / impl_builder_build
impl<V0,V1,> StructTupleBuilder<V0,V1,> {
    pub fn build(self) -> StructTuple {
        let mut built = <StructTuple as ::core::default::Default>::default();

        if let ::core::option::Option::Some(v0) = self.0 {
            built.0 = v0;
        }

        if let ::core::option::Option::Some(v1) = self.1 {
            built.1 = v1;
        }

        built
    }
}

// impl_builder
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
fn unit_build() {
    let built = StructUnit::builder()
        .build();
    assert_eq!(
        StructUnit,
        built,
    );
}

#[test]
fn named_build_default_implicit() {
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
fn named_build_default_explicit() {
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
fn named_build_default_explicit_none() {
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
fn tuple_build_default_implicit() {
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
fn tuple_build_unordered_default_explicit() {
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
fn tuple_build_unordered_default_explicit_none() {
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
fn tuple_build_ordered_default_explicit() {
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
fn tuple_build_ordered_default_explicit_none() {
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
