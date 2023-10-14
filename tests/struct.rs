use macon::Builder;

#[derive(Builder,PartialEq,Debug)]
pub struct Foo {
    integer: i32,
    string: String,
}

#[test]
fn builder_function() {
    Foo::builder();
}

#[test]
fn builder_default() {
    FooBuilder::default();
}

#[test]
fn builder_build() {
    let built = Foo::builder()
        .integer(42)
        .string(String::from("foobar"))
        .build();
    assert_eq!(
        Foo {
            integer: 42,
            string: String::from("foobar"),
        },
        built,
    );
}
