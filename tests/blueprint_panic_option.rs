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
struct NamedBuilder {
    mandatory: ::macon::Building<PathBuf>,
    option: Option<PathBuf>,
}

// impl_builder
impl NamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn mandatory<MANDATORY: ::core::convert::Into<PathBuf>>(mut self, mandatory: MANDATORY) -> NamedBuilder {
        self.mandatory = ::macon::Building::Set(mandatory.into());
        self
    }

    pub fn option_optional<OPTION: ::core::convert::Into<PathBuf>>(mut self, option: ::core::option::Option<OPTION>) -> NamedBuilder {
        self.option = option.map(::core::convert::Into::into);
        self
    }

    pub fn option<OPTION: ::core::convert::Into<PathBuf>>(mut self, option: OPTION) -> NamedBuilder {
        self.option = ::core::option::Option::Some(option.into());
        self
    }
    pub fn option_none(mut self) -> NamedBuilder {
        self.option = ::core::option::Option::None;
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> Named {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.mandatory.is_undefined() {
            errors.push("Field mandatory is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"));
        } else {
            Named {
                mandatory: self.mandatory.unwrap(),
                option: self.option,
            }
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<NamedBuilder> for Named {
    fn from(builder: NamedBuilder) -> Self {
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
struct TupleBuilder(
    ::macon::Building<PathBuf>,
    Option<PathBuf>,
);

// impl_builder
impl TupleBuilder {
    // impl_builder / impl_builder_setters
    pub fn set0<V0: ::core::convert::Into<PathBuf>>(mut self, v0: V0) -> TupleBuilder {
        self.0 = ::macon::Building::Set(v0.into());
        self
    }

    pub fn set1<V1: ::core::convert::Into<PathBuf>>(mut self, v1: V1) -> TupleBuilder {
        self.1 = ::core::option::Option::Some(v1.into());
        self
    }
    pub fn set1_none(mut self) -> TupleBuilder {
        self.1 = ::core::option::Option::None;
        self
    }
    pub fn set1_optional<V1: ::core::convert::Into<PathBuf>>(mut self, v1: ::core::option::Option<V1>) -> TupleBuilder {
        self.1 = v1.map(::core::convert::Into::into);
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> Tuple {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.0.is_undefined() {
            errors.push("Field 0 is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"));
        } else {
            Tuple(
                self.0.unwrap(),
                self.1,
            )
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<TupleBuilder> for Tuple {
    fn from(builder: TupleBuilder) -> Self {
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
fn named_builder_build_full_optional() {
    let built = Named::builder()
        .option_optional(Some("/tmp/builder_build_full/option"))
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
fn named_builder_into_full_optional() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_full/mandatory")
        .option_optional(Some("/tmp/builder_into_full/option"))
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
fn tuple_builder_build_full() {
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
fn tuple_builder_build_full_optional() {
    let built = Tuple::builder()
        .set1_optional(Some("/tmp/builder_build_full/option"))
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
fn tuple_builder_build_partial_implicit() {
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
fn tuple_builder_build_partial_explicit() {
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
fn tuple_builder_into_full_optional() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_full/mandatory")
        .set1_optional(Some("/tmp/builder_into_full/option"))
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
