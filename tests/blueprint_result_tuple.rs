#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

// struct_builder
// struct_builder / struct_builder_tuple
#[derive(Default)]
pub struct FoobarBuilder(Option<u8>,Option<String>);

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
    pub fn set0(mut self, value: u8) -> Self {
        self.0 = value.into();
        self
    }
    pub fn set1(mut self, value: String) -> Self {
        self.1 = value.into();
        self
    }

    // impl_builder_build
    // impl_builder_build / impl_builder_build_tuple
    pub fn build(self) -> Result<Foobar, String> {
        let mut errors: Vec<String> = vec![];

        if self.0.is_none() {
            errors.push("Field 0 is missing".into());
        }
        if self.1.is_none() {
            errors.push("Field 1 is missing".into());
        }

        if !errors.is_empty() {
            Err(errors.join("\n"))
        } else {
            Ok(Foobar(
                self.0.unwrap(),
                self.1.unwrap(),
            ))
        }
    }
}

// test
#[test]
fn builder_build_ok() {
    let builder = Foobar::builder()
        .set0(2);
    let built = builder
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Ok(Foobar(
            2,
            String::from("foobar"),
        )),
        built,
    );
}

#[test]
fn builder_build_missing_foo() {
    let built = Foobar::builder()
        .set1(String::from("foobar"))
        .build();
    assert_eq!(
        Err(String::from("Field 0 is missing")),
        built,
    );
}