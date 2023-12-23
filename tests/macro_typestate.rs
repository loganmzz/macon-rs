use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################

#[derive(Builder)]
#[builder(mode =Typestate)]
#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
    option: Option<String>,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let built = Foobar::builder()
        .foo(2)
        .bar("foobar")
        .option("optional")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: Some(String::from("optional")),
        },
        built,
    );
}
#[test]
fn builder_build_partial() {
    let built = Foobar::builder()
        .foo(2)
        .bar("foobar")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
            option: None,
        },
        built,
    );
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .option("into_option")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: Some(String::from("into_option")),
        },
        built,
    );
}

#[test]
fn builder_into_partial() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
            option: None,
        },
        built,
    );
}
