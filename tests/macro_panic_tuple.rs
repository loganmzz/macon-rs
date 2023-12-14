use macon::Builder;

#[derive(Builder)]
#[builder(mode=Panic)]
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

// test
#[test]
fn builder_build_ok() {
    let built = Foobar::builder()
        .set0(2)
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
        ),
        built,
    );
}
