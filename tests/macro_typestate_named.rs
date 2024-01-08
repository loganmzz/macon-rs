use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Typestate,)]
#[derive(PartialEq,Debug)]
struct Foobar {
    path1: PathBuf,
    path2: PathBuf,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build() {
    let built = Foobar::builder()
        .path1("/tmp/builder_build.001")
        .path2("/tmp/builder_build.002")
        .build();
    assert_eq!(
        Foobar {
            path1: PathBuf::from("/tmp/builder_build.001"),
            path2: PathBuf::from("/tmp/builder_build.002"),
        },
        built,
    );
}

#[test]
fn builder_into() {
    let built: Foobar = Foobar::builder()
    .path1("/tmp/builder_into.001")
    .path2("/tmp/builder_into.002")
        .into();
    assert_eq!(
        Foobar {
            path1: PathBuf::from("/tmp/builder_into.001"),
            path2: PathBuf::from("/tmp/builder_into.002"),
        },
        built,
    );
}
