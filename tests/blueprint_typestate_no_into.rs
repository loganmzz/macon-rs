// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
struct Foobar {
  f: Box<dyn Fn(usize) -> usize>,
}

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        <FoobarBuilder as ::core::default::Default>::default()
    }
}

// struct_builder
#[derive(Default,)]
struct FoobarBuilder<F=(),> {
    f: F,
    __typestate_markers: ::core::marker::PhantomData<(Box<dyn Fn(usize) -> usize>,)>,
}

// impl_builder
// impl_builder / impl_builder_setter
impl<> FoobarBuilder<(),> {
    pub fn f<>(self, f: Box<dyn Fn(usize) -> usize>) -> FoobarBuilder<Box<dyn Fn(usize) -> usize>,> {
        FoobarBuilder {
            f: f.into(),
            __typestate_markers: ::core::default::Default::default(),
        }
    }
}

// impl_builder / impl_builder_build
impl<> FoobarBuilder<Box<dyn Fn(usize) -> usize>,> {
    pub fn build(self) -> Foobar {
        Foobar {
            f: self.f,
        }
    }
}

// impl_builder / impl_builder_from
impl<> ::core::convert::From<FoobarBuilder<Box<dyn Fn(usize) -> usize>,>> for Foobar {
    fn from(builder: FoobarBuilder<Box<dyn Fn(usize) -> usize>,>) -> Self {
        builder.build()
    }
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
