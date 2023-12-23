use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug)]
#[derive(Builder)]
#[builder(mode=Result)]
pub struct Foobar(
    u8,
    String,
    Option<String>,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################


#[test]
fn builder_build_full_ok() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
        .set2("optional")
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        )),
        built,
    );
}

#[test]
fn builder_build_partial_ok() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
            None,
        )),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .set1("foobar")
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .set2("optional")
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            Some(String::from("optional")),
        ),
        built,
    );
}
