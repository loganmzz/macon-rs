use macon::Builder;

// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
#[derive(Builder)]
#[builder(mode=Panic,)]
#[builder(fields(Into=!))]
struct Foobar {
    f: Box<dyn Fn(usize) -> usize>,
}

// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build() {
    let built = Foobar::builder()
        .f(Box::new(|x| x + 1))
        .build();
    assert_eq!((built.f)(1), 2);
}
