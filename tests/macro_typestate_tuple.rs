use macon::Builder;

#[derive(Builder)]
#[builder(mode=Typestate)]
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

#[test]
fn builder_build() {
    let built = Foobar::builder()
        .set(2)
        .set(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
        ),
        built,
    );
}
