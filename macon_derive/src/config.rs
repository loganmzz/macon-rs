use std::{
    collections::HashSet,
    fs,
    env,
    ops::Deref,
    path,
    sync::OnceLock,
};
use anyhow::Context;
use serde::Deserialize;
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
        println!("Load Crate configuration from {:?}", config_path);
        let config_file = fs::File::open(config_path)?;
        Ok(Some(serde_yaml::from_reader(config_file)?))
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
    option_types: TypeSet,
}

#[derive(Debug,Default,)]
pub struct TypeSet {
    pathes: HashSet<String>,
}

impl TypeSet {
    fn create<F: FnOnce()->TypeSet>(config: TypeSetConfiguration, default_fn: F) -> Self {
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
            Type::Reference(TypeReference { elem, ..}) => self.match_type(elem.deref()),
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
        Self { default_types, option_types, }
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
        Self {
            default_types: TypeSet::create(crate_config.default_types, Self::default_default_types),
            option_types: TypeSet::create(crate_config.option_types, Self::default_option_types),
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
}

#[derive(Deserialize)]
struct CrateConfiguration {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
    #[serde(default)]
    pub default_types: TypeSetConfiguration,
    #[serde(default)]
    pub option_types: TypeSetConfiguration,
}

#[derive(Deserialize)]
struct TypeSetConfiguration {
    #[serde(default = "TypeSetConfiguration::default_defaults")]
    pub defaults: bool,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub excludes: Vec<String>,
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

#[cfg(test)]
mod tests {
    use syn::{
        Type,
        parse_quote,
    };
    use super::Configuration;

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
}
