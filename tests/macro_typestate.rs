use macon::Builder;

#[derive(Builder)]
#[builder(mode =Typestate)]
#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
}

#[test]
fn builder_build() {
    let built = Foobar::builder()
        .foo(2)
        .bar(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
        },
        built,
    );
}
