use macon::Builder;

#[derive(Builder)]
#[derive(PartialEq,Debug)]
pub struct Foo;

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
    assert_eq!(Foo, Foo::builder().build());
}
