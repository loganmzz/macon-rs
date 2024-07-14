use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
use std::path::PathBuf;

#[derive(Builder)]
#[derive(PartialEq,Debug)]
#[builder(mode=Panic,)]
#[builder(Default)]
struct DefaultStructNamed {
    list: Vec<PathBuf>,
}

impl Default for DefaultStructNamed {
    fn default() -> Self {
        Self {
            list: vec![
                "a",
                "b",
                "c",
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect(),
        }
    }
}

#[derive(Builder)]
#[derive(PartialEq,Debug)]
#[builder(mode=Panic,)]
struct DefaultFieldNamed {
    list: Vec<PathBuf>,
}

#[derive(Builder)]
#[derive(PartialEq,Debug)]
#[builder(mode=Panic,)]
struct MandatoryFieldNamed {
    #[builder(Default=!)]
    list: Vec<PathBuf>,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn defaultstructnamed_builder_struct_default() {
    let built = DefaultStructNamed::builder()
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("a"),
                PathBuf::from("b"),
                PathBuf::from("c"),
            ],
        },
        built,
    );
}

#[test]
fn defaultstructnamed_builder_keep_extend() {
    let built = DefaultStructNamed::builder()
        .list_keep()
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultStructNamed {
            list: vec![
                PathBuf::from("a"),
                PathBuf::from("b"),
                PathBuf::from("c"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

// #[test]
// fn defaultstructnamed_builder_default_extend() {
//     let built = DefaultStructNamed::builder()
//         .list_default()
//         // .list_extend(&["d", "e", "f",])
//         .build();

//     assert_eq!(
//         DefaultStructNamed {
//             list: vec![
//                 PathBuf::from("d"),
//                 PathBuf::from("e"),
//                 PathBuf::from("f"),
//             ],
//         },
//         built,
//     );
// }

// #[test]
// fn defaultstructnamed_builder_set_extend() {
//     let built = DefaultStructNamed::builder()
//         .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
//         // .list_extend(&["d", "e", "f",])
//         .build();

//     assert_eq!(
//         DefaultStructNamed {
//             list: vec![
//                 PathBuf::from("g"),
//                 PathBuf::from("h"),
//                 PathBuf::from("i"),
//                 PathBuf::from("d"),
//                 PathBuf::from("e"),
//                 PathBuf::from("f"),
//             ],
//         },
//         built,
//     );
// }

#[test]
fn defaultfieldnamed_builder_field_default() {
    let built = DefaultFieldNamed::builder()
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![],
        },
        built,
    );
}

#[test]
fn defaultfieldnamed_builder_default_extend() {
    let built = DefaultFieldNamed::builder()
        .list_default()
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn defaultfieldnamed_builder_set_extend() {
    let built = DefaultFieldNamed::builder()
        .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        DefaultFieldNamed {
            list: vec![
                PathBuf::from("g"),
                PathBuf::from("h"),
                PathBuf::from("i"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
fn mandatoryfieldnamed_builder_set_extend() {
    let built = MandatoryFieldNamed::builder()
        .list(vec!["g", "h", "i",].into_iter().map(PathBuf::from).collect::<Vec<_>>())
        .list_extend(&["d", "e", "f",])
        .build();

    assert_eq!(
        MandatoryFieldNamed {
            list: vec![
                PathBuf::from("g"),
                PathBuf::from("h"),
                PathBuf::from("i"),
                PathBuf::from("d"),
                PathBuf::from("e"),
                PathBuf::from("f"),
            ],
        },
        built,
    );
}

#[test]
#[should_panic(expected = "Field list is missing")]
fn mandatoryfieldnamed_builder_missing() {
    let _built = MandatoryFieldNamed::builder()
        .list_extend(&["d", "e", "f",])
        .build();
}
