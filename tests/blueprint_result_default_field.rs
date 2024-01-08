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
struct StructNamedBuilder {
    id: ::macon::Defaulting<i32>,
    value: ::macon::Defaulting<String>,
    optional: ::macon::Defaulting<Option<String>>,
    mandatory: ::macon::Building<PathBuf>,
}

// impl_builder
impl StructNamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn id<ID: ::core::convert::Into<i32>>(mut self, id: ID) -> StructNamedBuilder {
        self.id = ::macon::Defaulting::Set(id.into());
        self
    }

    pub fn id_default(mut self) -> StructNamedBuilder {
        self.id = ::macon::Defaulting::Default;
        self
    }

    pub fn value<VALUE: ::core::convert::Into<String>>(mut self, value: VALUE) -> StructNamedBuilder {
        self.value = ::macon::Defaulting::Set(value.into());
        self
    }

    pub fn value_default(mut self) -> StructNamedBuilder {
        self.value = ::macon::Defaulting::Default;
        self
    }

    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(mut self, optional: OPTIONAL) -> StructNamedBuilder {
        self.optional = ::macon::Defaulting::Set(::core::option::Option::Some(optional.into()));
        self
    }

    pub fn optional_none(mut self) -> StructNamedBuilder {
        self.optional = ::macon::Defaulting::Set(::core::option::Option::None);
        self
    }

    pub fn optional_default(mut self) -> StructNamedBuilder {
        self.optional = ::macon::Defaulting::Default;
        self
    }

    pub fn mandatory<MANDATORY: ::core::convert::Into<PathBuf>>(mut self, mandatory: MANDATORY) -> StructNamedBuilder {
        self.mandatory = ::macon::Building::Set(mandatory.into());
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> ::core::result::Result<StructNamed,::std::string::String> {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.mandatory.is_undefined() {
            errors.push("Field mandatory is missing".into());
        }

        if !errors.is_empty() {
            ::core::result::Result::Err(errors.join("\n"))
        } else {
            ::core::result::Result::Ok(StructNamed {
                id: self.id.unwrap(),
                value: self.value.unwrap(),
                optional: self.optional.unwrap(),
                mandatory: self.mandatory.unwrap(),
            })
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::TryFrom<StructNamedBuilder> for StructNamed {
    type Error = ::std::string::String;
    fn try_from(builder: StructNamedBuilder) -> ::core::result::Result<Self, Self::Error> {
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
struct StructTupleBuilder(
    ::macon::Defaulting<i32>,
    ::macon::Defaulting<String>,
    ::macon::Defaulting<Option<String>>,
    ::macon::Building<PathBuf>,
);

// impl_builder
impl StructTupleBuilder {
    // impl_builder / impl_builder_setters
    pub fn set0<V0: ::core::convert::Into<i32>>(mut self, v0: V0) -> StructTupleBuilder {
        self.0 = ::macon::Defaulting::Set(v0.into());
        self
    }

    pub fn set0_default(mut self) -> StructTupleBuilder {
        self.0 = ::macon::Defaulting::Default;
        self
    }

    pub fn set1<V1: ::core::convert::Into<String>>(mut self, v1: V1) -> StructTupleBuilder {
        self.1 = ::macon::Defaulting::Set(v1.into());
        self
    }

    pub fn set1_default(mut self) -> StructTupleBuilder {
        self.1 = ::macon::Defaulting::Default;
        self
    }

    pub fn set2<V2: ::core::convert::Into<String>>(mut self, v2: V2) -> StructTupleBuilder {
        self.2 = ::macon::Defaulting::Set(::core::option::Option::Some(v2.into()));
        self
    }

    pub fn set2_none(mut self) -> StructTupleBuilder {
        self.2 = ::macon::Defaulting::Set(::core::option::Option::None);
        self
    }

    pub fn set2_default(mut self) -> StructTupleBuilder {
        self.2 = ::macon::Defaulting::Default;
        self
    }

    pub fn set3<V3: ::core::convert::Into<PathBuf>>(mut self, v3: V3) -> StructTupleBuilder {
        self.3 = ::macon::Building::Set(v3.into());
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> ::core::result::Result<StructTuple,::std::string::String> {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.3.is_undefined() {
            errors.push("Field 3 is missing".into());
        }

        if !errors.is_empty() {
            ::core::result::Result::Err(errors.join("\n"))
        } else {
            ::core::result::Result::Ok(StructTuple(
                self.0.unwrap(),
                self.1.unwrap(),
                self.2.unwrap(),
                self.3.unwrap(),
            ))
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::TryFrom<StructTupleBuilder> for StructTuple {
    type Error = ::std::string::String;
    fn try_from(builder: StructTupleBuilder) -> ::core::result::Result<Self, Self::Error> {
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
        Ok(StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        }),
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
        Ok(StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        }),
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
        Ok(StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        }),
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
        Ok(StructNamed {
            id: 42,
            value: String::from("any value"),
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/dev/null"),
        }),
        built,
    )
}

#[test]
fn tuple_build_default_implicit() {
    let built = StructTuple::builder()
        .set3("/dev/null")
        .build();
    assert_eq!(
        Ok(StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        )),
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
        Ok(StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        )),
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
        Ok(StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        )),
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
        Ok(StructTuple(
            42,
            String::from("any value"),
            Some(String::from("optional")),
            PathBuf::from("/dev/null"),
        )),
        built,
    )
}
