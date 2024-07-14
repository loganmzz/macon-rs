use std::{
    collections::HashMap,
    fs,
    env,
    ops::Deref,
    path,
    sync::OnceLock,
};
use anyhow::Context;
use serde::{
    Deserialize,
    Deserializer,
};
use syn::{
    Path,
    Type,
    TypePath,
    TypeReference,
};

fn load_crate_config() -> anyhow::Result<Option<CrateConfiguration>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    if let Some(config_path) = ["yaml", "yml", "json"]
            .into_iter()
            .map(|ext| path::Path::new(&manifest_dir).join(&format!("macon-config.{}", ext)))
            .find(|path| path.is_file()) {
        #[cfg(feature = "debug")]
        eprintln!("Load Crate configuration from {:?}", config_path);
        let config_file = fs::File::open(config_path)?;
        let crate_configuration = serde_yaml::from_reader(config_file)?;
        #[cfg(feature = "debug")]
        eprintln!("{:#?}", crate_configuration);
        Ok(Some(crate_configuration))
    } else {
        Ok(None)
    }
}

pub fn get() -> &'static anyhow::Result<Configuration> {
    static CONFIGURATION: OnceLock<anyhow::Result<Configuration>> = OnceLock::new();
    CONFIGURATION.get_or_init(|| Configuration::load())
}

#[derive(Debug,)]
pub struct Configuration {
    default_types: TypeSet,
    option_types: TypeSet<ContainerTypeSetItem>,
    extend_types: TypeSet<ContainerTypeSetItem>,
}

pub trait TypeSetItem {
    fn path(&self) -> String;
}

impl TypeSetItem for String {
    fn path(&self) -> String {
        self.clone()
    }
}

#[derive(Debug,)]
pub struct TypeSet<V: TypeSetItem = String> {
    pathes: HashMap<String,V>,
}

impl<V: TypeSetItem> Default for TypeSet<V> {
    fn default() -> Self {
        Self {
            pathes: Default::default(),
        }
    }
}

impl<V: TypeSetItem> TypeSet<V> {
    fn create<DefaultConfigurationFn: FnOnce()->TypeSet<V>>(config: TypeSetConfiguration<V>, default_fn: DefaultConfigurationFn) -> Self {
        let mut this = if config.defaults {
            default_fn()
        } else {
            Self::default()
        };
        for item in config.includes {
            this.insert(item);
        }
        for path in config.excludes {
            this.remove(&path);
        }
        this
    }

    pub fn insert(&mut self, item: V) {
        self.pathes.insert(item.path(), item);
    }

    pub fn remove(&mut self, path: &str) {
        self.pathes.remove(path);
    }

    pub fn match_type(&self, ty: &Type) -> Option<&V> {
        match ty {
            Type::Path(TypePath { qself: None, path }) => self.match_path(path),
            Type::Reference(TypeReference { elem, ..}) => self.match_type(elem.deref()),
            _ => None,
        }
    }

    pub fn match_path(&self, path: &Path) -> Option<&V> {
        let str = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");

        self.match_str(&str)
    }

    pub fn match_str(&self, str: &str) -> Option<&V> {
        self.pathes.get(str)
    }
}

impl<'a, V: From<&'a str> + TypeSetItem> TypeSet<V> {
    fn add_path(mut self, path: &'a str) -> Self {
        self.insert(path.into());
        if path.contains("::") {
            if let Some(last_entry) = path.split("::").last() {
                self.insert(last_entry.into());
            }
        }
        self
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let default_types = Configuration::default_default_types();
        let option_types = Configuration::default_option_types();
        let extend_types = Configuration::default_extend_types();
        Self {
            default_types,
            option_types,
            extend_types,
        }
    }
}

impl Configuration {

    pub fn load() -> anyhow::Result<Self> {
        let crate_config = load_crate_config()
            .with_context(|| format!("Unable to load Macon crate configuration"))?;
        let that = crate_config
            .map(Self::create)
            .unwrap_or_default();
        Ok(that)
    }

    fn create(crate_config: CrateConfiguration) -> Self {
        let default_types = TypeSet::create(
            crate_config.default_types,
            Self::default_default_types,
        );
        let option_types = TypeSet::create(
            crate_config.option_types.map_includes(|i| i.into()),
            Self::default_option_types,
        );
        let extend_types = TypeSet::create(
            crate_config.extend_types.map_includes(|i| i.into()),
            Self::default_extend_types,
        );

        let this = Self {
            default_types,
            option_types,
            extend_types,
        };
        #[cfg(feature = "debug")]
        eprintln!("Merge configuration\n{:#?}", this);
        this
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

    pub fn default_option_types() -> TypeSet<ContainerTypeSetItem> {
        TypeSet::default()
            .add_path("std::option::Option")
            .add_path("core::option::Option")
    }

    pub fn default_extend_types() -> TypeSet<ContainerTypeSetItem> {
        TypeSet::default()
            .add_path("std::vec::Vec")
    }

    pub fn default_types(&self) -> &TypeSet {
        &self.default_types
    }
    pub fn option_types(&self) -> &TypeSet<ContainerTypeSetItem> {
        &self.option_types
    }
    pub fn extend_types(&self) -> &TypeSet<ContainerTypeSetItem> {
        &self.extend_types
    }
}

#[derive(Debug,Deserialize,)]
struct CrateConfiguration {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
    #[serde(default)]
    pub default_types: TypeSetConfiguration<String>,
    #[serde(default = "default_typesetconfiguration", deserialize_with="deserialize_crateconfiguration_container_types")]
    pub option_types: TypeSetConfiguration<ContainerTypeSetItem>,
    #[serde(default = "default_typesetconfiguration", deserialize_with="deserialize_crateconfiguration_container_types")]
    pub extend_types: TypeSetConfiguration<ContainerTypeSetItem>,
}

fn deserialize_crateconfiguration_container_types<'de, D>(deserializer: D) -> std::result::Result<TypeSetConfiguration<ContainerTypeSetItem>, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded: TypeSetConfiguration<OptionTypeSetItemConfiguration> = Deserialize::deserialize(deserializer)?;
    Ok(decoded.map_includes(ContainerTypeSetItem::from))
}

#[derive(Debug,Default,Deserialize,PartialEq,)]
pub struct ContainerTypeSetItem {
    pub path: String,
    pub wrapped: Option<String>,
}

impl TypeSetItem for ContainerTypeSetItem {
    fn path(&self) -> String {
        self.path.clone()
    }
}

impl From<&str> for ContainerTypeSetItem {
    fn from(path: &str) -> Self {
        Self {
            path: path.to_owned(),
            wrapped: None,
        }
    }
}

#[derive(Debug,Deserialize,PartialEq,)]
#[serde(untagged)]
pub enum OptionTypeSetItemConfiguration {
    String(String),
    Item(ContainerTypeSetItem),
}

impl Default for OptionTypeSetItemConfiguration {
    fn default() -> Self {
        OptionTypeSetItemConfiguration::String("".to_owned())
    }
}

impl From<OptionTypeSetItemConfiguration> for ContainerTypeSetItem {
    fn from(value: OptionTypeSetItemConfiguration) -> Self {
        match value {
            OptionTypeSetItemConfiguration::String(string) => string.as_str().into(),
            OptionTypeSetItemConfiguration::Item(item) => item,
        }
    }
}

#[derive(Debug,Deserialize)]
struct TypeSetConfiguration<V> {
    #[serde(default = "bool_true")]
    pub defaults: bool,
    #[serde(default)]
    pub includes: Vec<V>,
    #[serde(default)]
    pub excludes: Vec<String>,
}

impl<V> Default for TypeSetConfiguration<V> {
    fn default() -> Self {
        Self {
            defaults: bool_true(),
            includes: Default::default(),
            excludes: Default::default(),
        }
    }
}

impl<V> TypeSetConfiguration<V> {
    fn map_includes<T, MAP: FnMut(V)->T>(self, map: MAP) -> TypeSetConfiguration<T> {
        TypeSetConfiguration {
            defaults: self.defaults,
            includes: self.includes.into_iter().map(map).collect(),
            excludes: self.excludes.clone(),
        }
    }
}

fn default_typesetconfiguration<V>() -> TypeSetConfiguration<V>{
    TypeSetConfiguration::<V>::default()
}

fn bool_true() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use syn::{
        Type,
        parse_quote,
    };
    use super::*;

    macro_rules! assert_defaults {
        (!$($tt:tt)*) => {{
            let ty: Type = parse_quote!($($tt)*);
            assert!(
                Configuration::default().default_types().match_type(&ty).is_none(),
                "Default type: {:?}",
                stringify!($($tt)*)
            );
        }};
        ($($tt:tt)*) => {{
            let ty: Type = parse_quote!($($tt)*);
            assert!(
                Configuration::default().default_types().match_type(&ty).is_some(),
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
    fn crate_config_parse() {
        let config: CrateConfiguration = serde_yaml::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/config/macon-config-parse.yaml"))).unwrap();

        assert_eq!(config.version, "1", "version\n{:#?}", config);

        let default_types = &config.default_types;
        assert_eq!(default_types.defaults, true, "default_types.defaults\n{:#?}", config);

        let mut default_types_includes = default_types.includes.iter();
        assert_eq!(default_types_includes.next(), Some(&"DefaultOne".to_owned()), "default_types.includes[0]\n{:#?}", config);
        assert_eq!(default_types_includes.next(), Some(&"::full::path::DefaultTwo".to_owned()), "default_types.includes[1]\n{:#?}", config);
        assert_eq!(default_types_includes.next(), None, "default_types.includes[2]\n{:#?}", config);

        let mut default_types_excludes = default_types.excludes.iter();
        assert_eq!(default_types_excludes.next(), None, "default_types.excludes[0]\n{:#?}", config);

        let option_types = &config.option_types;
        assert_eq!(option_types.defaults, true, "option_types.defaults");

        let mut option_types_includes = option_types.includes.iter();
        assert_eq!(
            option_types_includes.next(),
            Some(&ContainerTypeSetItem {
                path: "AsString".to_owned(),
                wrapped: None,
            }),
            "option_types.includes[0]\n{:#?}", config
        );

        assert_eq!(
            option_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithoutWrapped".to_owned(), wrapped: None, }),
            "option_types.includes[1]\n{:#?}", config
        );

        assert_eq!(
            option_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithShortWrapped".to_owned(), wrapped: Some("ShortWrapped".to_string()), }),
            "option_types.includes[2]\n{:#?}", config
        );

        assert_eq!(
            option_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithFullWrapped".to_owned(), wrapped: Some("::full::path::FullWrapped".to_string()), }),
            "option_types.includes[3]\n{:#?}", config
        );

        assert!(option_types_includes.next().is_none(), "option_types.includes[4]\n{:#?}", config);

        let mut option_types_excludes = option_types.excludes.iter();
        assert_eq!(option_types_excludes.next(), None, "option_types.excludes[0]\n{:#?}", config);


        let extend_types = &config.extend_types;
        assert_eq!(extend_types.defaults, true, "extend_types.defaults");

        let mut extend_types_includes = extend_types.includes.iter();
        assert_eq!(
            extend_types_includes.next(),
            Some(&ContainerTypeSetItem {
                path: "AsString".to_owned(),
                wrapped: None,
            }),
            "extend_types.includes[0]\n{:#?}", config
        );

        assert_eq!(
            extend_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithoutWrapped".to_owned(), wrapped: None, }),
            "extend_types.includes[1]\n{:#?}", config
        );

        assert_eq!(
            extend_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithShortWrapped".to_owned(), wrapped: Some("ShortWrapped".to_string()), }),
            "extend_types.includes[2]\n{:#?}", config
        );

        assert_eq!(
            extend_types_includes.next(),
            Some(&ContainerTypeSetItem { path: "AsItemWithFullWrapped".to_owned(), wrapped: Some("::full::path::FullWrapped".to_string()), }),
            "extend_types.includes[3]\n{:#?}", config
        );

        assert!(extend_types_includes.next().is_none(), "extend_types.includes[4]\n{:#?}", config);

        let mut extend_types_excludes = extend_types.excludes.iter();
        assert_eq!(extend_types_excludes.next(), None, "extend_types.excludes[0]\n{:#?}", config);
    }
}
