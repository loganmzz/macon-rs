#[derive(PartialEq,Debug)]
pub struct Foobar(
    u8,
    String,
);

// struct_builder
// struct_builder / struct_builder_tuple
#[derive(Default)]
pub struct FoobarBuilder(
    Option<u8>,
    Option<String>,
);

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        Default::default()
    }
}

// impl_builder
impl FoobarBuilder {
    // impl_builder_setters
    // impl_builder_setters / impl_builder_setters_tuple
    pub fn set0(&mut self, value: u8) -> &mut Self {
        self.0 = value.into();
        self
    }

    pub fn set1(&mut self, value: String) -> &mut Self {
        self.1 = value.into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_tuple
    pub fn build(&mut self) -> Foobar {
        let mut errors: Vec<String> = vec![];

        if self.0.is_none() {
            errors.push("Field 0 is missing".into());
        }
        if self.1.is_none() {
            errors.push("Field 1 is missing".into());
        }

        if !errors.is_empty() {
            panic!("{}", errors.join("\n"))
        } else {
            Foobar(
                self.0.take().unwrap(),
                self.1.take().unwrap(),
            )
        }
    }
}

// test
#[test]
fn builder_build() {
    let built = Foobar::builder()
        .set0(2)
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
        ),
        built,
    );
}
