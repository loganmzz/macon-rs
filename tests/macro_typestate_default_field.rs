use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use ::std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Typestate,)]
#[derive(PartialEq,Debug,)]
struct StructNamed {
    id: i32,
    value: String,
    optional: Option<String>,
    mandatory: PathBuf,
}

#[derive(Builder)]
#[builder(mode=Typestate,)]
#[derive(PartialEq,Debug,)]
struct StructTuple(
    i32,
    String,
    Option<String>,
    PathBuf,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_build_default_implicit() {
    let built = StructNamed::builder()
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
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
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
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
        StructNamed {
            id: 0,
            value: String::from(""),
            optional: None,
            mandatory: PathBuf::from("/dev/null"),
        },
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
        StructNamed {
            id: 42,
            value: String::from("any value"),
            optional: Some(String::from("optional")),
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn tuple_build_default_implicit() {
    let built = StructTuple::builder()
        .set3("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
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
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
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
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
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
        StructTuple(
            42,
            String::from("any value"),
            Some(String::from("optional")),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_default_explicit() {
    let built = StructTuple::builder()
        .default()
        .default()
        .default()
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_default_explicit_none() {
    let built = StructTuple::builder()
        .default()
        .default()
        .none()
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            0,
            String::from(""),
            None,
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_ordered_build_full() {
    let built = StructTuple::builder()
        .set(42)
        .set("any value")
        .set("optional")
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            42,
            String::from("any value"),
            Some(String::from("optional")),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}
