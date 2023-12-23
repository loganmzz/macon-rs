use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(PartialEq,Debug)]
#[derive(Builder)]
#[builder(mode=Result)]
pub struct Foobar {
    foo: u8,
    bar: String,
    option: Option<String>,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full_ok() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .option("optional")
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: Some(String::from("optional")),
        }),
        built,
    );
}

#[test]
fn builder_build_partial_ok() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: None,
        }),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .bar("foobar")
        .build();
    assert_eq!(
        Err(String::from("Field foo is missing")),
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .option("optional")
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: Some(String::from("optional")),
        },
        built,
    );
}
