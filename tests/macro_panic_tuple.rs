use macon::Builder;

#[derive(Builder)]
#[builder(mode=Panic)]
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

// test
#[test]
fn builder_build_ok() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
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
