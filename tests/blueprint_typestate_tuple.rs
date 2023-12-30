// #############################################################################
// ################################### INPUT ###################################
// #############################################################################
#[derive(PartialEq,Debug)]
pub struct Foobar(u8,String,Option<String>,);

// #############################################################################
// ############################## IMPLEMENTATION ###############################
// #############################################################################

// impl_target
impl Foobar {
    pub fn builder() -> FoobarBuilder {
        ::core::default::Default::default()
    }
}

// struct_builder
#[derive(Default)]
pub struct FoobarBuilder<V0=(), V1=(), V2=(),>(
    V0,
    V1,
    Option<String>,
    ::core::marker::PhantomData<V2>,
);

// impl_builder
// impl_builder / impl_builder_setter
impl<V1,V2,> FoobarBuilder<(),V1,V2,> {
    pub fn set0<V0: ::core::convert::Into<u8>>(self, v0: V0) -> FoobarBuilder<u8,V1,V2> {
        FoobarBuilder(
            v0.into(),
            self.1,
            self.2,
            ::core::default::Default::default(),
        )
    }
}
impl FoobarBuilder<(),(),(),> {
    pub fn set<V0: ::core::convert::Into<u8>>(self, v0: V0) -> FoobarBuilder<u8,(),(),> {
        self.set0(v0)
    }
}

// impl_builder / impl_builder_setter
impl<V0,V2,> FoobarBuilder<V0,(),V2,> {
    pub fn set1<V1: ::core::convert::Into<String>>(self, v1: V1) -> FoobarBuilder<V0,String,V2,> {
        FoobarBuilder(
            self.0,
            v1.into(),
            self.2,
            ::core::default::Default::default(),
        )
    }
}
impl FoobarBuilder<u8,(),(),> {
    pub fn set<V1: ::core::convert::Into<String>>(self, v1: V1) -> FoobarBuilder<u8,String,(),> {
        self.set1(v1)
    }
}

// impl_builder / impl_builder_setter
impl<V0,V1,> FoobarBuilder<V0,V1,(),> {
    pub fn set2<V2: ::core::convert::Into<String>>(self, v2: V2) -> FoobarBuilder<V0,V1,Option<String>,> {
        FoobarBuilder(
            self.0,
            self.1,
            v2.into().into(),
            ::core::default::Default::default(),
        )
    }
    pub fn set2_none(self) -> FoobarBuilder<V0,V1,Option<String>,> {
        FoobarBuilder(
            self.0,
            self.1,
            ::core::option::Option::None,
            ::core::default::Default::default(),
        )
    }
}
impl FoobarBuilder<u8,String,(),> {
    pub fn set<V2: ::core::convert::Into<String>>(self, v2: V2) -> FoobarBuilder<u8,String,Option<String>,> {
        self.set2(v2)
    }
    pub fn none(self) -> FoobarBuilder<u8,String,Option<String>,> {
        self.set2_none()
    }
}

// impl_builder
// impl_builder / impl_builder_build
impl<OPTION> FoobarBuilder<u8,String,OPTION,> {
    pub fn build(self) -> Foobar {
        Foobar(
            self.0,
            self.1,
            self.2,
        )
    }
}

// impl_builder
// impl_builder / impl_builder_from
impl ::core::convert::From<FoobarBuilder<u8,String,(),>> for Foobar {
    fn from(builder: FoobarBuilder<u8,String,()>) -> Self {
        builder.build()
    }
}

impl ::core::convert::From<FoobarBuilder<u8,String,Option<String>,>> for Foobar {
    fn from(builder: FoobarBuilder<u8,String,Option<String>>) -> Self {
        builder.build()
    }
}


// #############################################################################
// ################################### TESTS ###################################
// #############################################################################

#[test]
fn builder_build_set_full() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .set("optional")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        ),
        built,
    );
}

#[test]
fn builder_build_set_partial_explicit() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .none()
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_partial_implicit() {
    let built = Foobar::builder()
        .set(2)
        .set("foobar")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_full() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .set2("optional")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            Some(String::from("optional")),
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_partial_explicit() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .set2_none()
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_build_set_n_partial_implicit() {
    let built = Foobar::builder()
        .set0(2)
        .set1("foobar")
        .build();
    assert_eq!(
        Foobar(
            2,
            String::from("foobar"),
            None,
        ),
        built,
    );
}

#[test]
fn builder_into_full() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .set2("optional_into")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            Some(String::from("optional_into")),
        ),
        built,
    );
}

#[test]
fn builder_into_partial() {
    let built: Foobar = Foobar::builder()
        .set0(3)
        .set1("builder_into")
        .into();
    assert_eq!(
        Foobar(
            3,
            String::from("builder_into"),
            None,
        ),
        built,
    );
}
