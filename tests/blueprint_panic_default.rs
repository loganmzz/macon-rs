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

// struct_builder
#[derive(Default)]
struct StructUnitBuilder;

// impl_target
impl StructUnit {
    pub fn builder() -> StructUnitBuilder {
        <StructUnitBuilder as ::core::default::Default>::default()
    }
}

// impl_builder
impl StructUnitBuilder {
    // impl_builder_build
    pub fn build(self) -> StructUnit {
        let built = <StructUnit as ::core::default::Default>::default();

        built
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl ::core::convert::From<StructUnitBuilder> for StructUnit {
    fn from(builder: StructUnitBuilder) -> Self {
        builder.build()
    }
}

// struct_builder
#[derive(Default)]
struct StructNamedBuilder {
    value: ::core::option::Option<String>,
    optional: ::core::option::Option<Option<String>>,
}

// impl_target
impl StructNamed {
    pub fn builder() -> StructNamedBuilder {
        <StructNamedBuilder as ::core::default::Default>::default()
    }
}

// impl_builder
impl StructNamedBuilder {
    // impl_builder_setters
    pub fn value<VALUE: ::core::convert::Into<String>>(mut self, value: VALUE) -> Self {
        self.value = ::core::option::Option::Some(value.into());
        self
    }

    pub fn value_keep(mut self) -> Self {
        self.value = ::core::option::Option::None;
        self
    }

    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(mut self, optional: OPTIONAL) -> Self {
        self.optional = ::core::option::Option::Some(::core::option::Option::Some(optional.into()));
        self
    }

    pub fn optional_none(mut self) -> Self {
        self.optional = ::core::option::Option::Some(::core::option::Option::None);
        self
    }

    pub fn optional_keep(mut self) -> Self {
        self.optional = ::core::option::Option::None;
        self
    }

    // impl_builder_build
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
impl ::core::convert::From<StructNamedBuilder> for StructNamed {
    fn from(builder: StructNamedBuilder) -> Self {
        builder.build()
    }
}

// struct_builder
#[derive(Default)]
struct StructTupleBuilder(
    ::core::option::Option<String>,
    ::core::option::Option<Option<String>>,
);

// impl_target
impl StructTuple {
    pub fn builder() -> StructTupleBuilder {
        <StructTupleBuilder as ::core::default::Default>::default()
    }
}

// impl_builder
impl StructTupleBuilder {
    // impl_builder_setters
    pub fn set0<V0: ::core::convert::Into<String>>(mut self, v0: V0) -> Self {
        self.0 = ::core::option::Option::Some(v0.into());
        self
    }

    pub fn set0_keep(mut self) -> Self {
        self.0 = ::core::option::Option::None;
        self
    }

    pub fn set1<V1: ::core::convert::Into<String>>(mut self, v1: V1) -> Self {
        self.1 = ::core::option::Option::Some(::core::option::Option::Some(v1.into()));
        self
    }

    pub fn set1_none(mut self) -> Self {
        self.1 = ::core::option::Option::Some(::core::option::Option::None);
        self
    }

    pub fn set1_keep(mut self) -> Self {
        self.1 = ::core::option::Option::None;
        self
    }


    // impl_builder_build
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
impl ::core::convert::From<StructTupleBuilder> for StructTuple {
    fn from(builder: StructTupleBuilder) -> Self {
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
