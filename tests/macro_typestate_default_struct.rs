use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(Builder)]
#[builder(mode=Typestate,)]
#[derive(PartialEq,Debug,Default)]
struct StructNamed {
    value: String,
    optional: Option<String>,
}

#[derive(Builder)]
#[builder(mode=Typestate,)]
#[derive(PartialEq,Debug,Default)]
struct StructTuple(
    String,
    Option<String>,
);

#[derive(Builder)]
#[builder(mode=Typestate,Default,)]
#[derive(PartialEq,Debug,)]
struct EnabledDefault {
    value: String,
    optional: Option<String>,
}

impl Default for EnabledDefault {
    fn default() -> Self {
        Self {
            value: String::from("42"),
            optional: Some(String::from("some")),
        }
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

#[test]
fn tuple_build_ordered_keep_explicit() {
    let built = StructTuple::builder()
        .keep()
        .keep()
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
fn tuple_build_ordered_keep_explicit_none() {
    let built = StructTuple::builder()
        .keep()
        .none()
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
fn tuple_ordered_build_full() {
    let built = StructTuple::builder()
        .set("any value")
        .set("optional")
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
fn tuple_ordered_build_default() {
    let built = StructTuple::builder()
        .default()
        .default()
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
fn enabled_build_keep_implicit() {
    let built = EnabledDefault::builder()
        .build();
    assert_eq!(
        EnabledDefault {
            value: String::from("42"),
            optional: Some(String::from("some")),
        },
        built,
    )
}

#[test]
fn enabled_build_partial() {
    let built = EnabledDefault::builder()
        .optional("overriden")
        .build();
    assert_eq!(
        EnabledDefault {
            value: String::from("42"),
            optional: Some(String::from("overriden")),
        },
        built,
    )
}

#[test]
fn enabled_build_option_none() {
    let built = EnabledDefault::builder()
        .optional_none()
        .build();
    assert_eq!(
        EnabledDefault {
            value: String::from("42"),
            optional: None,
        },
        built,
    )
}

#[test]
fn enabled_build_option_keep() {
    let built = EnabledDefault::builder()
        .optional_keep()
        .build();
    assert_eq!(
        EnabledDefault {
            value: String::from("42"),
            optional: Some(String::from("some")),
        },
        built,
    )
}

#[test]
fn enabled_build_value_default() {
    let built = EnabledDefault::builder()
        .value_default()
        .build();
    assert_eq!(
        EnabledDefault {
            value: String::from(""),
            optional: Some(String::from("some")),
        },
        built,
    )
}
