use macon::Builder;

#[derive(Builder)]
#[builder(mode=Typestate)]
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

#[test]
fn builder_build() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
        ),
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
        ),
        built,
    );
}
