use macon::Builder;

#[derive(PartialEq,Debug)]
#[derive(Builder)]
#[builder(mode=Result)]
pub struct Foobar(
    u8,
    String,
);


// test
#[test]
fn builder_build_ok() {
    let built = Foobar::builder()
        .set0(2)
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
        )),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    );
}
