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
struct StructNamedBuilder {
    value: ::macon::Keeping<::macon::Defaulting<String>>,
    optional: ::macon::Keeping<::macon::Defaulting<Option<String>>>,
}

// impl_builder
impl StructNamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn value<VALUE: ::core::convert::Into<String>>(mut self, value: VALUE) -> StructNamedBuilder {
        self.value = ::macon::Keeping::Set(::macon::Defaulting::Set(value.into()));
        self
    }
    pub fn value_keep(mut self) -> StructNamedBuilder {
        self.value = ::macon::Keeping::Keep;
        self
    }
    pub fn value_default(mut self) -> StructNamedBuilder {
        self.value = ::macon::Keeping::Set(::macon::Defaulting::Default);
        self
    }

    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(mut self, optional: OPTIONAL) -> StructNamedBuilder {
       self.optional = ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::Some(optional.into())));
       self
    }
    pub fn optional_none(mut self) -> StructNamedBuilder {
        self.optional = ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::None));
        self
    }
    pub fn optional_keep(mut self) -> StructNamedBuilder {
        self.optional = ::macon::Keeping::Keep;
        self
    }
    pub fn optional_default(mut self) -> StructNamedBuilder {
        self.optional = ::macon::Keeping::Set(::macon::Defaulting::Default);
        self
    }

    // impl_builder / impl_builder_build
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
impl ::core::convert::From<StructNamedBuilder> for StructNamed {
    fn from(builder: StructNamedBuilder) -> Self {
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
struct StructTupleBuilder(
    ::macon::Keeping<::macon::Defaulting<String>>,
    ::macon::Keeping<::macon::Defaulting<Option<String>>>,
);

// impl_builder
impl StructTupleBuilder {
    // impl_builder / impl_builder_setters
    pub fn set0<V0: ::core::convert::Into<String>>(mut self, v0: V0) -> StructTupleBuilder {
        self.0 = ::macon::Keeping::Set(::macon::Defaulting::Set(v0.into()));
        self
    }
    pub fn set0_keep(mut self) -> StructTupleBuilder {
        self.0 = ::macon::Keeping::Keep;
        self
    }
    pub fn set0_default(mut self) -> StructTupleBuilder {
        self.0 = ::macon::Keeping::Set(::macon::Defaulting::Default);
        self
    }

    pub fn set1<V1: ::core::convert::Into<String>>(mut self, v1: V1) -> StructTupleBuilder {
        self.1 = ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::Some(v1.into())));
        self
    }
    pub fn set1_none(mut self) -> StructTupleBuilder {
        self.1 = ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::None));
        self
    }
    pub fn set1_keep(mut self) -> StructTupleBuilder {
        self.1 = ::macon::Keeping::Keep;
        self
    }
    pub fn set1_default(mut self) -> StructTupleBuilder {
        self.1 = ::macon::Keeping::Set(::macon::Defaulting::Default);
        self
    }

    // impl_builder / impl_builder_build
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
impl ::core::convert::From<StructTupleBuilder> for StructTuple {
    fn from(builder: StructTupleBuilder) -> Self {
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
