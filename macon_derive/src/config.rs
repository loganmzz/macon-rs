use std::{
    collections::HashSet,
    sync::OnceLock, ops::Deref,
};
use syn::{
    Path,
    Type,
    TypePath,
    TypeReference,
};

pub fn get() -> &'static Configuration {
    static CONFIGURATION: OnceLock<Configuration> = OnceLock::new();
    CONFIGURATION.get_or_init(|| Default::default())
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
        let default_types = TypeSet::default()
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
        ;
        let option_types = TypeSet::default()
            .add_path("std::option::Option")
            .add_path("core::option::Option")
        ;
        Self { default_types, option_types, }
    }
}

impl Configuration {

    pub fn default_types(&self) -> &TypeSet {
        &self.default_types
    }
    pub fn option_types(&self) -> &TypeSet {
        &self.option_types
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
