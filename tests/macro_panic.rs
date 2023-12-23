use macon::Builder;

#[derive(Builder)]
#[builder(mode = Panic)]
#[derive(PartialEq,Debug)]
pub struct Foobar {
    foo: u8,
    bar: String,
}

// test
#[test]
fn builder_build() {
    let builder = Foobar::builder()
        .foo(2);
    let built = builder
        .bar("foobar")
        .build();
    assert_eq!(
        Foobar {
            foo: 2,
            bar: String::from("foobar"),
        },
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .foo(3)
        .bar("builder_into")
        .into();
    assert_eq!(
        Foobar {
            foo: 3,
            bar: String::from("builder_into"),
        },
        built,
    );
}
