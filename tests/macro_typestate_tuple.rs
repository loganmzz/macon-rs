use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(Builder)]
#[builder(mode=Typestate)]
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String,Option<String>,);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_set_full() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .set("optional")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        ),
        built,
    );
}

#[test]
fn builder_build_set_partial_explicit() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .none()
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_partial_implicit() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_full() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .set2("optional")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_partial_explicit() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .set2_none()
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_partial_implicit() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .set2("optional_into")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            Some(String::from("optional_into")),
        ),
        built,
    );
}

#[test]
fn builder_into_partial() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            None,
        ),
        built,
    );
}
