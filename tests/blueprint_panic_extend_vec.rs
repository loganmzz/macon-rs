// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(PartialEq,Debug)]
struct DefaultStructNamed {
    list: Vec<PathBuf>,
}

impl Default for DefaultStructNamed {
    fn default() -> Self {
        Self {
            list: vec![
                "a",
                "b",
                "c",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        }
    }
}

#[derive(PartialEq,Debug)]
struct DefaultFieldNamed {
    list: Vec<PathBuf>,
}

#[derive(PartialEq,Debug)]
struct MandatoryFieldNamed {
    list: Vec<PathBuf>,
}

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl DefaultStructNamed {
    pub fn builder() -> DefaultStructNamedBuilder {
        <DefaultStructNamedBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct DefaultStructNamedBuilder {
    list: ::macon::Extending<::macon::Keeping<::macon::Defaulting<Vec<PathBuf>>>, PathBuf>,
}

// impl_builder
impl DefaultStructNamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn list<LIST: ::core::convert::Into<Vec<PathBuf>>>(mut self, list: LIST) -> Self {
        *self.list.value_mut() = ::macon::Keeping::Set(::macon::Defaulting::Set(::core::convert::Into::into(list)));
        self
    }
    pub fn list_keep(mut self) -> Self {
        *self.list.value_mut() = ::macon::Keeping::Keep;
        self
    }
    pub fn list_default(mut self) -> Self {
        *self.list.value_mut() = ::macon::Keeping::Set(::macon::Defaulting::Default);
        self
    }
    pub fn list_extend<LISTITEM: ::core::convert::Into<PathBuf>, LIST: ::std::iter::IntoIterator<Item = LISTITEM>>(mut self, list: LIST) -> Self {
        self.list.extend(::core::iter::IntoIterator::into_iter(list).map(::core::convert::Into::into));
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> DefaultStructNamed {
        let mut built = <DefaultStructNamed as ::core::default::Default>::default();

        let (list, listitems) = self.list.unwrap();
        if list.is_set() {
            built.list = list.unwrap().unwrap();
        }
        built.list.extend(listitems);

        built
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<DefaultStructNamedBuilder> for DefaultStructNamed {
    fn from(builder: DefaultStructNamedBuilder) -> Self {
        builder.build()
    }
}


// impl_target
impl DefaultFieldNamed {
    pub fn builder() -> DefaultFieldNamedBuilder {
        <DefaultFieldNamedBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct DefaultFieldNamedBuilder {
    list: ::macon::Extending<::macon::Defaulting<Vec<PathBuf>>, PathBuf>,
}

// impl_builder
impl DefaultFieldNamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn list<LIST: ::core::convert::Into<Vec<PathBuf>>>(mut self, list: LIST) -> DefaultFieldNamedBuilder {
        *self.list.value_mut() = ::macon::Defaulting::Set(::core::convert::Into::into(list));
        self
    }
    pub fn list_default(mut self) -> DefaultFieldNamedBuilder {
        *self.list.value_mut() = ::macon::Defaulting::Default;
        self
    }
    pub fn list_extend<LISTITEM: ::core::convert::Into<PathBuf>, LIST: ::std::iter::IntoIterator<Item = LISTITEM>>(mut self, list: LIST) -> DefaultFieldNamedBuilder {
        self.list.extend(::core::iter::IntoIterator::into_iter(list).map(::core::convert::Into::into));
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> DefaultFieldNamed {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"));
        } else {
            DefaultFieldNamed {
                list: {
                    let (list, listitems) = self.list.unwrap();
                    let mut list = list.unwrap();
                    list.extend(listitems);
                    list
                }
            }
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<DefaultFieldNamedBuilder> for DefaultFieldNamed {
    fn from(builder: DefaultFieldNamedBuilder) -> Self {
        builder.build()
    }
}

// impl_target
impl MandatoryFieldNamed {
    pub fn builder() -> MandatoryFieldNamedBuilder {
        <MandatoryFieldNamedBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct MandatoryFieldNamedBuilder {
    list: ::macon::Extending<::macon::Building<Vec<PathBuf>>, PathBuf>,
}

// impl_builder
impl MandatoryFieldNamedBuilder {
    // impl_builder / impl_builder_setters
    pub fn list<LIST: ::core::convert::Into<Vec<PathBuf>>>(mut self, list: LIST) -> MandatoryFieldNamedBuilder {
        *self.list.value_mut() = ::macon::Building::Set(::core::convert::Into::into(list));
        self
    }
    pub fn list_extend<LISTITEM: ::core::convert::Into<PathBuf>, LIST: ::std::iter::IntoIterator<Item = LISTITEM>>(mut self, list: LIST) -> MandatoryFieldNamedBuilder {
        self.list.extend(::core::iter::IntoIterator::into_iter(list).map(::core::convert::Into::into));
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> MandatoryFieldNamed {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.list.is_undefined() {
            errors.push("Field list is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"));
        } else {
            MandatoryFieldNamed {
                list: {
                    let (list, listitems) = self.list.unwrap();
                    let mut list = list.unwrap();
                    list.extend(listitems);
                    list
                }
            }
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::From<MandatoryFieldNamedBuilder> for MandatoryFieldNamed {
    fn from(builder: MandatoryFieldNamedBuilder) -> Self {
        builder.build()
    }
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn defaultstructnamed_builder_struct_default() {
    let built = DefaultStructNamed::builder()
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("a"),
                PathBuf::from("b"),
                PathBuf::from("c"),
            ],
        },
        built,
    );
}

#[test]
fn defaultstructnamed_builder_keep_extend() {
    let built = DefaultStructNamed::builder()
        .list_keep()
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("a"),
                PathBuf::from("b"),
                PathBuf::from("c"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn defaultstructnamed_builder_default_extend() {
    let built = DefaultStructNamed::builder()
        .list_default()
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn defaultstructnamed_builder_set_extend() {
    let built = DefaultStructNamed::builder()
        .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("g"),
                PathBuf::from("h"),
                PathBuf::from("i"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn defaultfieldnamed_builder_field_default() {
    let built = DefaultFieldNamed::builder()
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![],
        },
        built,
    );
}

#[test]
fn defaultfieldnamed_builder_default_extend() {
    let built = DefaultFieldNamed::builder()
        .list_default()
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn defaultfieldnamed_builder_set_extend() {
    let built = DefaultFieldNamed::builder()
        .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![
                PathBuf::from("g"),
                PathBuf::from("h"),
                PathBuf::from("i"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn mandatoryfieldnamed_builder_set_extend() {
    let built = MandatoryFieldNamed::builder()
        .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        MandatoryFieldNamed {
            list: vec![
                PathBuf::from("g"),
                PathBuf::from("h"),
                PathBuf::from("i"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
#[should_panic(expected = "Field list is missing")]
fn mandatoryfieldnamed_builder_missing() {
    let _built = MandatoryFieldNamed::builder()
        .list_extend(&["d", "e", "f",])
        .build();
}
