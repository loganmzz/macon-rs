use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(Builder)]
#[builder(mode=Panic)]
#[derive(PartialEq,Debug)]
pub struct Foobar(
    u8,
    String,
    Option<String>,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
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
fn builder_build_partial() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
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
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .set2("optional")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            Some(String::from("optional")),
        ),
        built,
    );
}

