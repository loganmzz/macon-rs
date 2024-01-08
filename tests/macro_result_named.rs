use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[builder(mode=Result)]
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
        Ok(Foobar {
            path1: PathBuf::from("/tmp/builder_build.001"),
            path2: PathBuf::from("/tmp/builder_build.002"),
        }),
        built,
    );
}

#[test]
fn builder_build_missing() {
    let built = Foobar::builder()
        .path2("/tmp/builder_build.002")
        .build();
    assert_eq!(
        Err(String::from("Field path1 is missing")),
        built,
    );
}

#[test]
fn builder_into_full() {
    let built = Foobar::builder()
        .path1("/tmp/builder_into.001")
        .path2("/tmp/builder_into.002")
        .try_into();
    assert_eq!(
        Ok(Foobar {
            path1: PathBuf::from("/tmp/builder_into.001"),
            path2: PathBuf::from("/tmp/builder_into.002"),
        }),
        built,
    );
}

#[test]
fn builder_into_missing() {
    let built: Result<Foobar, _> = Foobar::builder()
        .path1("/tmp/builder_into.001")
        .try_into();
    assert_eq!(
        Err(String::from("Field path2 is missing")),
        built,
    );
}
