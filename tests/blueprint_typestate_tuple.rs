#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String);

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        Default::default()
    }
}

// struct_builder
// struct_builder / struct_builder_tuple
pub struct FoobarBuilder<V0=(), V1=()>(V0, V1);


// default_builder
// default_builder / default_builder_tuple
impl core::default::Default for FoobarBuilder {
    fn default() -> Self {
        Self(
            core::default::Default::default(),
            core::default::Default::default(),
        )
    }
}


// impl_builder
// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_tuple
impl FoobarBuilder<(),()> {
    pub fn set(self, v0: u8) -> FoobarBuilder<u8,()> {
        self.set0(v0)
    }
}
impl<V1> FoobarBuilder<(),V1> {
    pub fn set0(self, v0: u8) -> FoobarBuilder<u8,V1> {
        FoobarBuilder(v0, self.1)
    }
}

// impl_builder / impl_builder_setter
// impl_builder / impl_builder_setter / impl_builder_setter_tuple
impl FoobarBuilder<u8,()> {
    pub fn set(self, v1: String) -> FoobarBuilder<u8,String> {
        self.set1(v1)
    }
}
impl<V0> FoobarBuilder<V0,()> {
    pub fn set1(self, v1: String) -> FoobarBuilder<V0,String> {
        FoobarBuilder(self.0, v1)
    }
}

// impl_builder
// impl_builder / impl_builder_build
// impl_builder / impl_builder_build / impl_builder_build_tuple
impl FoobarBuilder<u8,String> {
    pub fn build(self) -> Foobar {
        Foobar(
            self.0,
            self.1,
        )
    }
}

// test
#[test]
fn builder_build_set() {
    let built = Foobar::builder()
        .set(2)
        .set(String::from("foobar"))
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
        ),
        built,
    );
}

#[test]
fn builder_build_set_n() {
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
