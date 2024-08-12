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
struct FoobarBuilder {
    f: ::macon::Building<Box<dyn Fn(usize) -> usize>>,
}

// impl_builder
impl FoobarBuilder {
    // impl_builder / impl_builder_setters
    pub fn f<>(mut self, f: Box<dyn Fn(usize) -> usize>) -> FoobarBuilder {
        self.f = ::macon::Building::Set(f);
        self
    }

    // impl_builder / impl_builder_build
    pub fn build(self) -> ::core::result::Result<Foobar,::std::string::String> {
        let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

        if self.f.is_undefined() {
            errors.push("Field f is missing".into());
        }

        if !errors.is_empty() {
            ::core::result::Result::Err(errors.join("\n"))
        } else {
            ::core::result::Result::Ok(Foobar {
                f: self.f.unwrap(),
            })
        }
    }
}

// impl_builder / impl_builder_from
impl ::core::convert::TryFrom<FoobarBuilder> for Foobar {
    type Error = ::std::string::String;
    fn try_from(builder: FoobarBuilder) -> Result<Self, Self::Error> {
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
        .build()
        .unwrap();
    assert_eq!((built.f)(1), 2);
}
