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
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1("foobar")
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
        .set1("foobar")
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    );
}

// #[test]
// fn builder_into() {
//     let built: Foobar = Foobar::builder()
//         .set0(3)
//         .set1("builder_into")
//         .try_into()
//         .unwrap();
//     assert_eq!(
//         Foobar(
//             3,
//             String::from("builder_into"),
//         ),
//         built,
//     );
// }
