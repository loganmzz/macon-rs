use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Result)]
#[derive(PartialEq,Debug)]
struct Foobar(
    PathBuf,
    PathBuf,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
    let built = Foobar::builder()
        .set1("/tmp/builder_build.1")
        .set0("/tmp/builder_build.0")
        .build();
    assert_eq!(
        Ok(Foobar(
            PathBuf::from("/tmp/builder_build.0"),
            PathBuf::from("/tmp/builder_build.1"),
        )),
        built,
    );
}

#[test]
fn builder_build_missing() {
    let built = Foobar::builder()
        .set1("/tmp/builder_build.002")
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    )
}

#[test]
fn builder_into_full() {
    let built: Result<Foobar,_> = Foobar::builder()
        .set0("/tmp/builder_into.0")
        .set1("/tmp/builder_into.1")
        .try_into();
    assert_eq!(
        Ok(Foobar(
            PathBuf::from("/tmp/builder_into.0"),
            PathBuf::from("/tmp/builder_into.1"),
        )),
        built,
    );
}

#[test]
fn builder_into_missing() {
    let built: Result<Foobar,_> = Foobar::builder()
        .set0("/tmp/builder_into.001")
        .try_into();
    assert_eq!(
        Err(String::from("Field 1 is missing")),
        built,
    );
}
