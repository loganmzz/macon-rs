use anyhow::Context;
use regex;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashSet, env, fs, hash::Hash, ops::Deref, path::{self, PathBuf}, sync::OnceLock};
use syn::{Path, Type, TypePath, TypeReference};
use crate::common::Setting;

fn load_crate_config() -> anyhow::Result<Option<CrateConfiguration>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    if let Some(config_path) = ["yaml", "yml", "json"]
        .into_iter()
        .map(|ext| path::Path::new(&manifest_dir).join(&format!("macon-config.{}", ext)))
        .find(|path| path.is_file())
    {
        println!("Load Crate configuration from {:?}", config_path);
        read_create_config(&config_path)
    } else {
        Ok(None)
    }
}

fn read_create_config(config_path: &PathBuf) -> anyhow::Result<Option<CrateConfiguration>> {
    let config_file = fs::File::open(config_path)?;
    Ok(Some(serde_yaml::from_reader(config_file)?))
}

pub fn get() -> &'static anyhow::Result<Configuration> {
    static CONFIGURATION: OnceLock<anyhow::Result<Configuration>> = OnceLock::new();
    CONFIGURATION.get_or_init(|| Configuration::load())
}

#[derive(Debug)]
pub struct Configuration {
    default_types: TypeSet,
    option_types: TypeSet,
    settingsets: Vec<SettingSetConfiguration>,
}

#[derive(Debug, Default)]
pub struct TypeSet {
    pathes: HashSet<String>,
}

impl TypeSet {
    fn create<F: FnOnce() -> TypeSet>(config: TypeSetConfiguration, default_fn: F) -> Self {
        let mut this = if config.defaults {
            default_fn()
        } else {
            Self::default()
        };
        for path in config.includes {
            this.pathes.insert(path);
        }
        for path in config.excludes {
            this.pathes.remove(&path);
        }
        this
    }

    pub fn add_path(mut self, path: &str) -> Self {
        self.pathes.insert(path.to_string());
        if path.contains("::") {
            if let Some(last_entry) = path.split("::").last() {
                self.pathes.insert(last_entry.to_string());
            }
        }
        self
    }

    pub fn match_type(&self, ty: &Type) -> bool {
        match ty {
            Type::Path(TypePath { qself: None, path }) => self.match_path(path),
            Type::Reference(TypeReference { elem, .. }) => self.match_type(elem.deref()),
            _ => false,
        }
    }

    pub fn match_path(&self, path: &Path) -> bool {
        let str = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        self.match_str(&str)
    }

    pub fn match_str(&self, str: &str) -> bool {
        self.pathes.contains(str)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let default_types = Configuration::default_default_types();
        let option_types = Configuration::default_option_types();
        let settingsets = Default::default();
        Self {
            default_types,
            option_types,
            settingsets,
        }
    }
}

impl Configuration {
    pub fn load() -> anyhow::Result<Self> {
        let crate_config = load_crate_config()
            .with_context(|| format!("Unable to load Macon crate configuration"))?;
        let that = crate_config.map(Self::create).unwrap_or_default();
        Ok(that)
    }

    pub fn read(config_path: &PathBuf) -> anyhow::Result<Self> {
        read_create_config(config_path)?
            .map(Self::create)
            .with_context(|| format!("No config found at {:?}", config_path))
    }

    fn create(crate_config: CrateConfiguration) -> Self {
        Self {
            default_types: TypeSet::create(crate_config.default_types, Self::default_default_types),
            option_types: TypeSet::create(crate_config.option_types, Self::default_option_types),
            settingsets: crate_config.settingsets,
        }
    }

    pub fn default_default_types() -> TypeSet {
        TypeSet::default()
            .add_path("bool")
            .add_path("char")
            .add_path("f32")
            .add_path("f64")
            .add_path("i8")
            .add_path("i16")
            .add_path("i32")
            .add_path("i64")
            .add_path("i128")
            .add_path("isize")
            .add_path("str")
            .add_path("u8")
            .add_path("u16")
            .add_path("u32")
            .add_path("u64")
            .add_path("u128")
            .add_path("usize")
            .add_path("std::string::String")
            .add_path("core::option::Option")
            .add_path("std::option::Option")
            .add_path("std::vec::Vec")
            .add_path("alloc::vec::Vec")
            .add_path("std::collections::HashMap")
            .add_path("std::collections::hash_map::HashMap")
            .add_path("std::collections::HashSet")
            .add_path("std::collections::hash_set::HashSet")
    }

    pub fn default_option_types() -> TypeSet {
        TypeSet::default()
            .add_path("std::option::Option")
            .add_path("core::option::Option")
    }

    pub fn default_types(&self) -> &TypeSet {
        &self.default_types
    }
    pub fn option_types(&self) -> &TypeSet {
        &self.option_types
    }

    pub fn resolve_settings(
        &self,
        container: MatchingStruct,
        field: Option<MatchingField>,
    ) -> SettingSetValues {
        let mut result = SettingSetValues::default();
        for settingset in self.settingsets.iter() {
            if settingset.is_match(&container, &field) {
                result = result.overrides_with(settingset.settings.clone())
            }
        }
        result
    }
}

/// Crate configuration loaded from `macon-config.<yaml|yaml|json>`.
///
/// It defines default/option type set, and setting sets.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CrateConfiguration {
    /// Configuration version. Ignored.
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
    /// Type sets for [`Default`].
    #[serde(default)]
    pub default_types: TypeSetConfiguration,
    /// Type sets for [`Option`].
    #[serde(default)]
    pub option_types: TypeSetConfiguration,
    /// Setting sets
    #[serde(default)]
    pub settingsets: Vec<SettingSetConfiguration>,
}

/// Type sets that should be included/excluded
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TypeSetConfiguration {
    /// Specify if default type sets must be used.
    #[serde(default = "TypeSetConfiguration::default_defaults")]
    pub defaults: bool,
    /// Type pathes that should be included
    #[serde(default)]
    pub includes: Vec<String>,
    /// Type pathes that should be excluded (from default ones)
    #[serde(default)]
    pub excludes: Vec<String>,
}

/// Setting set with an `id` (for debug purpose) defined by criteria and settings to apply.
#[derive(Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct SettingSetConfiguration {
    /// Identifier (printed in debug mode)
    #[serde(default)]
    pub id: String,
    /// Conditions. If it begins by:
    ///
    /// * [`includes`](SettingSetCriterionIX::Includes), none is included by default,
    /// * [`excludes`](SettingSetCriterionIX::Excludes), everything is included by default.
    #[serde(default)]
    pub criteria: Vec<SettingSetCriterionIXWrapper>,
    /// Settings.
    #[serde(default)]
    pub settings: SettingSetValues,
}

/// Wrapper around [`SettingSetCriterionIX`] to use [`serde_yaml::with::singleton_map`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
struct SettingSetCriterionIXWrapper(
    // Wrapped.
    #[serde(with = "serde_yaml::with::singleton_map")] pub SettingSetCriterionIX,
);
/// Exclusion/Inclusion conditions.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
enum SettingSetCriterionIX {
    // Include if match.
    Includes(SettingSetCriterion),
    // Exclude if match.
    Excludes(SettingSetCriterion),
}

/// Criteria to match.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SettingSetCriterion {
    /// structs & fields can have key sets. It matches against the combine set of struct & field ones.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<SetFilterWrapper>,
    /// it matches against keys defined at struct level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_keys: Option<SetFilterWrapper>,
    /// it matches against keys defined at field level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_keys: Option<SetFilterWrapper>,
    /// it matches against the struct name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub struct_name: Option<StringFilterWrapper>,
    /// it matches against the field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<StringFilterWrapper>,
    /// canonical string representation of field type tokens. As Rust macros only operate on full text / token, it can't match on full/resolved type pathes; only as text as found in source code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<StringFilterWrapper>,
}

/// Wrapper around [`SetFilter`] to use [`serde_yaml::with::singleton_map`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
struct SetFilterWrapper(
    // Wrapped.
    #[serde(with = "serde_yaml::with::singleton_map")] pub SetFilter,
);
/// Filter applicable to [`String`] sets.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
enum SetFilter {
    /// set of keys that must be strictly specified.
    Equals(Vec<String>),
    /// sub-set of keys that must be specified.
    Contains(Vec<String>),
}
/// Wrapper around [`StringFilter`] to use [`serde_yaml::with::singleton_map`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(transparent)]
struct StringFilterWrapper(
    /// Wrapped.
    #[serde(with = "serde_yaml::with::singleton_map")]
    pub StringFilter,
);
/// Filter applicable to [`String`].
#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "lowercase")]
enum StringFilter {
    /// string value that must equal.
    Equals(String),
    /// regular expression that must [match](regex::Regex::is_match).
    Matches(String),
}

/// Settings to apply when match.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SettingSetValues {
    /// struct settings.
    #[serde(default, rename = "struct")]
    pub struct_: SettingSetStructValues,
    /// field settings.
    #[serde(default)]
    pub field: SettingSetFieldValues,
}

/// Struct settings.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "PascalCase")]
pub struct SettingSetStructValues {
    /// boolean indicating if struct derives Default. See [`Default` struct](macon).
    #[serde(skip_serializing_if = "Setting::is_undefined")]
    pub default: Setting<()>,
}

/// Field settings.
#[derive(Deserialize)]
#[derive(Clone, Debug, Default, PartialEq)]
#[serde(default, deny_unknown_fields, rename_all = "PascalCase")]
pub struct SettingSetFieldValues {
    /// * `false` or `"!"` to disable [`Option`] support. See [`Option` fields](macon#option-fields).
    /// * `<any string>` to enforce [`Option`] support. See [`Option` fields](macon#option-fields).
    #[serde(skip_serializing_if = "Setting::is_undefined")]
    pub option: Setting<String>,
    /// boolean indicating if field derives [`Default`]. See [`Default` fields](macon#default-fields).
    #[serde(skip_serializing_if = "Setting::is_undefined")]
    pub default: Setting<()>,
    /// Only `false` or `"!"` is supported. Disable [`Into`] for field setter. See [`Into` argument](macon#into-argument).
    #[serde(skip_serializing_if = "Setting::is_undefined")]
    pub into: Setting<()>,
}

/// Describe struct to compute settings from setting sets.
#[derive(Clone)]
pub struct MatchingStruct<'a> {
    // Struct keys.
    keys: HashSet<&'a str>,
    // Struct name.
    name: &'a str,
}
/// Describe field to compute settings from setting sets.
#[derive(Clone)]
pub struct MatchingField<'a> {
    // Field keys.
    keys: HashSet<&'a str>,
    // Field name.
    name: &'a str,
    // Canonical string representation of field type tokens. As Rust macros only operate on full text / token, it can't match on full/resolved type pathes; only as text as found in source code.
    type_str: &'a str,
}

impl<'a> MatchingStruct<'a> {
    pub fn new(name: &'a  str) -> Self {
        Self {
            keys: HashSet::new(),
            name,
        }
    }
}

impl<'a> MatchingField<'a> {
    pub fn new(name: &'a  str, type_str: &'a  str) -> Self {
        Self {
            keys: HashSet::new(),
            name,
            type_str,
        }
    }
}

impl TypeSetConfiguration {
    fn default_defaults() -> bool {
        true
    }
}

impl Default for TypeSetConfiguration {
    fn default() -> Self {
        Self {
            defaults: Self::default_defaults(),
            includes: Default::default(),
            excludes: Default::default(),
        }
    }
}

impl SettingSetConfiguration {
    fn is_match(&self, container: &MatchingStruct, field: &Option<MatchingField>) -> bool {
        for criteria in self.criteria.iter() {
            if ! criteria.is_match(container, field) {
                return false;
            }
        }
        true
    }
}

impl Deref for SettingSetCriterionIXWrapper {
    type Target = SettingSetCriterionIX;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SettingSetCriterionIX {
    fn is_match(&self, container: &MatchingStruct, field: &Option<MatchingField>) -> bool {
        match self {
            Self::Includes(ref criterion) => criterion.is_match(container, field),
            Self::Excludes(ref criterion) => !criterion.is_match(container, field),
        }
    }
}

impl SettingSetCriterion {
    pub fn includes(criterion: SettingSetCriterion) -> SettingSetCriterionIXWrapper {
        SettingSetCriterionIXWrapper(SettingSetCriterionIX::Includes(criterion))
    }
    pub fn excludes(criterion: SettingSetCriterion) -> SettingSetCriterionIXWrapper {
        SettingSetCriterionIXWrapper(SettingSetCriterionIX::Excludes(criterion))
    }

    fn is_match(&self, container: &MatchingStruct, field: &Option<MatchingField>) -> bool {
        // First match simpler ones
        if let Some(ref expected) = self.struct_name {
            if ! expected.is_match(container.name) {
                return false;
            }
        }

        if let Some(ref expected) = self.field_name {
            if let Some(field) = field {
                if ! expected.is_match(field.name) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref expected) = self.field_type {
            if let Some(field) = field {
                if ! expected.is_match(field.type_str) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref expected) = self.struct_keys {
            if ! expected.is_match(&container.keys) {
                return false;
            }
        }

        if let Some(ref expected)= self.field_keys {
            if let Some(field) = field {
                if ! expected.is_match(&field.keys) {
                    return false;
                }
            } else {
                return false;
            }
        }

        if let Some(ref expected) = self.keys {
            let mut keys = Cow::Borrowed(&container.keys);
            if let Some(field) = field {
                let merge_keys: HashSet<&str> = container.keys.union(&field.keys).cloned().collect();
                keys = Cow::Owned(merge_keys);
            }
            if ! expected.is_match(keys.as_ref()) {
                return false;
            }
        }

        true
    }
}

impl Deref for SetFilterWrapper {
    type Target = SetFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SetFilter {
    pub fn equals<S: ToString>(values: Vec<S>) -> Option<SetFilterWrapper> {
        Some(SetFilterWrapper(SetFilter::Equals(
            values.into_iter().map(|s| s.to_string()).collect(),
        )))
    }
    pub fn contains<S: ToString>(values: Vec<S>) -> Option<SetFilterWrapper> {
        Some(SetFilterWrapper(SetFilter::Contains(
            values.into_iter().map(|s| s.to_string()).collect(),
        )))
    }

    fn is_match(&self, values: &HashSet<&str>) -> bool {
        match self {
            Self::Equals(expected) => {
                let expected: HashSet<&str> = expected.iter().map(String::as_str).collect();
                &expected == values
            },
            Self::Contains(expected) => {
                let expected: HashSet<&str> = expected.iter().map(String::as_str).collect();
                expected.is_subset(values)
            },
        }
    }
}

impl Deref for StringFilterWrapper {
    type Target = StringFilter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StringFilter {
    pub fn equals<S: ToString>(value: S) -> Option<StringFilterWrapper> {
        Some(StringFilterWrapper(StringFilter::Equals(value.to_string())))
    }
    pub fn matches<S: ToString>(value: S) -> Option<StringFilterWrapper> {
        Some(StringFilterWrapper(StringFilter::Matches(
            value.to_string(),
        )))
    }

    fn is_match(&self, value: &str) -> bool {
        match self {
            Self::Equals(expected) => expected == value,
            Self::Matches(expected) => {
                let re = regex::Regex::new(expected).unwrap();
                re.is_match(value)
            }
        }
    }
}

impl SettingSetValues {
    pub fn overrides_with(mut self, overrides: Self) -> Self {
        self.struct_ = self.struct_.overrides_with(overrides.struct_);
        self.field = self.field.overrides_with(overrides.field);
        self
    }
}

impl SettingSetStructValues {
    pub fn overrides_with(mut self, overrides: Self) -> Self {
        self.default = overrides.default.or(self.default);
        self
    }
}

impl SettingSetFieldValues {
    pub fn overrides_with(mut self, overrides: Self) -> Self {
        self.option = overrides.option.or(self.option);
        self.default = overrides.default.or(self.default);
        self.into = overrides.into.or(self.into);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{
        SetFilter, SettingSetConfiguration, SettingSetCriterion, SettingSetCriterionIXWrapper,
        StringFilter,
    };
    use serde_yaml;
    use syn::{
        parse_quote,
        Type,
    };

    use super::*;

    macro_rules! assert_defaults {
        (!$($tt:tt)*) => {{
            let ty: Type = parse_quote!($($tt)*);
            assert!(
                !Configuration::default().default_types().match_type(&ty),
                "Default type: {:?}",
                stringify!($($tt)*)
            );
        }};
        ($($tt:tt)*) => {{
            let ty: Type = parse_quote!($($tt)*);
            assert!(
                Configuration::default().default_types().match_type(&ty),
                "Not default type: {:?}",
                stringify!($($tt)*)
            );
        }};
    }

    #[test]
    fn default_types_str() {
        assert_defaults!(str)
    }

    #[test]
    fn default_types_str_ref() {
        assert_defaults!(&str)
    }

    #[test]
    fn default_types_str_ref_lifetime() {
        assert_defaults!(&'a str)
    }

    #[test]
    fn default_types_str_ref_static() {
        assert_defaults!(&'static str)
    }

    #[test]
    fn default_types_option_short() {
        assert_defaults!(Option)
    }

    #[test]
    fn default_types_option_core() {
        assert_defaults!(::core::option::Option)
    }

    #[test]
    fn default_types_option_std() {
        assert_defaults!(std::option::Option)
    }

    #[test]
    fn default_types_random() {
        assert_defaults!(!Random)
    }

    #[test]
    fn parse_settingssets_empty() {
        let config: CrateConfiguration =
            crate::test::data::load_yaml("config/parse", "settingssets_empty");
        assert_eq!(0, config.settingsets.len(), "settingssets.len",);
    }

    #[test]
    fn parse_criterionix() {
        let instance =
            SettingSetCriterionIXWrapper(SettingSetCriterionIX::Includes(Default::default()));
        let format = serde_yaml::to_string(&instance).unwrap();
        assert_eq!(
            r#"
includes: {}
"#
            .trim_start(),
            format,
        );

        let input = r#"
includes: {}
"#;
        let parsed: SettingSetCriterionIXWrapper = serde_yaml::from_str(input).unwrap();
        assert_eq!(
            SettingSetCriterionIXWrapper(SettingSetCriterionIX::Includes(SettingSetCriterion {
                keys: None,
                struct_keys: None,
                field_keys: None,
                struct_name: None,
                field_name: None,
                field_type: None,
            })),
            parsed,
        );
    }

    #[test]
    fn parse_settingssets_default() {
        let config: CrateConfiguration =
            crate::test::data::load_yaml("config/parse", "settingssets_default");

        let mut settingsets = config.settingsets.iter();
        let settingset = settingsets.next();
        assert!(settingset.is_some(), "settingsets[0]",);
        let settingset = settingset.unwrap();
        assert_eq!("", settingset.id, "settingsets[0].id",);
        let mut criteria = settingset.criteria.iter();
        let criterionix = criteria.next();
        assert_eq!(None, criterionix, "settingsets[0].criteria[0]",);
        assert_eq!(
            Setting::undefined(), settingset.settings.struct_.default,
            "settingsets[0].settings.struct.Default",
        );
        assert_eq!(
            Setting::undefined(), settingset.settings.field.option,
            "settingsets[0].settings.field.Option",
        );
        assert_eq!(
            Setting::undefined(), settingset.settings.field.default,
            "settingsets[0].settings.field.Default",
        );
        assert_eq!(
            Setting::undefined(), settingset.settings.field.into,
            "settingsets[0].settings.field.Into",
        );

        let settingset = settingsets.next();
        assert_eq!(None, settingset, "settingsets[1]",);
    }

    #[test]
    fn parse_settingssets_demo() {
        let config: CrateConfiguration =
            crate::test::data::load_yaml("config/parse", "settingssets_demo");

        assert_eq!(
            vec![
                SettingSetConfiguration {
                    id: "foobar".to_owned(),
                    criteria: vec![SettingSetCriterion::includes(SettingSetCriterion {
                        keys: SetFilter::equals(vec!["foo", "bar",]),
                        struct_keys: SetFilter::equals(vec!["foo", "bar",]),
                        field_keys: SetFilter::equals(vec!["foo", "bar",]),
                        struct_name: StringFilter::equals("Foobar"),
                        field_name: StringFilter::equals("foobar"),
                        field_type: StringFilter::equals("Foobar"),
                    }),],
                    settings: SettingSetValues {
                        struct_: SettingSetStructValues {
                            default: Setting::enable(()),
                        },
                        field: SettingSetFieldValues {
                            option: "false".try_into().unwrap(),
                            default: true.into(),
                            into: false.into(),
                        },
                    },
                },
                SettingSetConfiguration {
                    id: "".to_owned(),
                    criteria: vec![SettingSetCriterion::excludes(SettingSetCriterion {
                        keys: SetFilter::contains(vec!["foo", "bar",]),
                        struct_keys: SetFilter::contains(vec!["foo", "bar",]),
                        field_keys: SetFilter::contains(vec!["foo", "bar",]),
                        struct_name: StringFilter::matches("Foobar"),
                        field_name: StringFilter::matches("foobar"),
                        field_type: StringFilter::matches("Foobar"),
                    }),],
                    settings: SettingSetValues {
                        struct_: SettingSetStructValues {
                            default: false.into(),
                        },
                        field: SettingSetFieldValues {
                            option: "!".try_into().unwrap(),
                            default: false.into(),
                            into: false.into(),
                        },
                    },
                },
                SettingSetConfiguration {
                    id: "".to_owned(),
                    criteria: vec![],
                    settings: SettingSetValues {
                        struct_: SettingSetStructValues { default: Setting::undefined() },
                        field: SettingSetFieldValues {
                            option: "bool".try_into().unwrap(),
                            default: Setting::undefined(),
                            into: Setting::undefined(),
                        },
                    },
                },
            ],
            config.settingsets,
        );
    }

    #[test]
    fn settingset_overrides_empty_empty() {
        let base = SettingSetValues {
            struct_: SettingSetStructValues { default: Setting::undefined() },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        let overrides = SettingSetValues {
            struct_: SettingSetStructValues { default: Setting::undefined() },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        assert_eq!(
            SettingSetValues {
                struct_: SettingSetStructValues { default: Setting::undefined() },
                field: SettingSetFieldValues {
                    option: Setting::undefined(),
                    default: Setting::undefined(),
                    into: Setting::undefined(),
                },
            },
            base.overrides_with(overrides),
        )
    }

    #[test]
    fn settingset_overrides_empty_struct_default() {
        let base = SettingSetValues {
            struct_: SettingSetStructValues { default: Setting::undefined() },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        let overrides = SettingSetValues {
            struct_: SettingSetStructValues {
                default: true.into(),
            },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        assert_eq!(
            SettingSetValues {
                struct_: SettingSetStructValues {
                    default: true.into(),
                },
                field: SettingSetFieldValues {
                    option: Setting::undefined(),
                    default: Setting::undefined(),
                    into: Setting::undefined(),
                },
            },
            base.overrides_with(overrides),
        )
    }

    #[test]
    fn settingset_overrides_struct_not_default_struct_default() {
        let base = SettingSetValues {
            struct_: SettingSetStructValues {
                default: false.into(),
            },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        let overrides = SettingSetValues {
            struct_: SettingSetStructValues {
                default: true.into(),
            },
            field: SettingSetFieldValues {
                option: Setting::undefined(),
                default: Setting::undefined(),
                into: Setting::undefined(),
            },
        };
        assert_eq!(
            SettingSetValues {
                struct_: SettingSetStructValues {
                    default: true.into(),
                },
                field: SettingSetFieldValues {
                    option: Setting::undefined(),
                    default: Setting::undefined(),
                    into: Setting::undefined(),
                },
            },
            base.overrides_with(overrides),
        )
    }

    #[test]
    fn crate_configuration_resolve() {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ResolveExpectation {
            #[serde(alias = "Foo")]
            foo: SettingSetValues,
            #[serde(alias = "Foo.foo")]
            foo_foo: SettingSetValues,
            #[serde(alias = "Foo.bar")]
            foo_bar: SettingSetValues,
            #[serde(alias = "Foo.boo")]
            foo_boo: SettingSetValues,
            #[serde(alias = "Bar")]
            bar: SettingSetValues,
            #[serde(alias = "Bar.foo")]
            bar_foo: SettingSetValues,
        }

        let cases = vec![
            // 00 - Empty
            "00_empty",
            // 01
            // - Include any
            "01_include_any",
            // 02
            // - Include Foo
            "02_include_Foo",
            // 03
            // - Include *.foo
            "03_include_foo",
            // 04
            // - Include /oo/
            "04_include_oo",
            // 05
            // - Include Foo.foo
            "05_include_Foo_foo",
            // 06
            // - Include type usize
            "06_include_type_usize",
            // 07
            // - Include type /^Str/
            "07_include_type_Str",
            // 08
            // - Include struct key exact no match
            "08_include_struct_key_exact_no_match",
            // 09
            // - Include struct key exact match
            "09_include_struct_key_exact_match",
            // 10
            // - Include struct key contains
            "10_include_struct_key_contains",
            // 11
            // - Include field key exact no match
            "11_include_field_key_exact_no_match",
            // 12
            // - Include field key exact match
            "12_include_field_key_exact_match",
            // 13
            // - Include struct key contains
            "13_include_field_key_contains",
            // 14
            // - Include all key exact no match
            "14_include_all_key_exact_no_match",
            // 15
            // - Include all key exact match
            "15_include_all_key_exact_match",
            // 16
            // - Include all key contains
            "16_include_all_key_contains",
            // 17
            // - Include Foo or foo
            "17_include_Foo_or_foo",
            // 18
            // - Include override struct Default
            "18_include_override_struct_Default",
            // 19
            // - Include override field Option
            "19_include_override_field_Option",
            // 20
            // - Include override field Default
            "20_include_override_field_Default",
            // 21
            // - Include override field Into
            "21_include_override_field_Into",
        ];

        for case in cases {
            let config_path = crate::test::data::path("config/crate_configuration_resolve", &format!("{}.config.yaml", case));
            let config = Configuration::read(&config_path)
                .with_context(|| format!("can't load config for case {} (path: {:?})", case, config_path))
                .unwrap();
            let expecteds: ResolveExpectation = crate::test::data::load_yaml("config/crate_configuration_resolve", &format!("{}.expecteds", case));

            let struct_foo = MatchingStruct {
                keys: vec![
                    "struct",
                    "struct_foo",
                    "foo",
                ].into_iter().collect(),
                name: "Foo",
            };
            let struct_bar = MatchingStruct {
                keys: vec![
                    "struct",
                    "struct_bar",
                    "bar",
                ].into_iter().collect(),
                name: "Bar",
            };
            let field_foo = Some(MatchingField {
                keys: vec![
                    "field",
                    "field_foo",
                    "foo",
                    "usize",
                ].into_iter().collect(),
                name: "foo",
                type_str: "usize",
            });
            let field_bar = Some(MatchingField {
                keys: vec![
                    "field",
                    "field_bar",
                    "bar",
                    "string",
                ].into_iter().collect(),
                name: "bar",
                type_str: "String",
            });
            let field_boo = Some(MatchingField {
                keys: vec![
                    "field",
                    "field_boo",
                    "boo",
                    "string",
                ].into_iter().collect(),
                name: "boo",
                type_str: "String",
            });

            assert_eq!(
                expecteds.foo,
                config.resolve_settings(struct_foo.clone(), None),
                "{}: Foo", case,
            );
            assert_eq!(
                expecteds.foo_foo,
                config.resolve_settings(struct_foo.clone(), field_foo.clone()),
                "{}: Foo.foo", case,
            );
            assert_eq!(
                expecteds.foo_bar,
                config.resolve_settings(struct_foo.clone(), field_bar.clone()),
                "{}: Foo.bar", case,
            );
            assert_eq!(
                expecteds.foo_boo,
                config.resolve_settings(struct_foo.clone(), field_boo.clone()),
                "{}: Foo.boo", case,
            );
            assert_eq!(
                expecteds.bar,
                config.resolve_settings(struct_bar.clone(), None),
                "{}: Bar", case,
            );
            assert_eq!(
                expecteds.bar_foo,
                config.resolve_settings(struct_bar.clone(), field_foo.clone()),
                "{}: Bar.foo", case,
            );
        }
    }
}
