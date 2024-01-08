use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder,)]
#[builder(mode=Result,)]
#[derive(PartialEq,Debug)]
struct Named {
    mandatory: PathBuf,
    #[builder(Default=!,)]
    option: Option<PathBuf>,
}

#[derive(Builder,)]
#[builder(mode=Result,)]
#[derive(PartialEq,Debug)]
struct Tuple(
    PathBuf,
    #[builder(Default=!,)]
    Option<PathBuf>,
);

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn named_builder_build_full() {
    let built = Named::builder()
        .option("/tmp/builder_build_full/option")
        .mandatory("/tmp/builder_build_full/mandatory")
        .build();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_build_full/mandatory"),
            option: Some(PathBuf::from("/tmp/builder_build_full/option")),
        }),
        built,
    );
}

#[test]
fn named_builder_build_partial_implicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_build_partial_implicit/mandatory")
        .build();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_build_partial_implicit/mandatory"),
            option: None,
        }),
        built,
    );
}

#[test]
fn named_builder_build_partial_explicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_build_partial_explicit/mandatory")
        .option_none()
        .build();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_build_partial_explicit/mandatory"),
            option: None,
        }),
        built,
    );
}
#[test]
fn named_builder_into_full() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_full/mandatory")
        .option("/tmp/builder_into_full/option")
        .try_into();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_into_full/mandatory"),
            option: Some(PathBuf::from("/tmp/builder_into_full/option")),
        }),
        built,
    );
}

#[test]
fn named_builder_into_partial_implicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_partial_implicit/mandatory")
        .try_into();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_into_partial_implicit/mandatory"),
            option: None,
        }),
        built,
    );
}

#[test]
fn named_builder_into_partial_explicit() {
    let built = Named::builder()
        .mandatory("/tmp/builder_into_partial_explicit/mandatory")
        .option_none()
        .try_into();
    assert_eq!(
        Ok(Named {
            mandatory: PathBuf::from("/tmp/builder_into_partial_explicit/mandatory"),
            option: None,
        }),
        built,
    );
}

#[test]
fn tuple_builder_build_full() {
    let built = Tuple::builder()
        .set1("/tmp/builder_build_full/option")
        .set0("/tmp/builder_build_full/mandatory")
        .build();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_build_full/mandatory"),
            Some(PathBuf::from("/tmp/builder_build_full/option")),
        )),
        built,
    );
}

#[test]
fn tuple_builder_build_partial_implicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_build_partial_implicit/mandatory")
        .build();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_build_partial_implicit/mandatory"),
            None,
        )),
        built,
    );
}

#[test]
fn tuple_builder_build_partial_explicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_build_partial_explicit/mandatory")
        .set1_none()
        .build();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_build_partial_explicit/mandatory"),
            None,
        )),
        built,
    );
}

#[test]
fn tuple_builder_into_full() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_full/mandatory")
        .set1("/tmp/builder_into_full/option")
        .try_into();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_into_full/mandatory"),
            Some(PathBuf::from("/tmp/builder_into_full/option")),
        )),
        built,
    );
}

#[test]
fn tuple_builder_into_partial_implicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_partial_implicit/mandatory")
        .try_into();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_into_partial_implicit/mandatory"),
            None,
        )),
        built,
    );
}

#[test]
fn tuple_builder_into_partial_explicit() {
    let built = Tuple::builder()
        .set0("/tmp/builder_into_partial_explicit/mandatory")
        .set1_none()
        .try_into();
    assert_eq!(
        Ok(Tuple(
            PathBuf::from("/tmp/builder_into_partial_explicit/mandatory"),
            None,
        )),
        built,
    );
}
