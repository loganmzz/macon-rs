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
    let built = Foobar::builder()
        .foo(2)
        .bar(String::from("foobar"))
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
        .bar(String::from("foobar"))
        .build();
    assert_eq!(
        Err(String::from("Field foo is missing")),
        built,
    );
}
