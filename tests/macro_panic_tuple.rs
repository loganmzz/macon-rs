use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Panic)]
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
        Foobar(
            PathBuf::from("/tmp/builder_build.0"),
            PathBuf::from("/tmp/builder_build.1"),
        ),
        built,
    );
}

#[test]
#[should_panic(expected = "Field 0 is missing")]
fn builder_build_missing() {
    Foobar::builder()
        .set1("/tmp/builder_build.002")
        .build();
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .set0("/tmp/builder_into.0")
        .set1("/tmp/builder_into.1")
        .into();
    assert_eq!(
        Foobar(
            PathBuf::from("/tmp/builder_into.0"),
            PathBuf::from("/tmp/builder_into.1"),
        ),
        built,
    );
}

#[test]
#[should_panic(expected = "Field 1 is missing")]
fn builder_into_missing() {
    Foobar::builder()
        .set0("/tmp/builder_into.001")
        .build();
}
