use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Panic,)]
#[derive(PartialEq,Debug)]
struct Foobar {
    path1: PathBuf,
    path2: PathBuf,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_full() {
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
#[should_panic(expected = "Field path1 is missing")]
fn builder_build_missing() {
    Foobar::builder()
        .path2("/tmp/builder_build.002")
        .build();
}

#[test]
fn builder_into_full() {
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

#[test]
#[should_panic(expected = "Field path2 is missing")]
fn builder_into_missing() {
    Foobar::builder()
        .path1("/tmp/builder_into.001")
        .build();
}
