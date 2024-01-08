use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Typestate)]
#[derive(PartialEq,Debug)]
struct Foobar(
    PathBuf,
    PathBuf,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_unordered() {
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
fn builder_build_ordered() {
    let built = Foobar::builder()
        .set("/tmp/builder_build.0")
        .set("/tmp/builder_build.1")
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
fn builder_into() {
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
