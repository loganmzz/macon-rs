use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(Builder)]
#[derive(Debug,PartialEq)]
struct NamedStruct {
    one: Option<String>,
    two: Option<String>,
    three: Option<String>,
}

#[derive(Builder)]
#[derive(Debug,PartialEq)]
struct TupleStruct(
    Option<String>,
    Option<String>,
    Option<String>,
);

#[derive(Builder)]
#[builder(Option=!)]
#[derive(Debug,PartialEq)]
struct DisabledAtStruct {
    optional: Option<String>,
}

type OptString = Option<String>;
#[derive(Builder)]
#[derive(Debug,PartialEq)]
struct Alias {
    #[builder(Option=String)]
    optional: OptString,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_none() {
    let built = NamedStruct::builder()
        .build();
    assert_eq!(
        NamedStruct {
            one: None,
            two: None,
            three: None,
        },
        built,
    );
}

#[test]
fn named_one() {
    let built = NamedStruct::builder()
        .one("A")
        .build();
    assert_eq!(
        NamedStruct {
            one: Some("A".into()),
            two: None,
            three: None,
        },
        built,
    );
}

#[test]
fn named_two() {
    let built = NamedStruct::builder()
        .two("B")
        .build();
    assert_eq!(
        NamedStruct {
            one: None,
            two: Some("B".into()),
            three: None,
        },
        built,
    );
}

#[test]
fn named_thtree() {
    let built = NamedStruct::builder()
        .three("C")
        .build();
    assert_eq!(
        NamedStruct {
            one: None,
            two: None,
            three: Some("C".into()),
        },
        built,
    );
}

#[test]
fn named_one_three() {
    let built = NamedStruct::builder()
        .one("A")
        .three("C")
        .build();
    assert_eq!(
        NamedStruct {
            one: Some("A".into()),
            two: None,
            three: Some("C".into()),
        },
        built,
    );
}

#[test]
fn named_full() {
    let built = NamedStruct::builder()
        .one("A")
        .two("B")
        .three("C")
        .build();
    assert_eq!(
        NamedStruct {
            one: Some("A".into()),
            two: Some("B".into()),
            three: Some("C".into()),
        },
        built,
    );
}

#[test]
fn tuple_unordered_none() {
    let built = TupleStruct::builder()
        .build();
    assert_eq!(
        TupleStruct(
            None,
            None,
            None,
        ),
        built,
    );
}

#[test]
fn tuple_unordered_0() {
    let built = TupleStruct::builder()
        .set0("A")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            None,
            None,
        ),
        built,
    );
}

#[test]
fn tuple_unordered_1() {
    let built = TupleStruct::builder()
        .set1("B")
        .build();
    assert_eq!(
        TupleStruct(
            None,
            Some("B".into()),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_unordered_2() {
    let built = TupleStruct::builder()
        .set2("C")
        .build();
    assert_eq!(
        TupleStruct(
            None,
            None,
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn tuple_unordered_0_2() {
    let built = TupleStruct::builder()
        .set0("A")
        .set2("C")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            None,
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn tuple_unordered_full() {
    let built = TupleStruct::builder()
        .set0("A")
        .set1("B")
        .set2("C")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            Some("B".into()),
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn tuple_ordered_none() {
    let built = TupleStruct::builder()
        .none()
        .none()
        .none()
        .build();
    assert_eq!(
        TupleStruct(
            None,
            None,
            None,
        ),
        built,
    );
}

#[test]
fn tuple_ordered_0() {
    let built = TupleStruct::builder()
        .set("A")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            None,
            None,
        ),
        built,
    );
}

#[test]
fn tuple_ordered_1() {
    let built = TupleStruct::builder()
        .none()
        .set("B")
        .build();
    assert_eq!(
        TupleStruct(
            None,
            Some("B".into()),
            None,
        ),
        built,
    );
}

#[test]
fn tuple_ordered_2() {
    let built = TupleStruct::builder()
        .none()
        .none()
        .set("C")
        .build();
    assert_eq!(
        TupleStruct(
            None,
            None,
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn tuple_ordered_0_2() {
    let built = TupleStruct::builder()
        .set("A")
        .none()
        .set("C")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            None,
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn tuple_ordered_full() {
    let built = TupleStruct::builder()
        .set("A")
        .set("B")
        .set("C")
        .build();
    assert_eq!(
        TupleStruct(
            Some("A".into()),
            Some("B".into()),
            Some("C".into()),
        ),
        built,
    );
}

#[test]
fn disabled_at_struct() {
    let built = DisabledAtStruct::builder()
        .optional(Some(String::from("value")))
        .build();
    assert_eq!(
        DisabledAtStruct {
            optional: Some(String::from("value")),
        },
        built,
    );
}

#[test]
fn alias() {
    let built = Alias::builder()
        .optional("aliasing")
        .build();
    assert_eq!(
        Alias {
            optional: Some(String::from("aliasing")),
        },
        built,
    );
}
