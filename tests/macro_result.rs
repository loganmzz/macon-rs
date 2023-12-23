use macon::Builder;

#[derive(PartialEq,Debug)]
#[derive(Builder)]
#[builder(mode=Result)]
pub struct Foobar {
    foo: u8,
    bar: String,
}


// test
#[test]
fn builder_build_ok() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Ok(Foobar {
            foo: 2,
            bar: String::from("foobar"),
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
        .try_into()
        .unwrap();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
        },
        built,
    );
}
