// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(PartialEq,Debug)]
struct DefaultStructNamed {
    list: Vec<PathBuf>,
    optional: Option<String>,
    mandatory: PathBuf,
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
            optional: Some("optional".into()),
            mandatory: "/mandatory".into(),
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
      ::core::default::Default::default()
  }
}

// struct_builder
#[derive(Default,)]
struct DefaultStructNamedBuilder<LIST=(),OPTIONAL=(),MANDATORY=(),> {
  list: ::macon::Extending<::macon::Keeping<::macon::Defaulting<Vec<PathBuf>>>,PathBuf,>,
  optional: ::macon::Keeping<::macon::Defaulting<Option<String>>>,
  mandatory: ::macon::Keeping<PathBuf>,
  __typestate_markers: ::core::marker::PhantomData<(LIST,OPTIONAL,MANDATORY,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<OPTIONAL,MANDATORY,> DefaultStructNamedBuilder<(),OPTIONAL,MANDATORY,> {
    pub fn list<LIST: ::core::convert::Into<Vec<PathBuf>>>(self, list: LIST) -> DefaultStructNamedBuilder<Vec<PathBuf>,OPTIONAL,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list.with_value(::macon::Keeping::Set(::macon::Defaulting::Set(list.into()))),
        optional: self.optional,
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
    pub fn list_keep(self) -> DefaultStructNamedBuilder<Vec<PathBuf>,OPTIONAL,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list.with_value(::macon::Keeping::Keep),
        optional: self.optional,
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
    pub fn list_default(self) -> DefaultStructNamedBuilder<Vec<PathBuf>,OPTIONAL,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list.with_value(::macon::Keeping::Set(::macon::Defaulting::Default)),
        optional: self.optional,
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
}

impl<LIST,OPTIONAL,MANDATORY,> DefaultStructNamedBuilder<LIST,OPTIONAL,MANDATORY,> {
  pub fn list_extend<LISTITEM: ::core::convert::Into<PathBuf>, LISTITEMS: ::std::iter::IntoIterator<Item = LISTITEM>>(mut self, list: LISTITEMS) -> DefaultStructNamedBuilder<Vec<PathBuf>,OPTIONAL,MANDATORY,> {
    self.list.extend(::core::iter::IntoIterator::into_iter(list).map(::core::convert::Into::into));
    DefaultStructNamedBuilder {
      list: self.list,
      optional: self.optional,
      mandatory: self.mandatory,
      __typestate_markers: ::core::default::Default::default(),
    }
  }
}

// impl_builder / impl_builder_setter
impl<LIST,MANDATORY,> DefaultStructNamedBuilder<LIST,(),MANDATORY,> {
    pub fn optional<OPTIONAL: ::core::convert::Into<String>>(self, optional: OPTIONAL) -> DefaultStructNamedBuilder<LIST,Option<String>,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list,
        optional: ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::Some(optional.into()))),
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
    pub fn optional_none(self) -> DefaultStructNamedBuilder<LIST,Option<String>,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list,
        optional: ::macon::Keeping::Set(::macon::Defaulting::Set(::core::option::Option::None)),
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
    pub fn optional_keep(self) -> DefaultStructNamedBuilder<LIST,Option<String>,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list,
        optional: ::macon::Keeping::Keep,
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
    pub fn optional_default(self) -> DefaultStructNamedBuilder<LIST,Option<String>,MANDATORY,> {
      DefaultStructNamedBuilder {
        list: self.list,
        optional: ::macon::Keeping::Set(::macon::Defaulting::Default),
        mandatory: self.mandatory,
        __typestate_markers: ::core::default::Default::default(),
      }
    }
}

// impl_builder / impl_builder_setter
impl<LIST,OPTIONAL,> DefaultStructNamedBuilder<LIST,OPTIONAL,(),> {
    pub fn mandatory<MANDATORY: ::core::convert::Into<PathBuf>>(self, mandatory: MANDATORY) -> DefaultStructNamedBuilder<LIST,OPTIONAL,PathBuf,> {
      DefaultStructNamedBuilder {
        list: self.list,
        optional: self.optional,
        mandatory: ::macon::Keeping::Set(mandatory.into()),
        __typestate_markers: ::core::default::Default::default(),
      }
    }
}

// impl_builder / impl_builder_build
impl<LIST,OPTIONAL,MANDATORY,> DefaultStructNamedBuilder<LIST,OPTIONAL,MANDATORY,> {
  pub fn build(self) -> DefaultStructNamed {
    DefaultStructNamed {
      list: self.list.unwrap_with(|list| list.unwrap().unwrap()),
      optional: self.optional.unwrap().unwrap(),
      mandatory: self.mandatory.unwrap(),
    }
  }
}

// impl_builder / impl_builder_from
impl<LIST,OPTIONAL,> ::core::convert::From<DefaultStructNamedBuilder<LIST,OPTIONAL,PathBuf,>> for DefaultStructNamed {
    fn from(builder: DefaultStructNamedBuilder<LIST,OPTIONAL,PathBuf,>) -> Self {
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
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/mandatory"),
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
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/mandatory"),
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
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/mandatory"),
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
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/mandatory"),
        },
        built,
    );
}
