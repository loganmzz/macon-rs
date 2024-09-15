use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use ::std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Typestate,fields(Default=!),)]
#[derive(PartialEq,Debug,)]
struct StructNamed {
    id: i32,
    value: String,
    optional: Option<String>,
    #[builder(Default)]
    exception: String,
    mandatory: PathBuf,
}

#[derive(Builder)]
#[builder(mode=Typestate,fields(Default=!),)]
#[derive(PartialEq,Debug,)]
struct StructTuple(
    i32,
    String,
    Option<String>,
    #[builder(Default)]
    String,
    PathBuf,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_build_minimum() {
    let built = StructNamed::builder()
        .id(42)
        .value("a value")
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 42,
            value: String::from("a value"),
            optional: None,
            exception: String::from(""),
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn named_build_default_explicit_none() {
    let built = StructNamed::builder()
        .id(43)
        .value("another value")
        .optional_none()
        .exception_default()
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 43,
            value: String::from("another value"),
            optional: None,
            exception: String::from(""),
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn named_build_full() {
    let built = StructNamed::builder()
        .id(44)
        .value("any value")
        .optional("optional")
        .exception("exception")
        .mandatory("/dev/null")
        .build();
    assert_eq!(
        StructNamed {
            id: 44,
            value: String::from("any value"),
            optional: Some(String::from("optional")),
            exception: String::from("exception"),
            mandatory: PathBuf::from("/dev/null"),
        },
        built,
    )
}

#[test]
fn tuple_build_minimum() {
    let built = StructTuple::builder()
        .set0(42)
        .set1("a value")
        .set4("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            42,
            String::from("a value"),
            None,
            String::from(""),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_default_explicit_none() {
    let built = StructTuple::builder()
        .set0(43)
        .set1("another value")
        .set2_none()
        .set3_default()
        .set4("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            43,
            String::from("another value"),
            None,
            String::from(""),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_unordered_full() {
    let built = StructTuple::builder()
        .set0(44)
        .set1("any value")
        .set2("optional")
        .set3("exception")
        .set4("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            44,
            String::from("any value"),
            Some(String::from("optional")),
            String::from("exception"),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_build_ordered_default_explicit_none() {
    let built = StructTuple::builder()
        .set(43)
        .set("another value")
        .none()
        .default()
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            43,
            String::from("another value"),
            None,
            String::from(""),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}

#[test]
fn tuple_ordered_build_full() {
    let built = StructTuple::builder()
        .set(44)
        .set("any value")
        .set("optional")
        .set("exception")
        .set("/dev/null")
        .build();
    assert_eq!(
        StructTuple(
            44,
            String::from("any value"),
            Some(String::from("optional")),
            String::from("exception"),
            PathBuf::from("/dev/null"),
        ),
        built,
    )
}
