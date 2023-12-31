Another builder macro-based generator with its own idioms.

"[Maçon](https://fr.wiktionary.org/wiki/ma%C3%A7on#Nom_commun_2)" is French translation for "[builder](https://www.wordreference.com/enfr/builder)"

### Usage

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyType {
  integer: i32,
  string: String,
  optional: Option<String>,
}

let _mytype: MyType = MyType::builder()
    .integer(42)
    .string(String::from("foobar"))
    .build();
```

* adds a builder struct (`<TargetStruct>Builder`)
* build struct implements [`Default`][Default]
* adds a `builder()` function to target struct to initialize a new builder
* each target struct field can be set with function of same name and parameter of same type
* use `build()` function to create new target struct instance
* any unset field will make `build()` call not compile (default)
* setter argument is generic over [`Into`][Into]
* [`Option`][Option] fields are not mandatory. And setters use wrapped type.

### Settings

Settings are set using `#[builder()]` attribute.

#### struct

* **`mode=<value>`** <br/>
Change builder and associated `build()` function behavior. Supported values: [`Typestate`](#typestate-pattern-default) (_default_), [`Panic`](#panic-on-build) or [`Result`](#result-on-build).

* **`Option=!`** <br/>
Disable automatic [`Option`][Option] detection for whole struct.

#### field

* **`Option=!`** <br/>
Disable automatic [`Option`][Option] detection for given field. Generated setter will rely on declared field type.

* **`Option=WrappedType`** <br/>
Enforce [`Option`][Option] support for given field. Generated setter will rely on `WrappedType`.


### Features

For any feature, you can find blueprints in [`./tests` directory)[tests] showing code generated by macro.

#### Typestate pattern (default)

Blueprint: [`blueprint_typestate.rs`][blueprint_typestate.rs]

By default, builder rely on typestate pattern. It means state is encoded in type (using generics). Applicable functions are implemented
(callable) only when state (type) matches:

* Build function `build()` when all properties has been set
* Each property setter function as long as property haven't been set

Optionally, you can set it explictly:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
#[builder(mode=Typestate)]
struct MyType {
  integer: i32,
  string: String,
}
```

#### Panic on `build()`

Blueprint: [`blueprint_panic.rs`][blueprint_panic.rs]

By default, builder rely on typestate pattern to avoid misconfiguration by adding compilation constraint. You can switch to a builder
that just panic when misconfigured:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
#[builder(mode=Panic)]
struct MyType {
  integer: i32,
  string: String,
}
let _mytype: MyType = MyType::builder()
    .integer(42)
    .build();
```

#### Result on `build()`

Blueprint: [`blueprint_result.rs`][blueprint_result.rs]

By default, builder rely on typestate pattern to avoid misconfiguration by adding compilation constraint. You can switch to a builder
that returns a [`Result`][Result]:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
#[builder(mode=Result)]
struct MyType {
  integer: i32,
  string: String,
}

let myTypeResult: Result<MyType,String> = MyType::builder()
    .integer(42)
    .build();

assert!(myTypeResult.is_err());
```

#### Tuple

Blueprints:
* [`blueprint_typestate_tuple.rs`][blueprint_typestate_tuple.rs]
* [`blueprint_panic_tuple.rs`][blueprint_panic_tuple.rs]
* [`blueprint_result_tuple.rs`][blueprint_result_tuple.rs]

Tuples are struct with unamed fields. Then `set<ordinal>()` is used as setter:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyTuple(
  i32,
  Option<String>,
  String,
);

let _mytuple: MyTuple = MyTuple::builder()
    .set0(42)
    .set2(String::from("foobar"))
    .build();
```

Only for `Typestate` mode, you can use `set()/none()`-calls to assign values in order:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyTuple(
  i32,
  Option<String>,
  String,
);
let _mytuple: MyTuple = MyTuple::builder()
    .set(42)
    .none()
    .set(String::from("foobar"))
    .build();
```

#### `Into` argument

Setter function argument is generic over [`Into`][Into] to ease conversion (especially for `&str`):

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyTuple(
  String,
);
let _mytuple: MyTuple = MyTuple::builder()
    .set("foobar")
    .build();
```

#### Implement `Into`

Builders implement [`Into`][Into] for target type (and reverse [`From`][From] also). Except for `Result` mode which uses [`TryInto`][TryInto] / [`TryFrom`][TryFrom].

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct MyStruct {
  value: String,
};
let _mytuple: MyStruct = MyStruct::builder()
    .value("foobar")
    .into();
```

#### `Option` fields

As their name suggests, [`Option`][Option] fields are facultative: you can build instance without setting them explicitly.

Setter argument are still generic over [`Into`][Into] but for wrapped type. No need to wrap into an [`Option`][Option]:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
struct WithOptional {
  mandatory: String,
  optional: Option<String>,
}

let built = WithOptional::builder()
  .optional("optional value")
  .mandatory("some value")
  .build();

assert_eq!(Some(String::from("optional value")), built.optional);
```

You can set them explicitly to [`None`][None] with `<field>_none`:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
pub struct WithOptional {
  mandatory: String,
  optional: Option<String>,
}

let built = WithOptional::builder()
  .optional_none()
  .mandatory("some value")
  .build();

assert_eq!(None, built.optional);
```

<div class="warning">

Note: In order to detect optional fields, field type **name** must match:

* `core::option::Option`
* `::core::option::Option`
* `std::option::Option`
* `::std::option::Option`
* `Option`

</div>

You can disable [`Option`][Option] support by using `#[builder(Option=!)]` at struct or field level:

```rust
#[macro_use] extern crate macon;

#[derive(Builder)]
#[builder(Option=!)]
struct DisableOptionStruct {
  optional: Option<String>,
}

let built = DisableOptionStruct::builder()
  .optional(Some(String::from("mandatory value")))
  .build();

assert_eq!(Some(String::from("mandatory value")), built.optional);
```

If you use an alias, use `#[builder(Option=<WrappedType>)]` at field level to enable [`Option`][Option] support:

```rust
#[macro_use] extern crate macon;

type OptString = Option<String>;
#[derive(Builder)]
struct AliasedOptionStruct {
  #[builder(Option=String)]
  optional: OptString,
}

let built = AliasedOptionStruct::builder()
  .optional("aliased value")
  .build();

assert_eq!(Some(String::from("aliased value")), built.optional);
```

[Default]: https://doc.rust-lang.org/core/default/trait.Default.html
[From]: https://doc.rust-lang.org/std/convert/trait.From.html
[Into]: https://doc.rust-lang.org/core/convert/trait.Into.html
[None]: https://doc.rust-lang.org/core/option/enum.Option.html#variant.None
[Option]: https://doc.rust-lang.org/core/option/enum.Option.html
[Result]: https://doc.rust-lang.org/std/result/enum.Result.html
[TryFrom]: https://doc.rust-lang.org/std/convert/trait.TryFrom.html
[TryInto]: https://doc.rust-lang.org/std/convert/trait.TryInto.html

[tests]: https://github.com/loganmzz/macon-rs/tree/main/tests
[blueprint_panic.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_panic.rs
[blueprint_panic_tuple.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_panic_tuple.rs
[blueprint_result.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_result.rs
[blueprint_result_tuple.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_result_tuple.rs
[blueprint_typestate.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_typestate.rs
[blueprint_typestate_tuple.rs]: https://github.com/loganmzz/macon-rs/blob/main/tests/blueprint_typestate_tuple.rs
