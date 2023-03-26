# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add `#[graphql(validator(validate_fn))]` attribute to validate scalar value

```rust
#[derive(Scalar)]
#[graphql(validator(validate_foo))]
struct Foo(String);

fn validate_foo(value: &Value) -> bool {
    match value {
        Value::String(s) => s.len() <= 5,
        _ => false,
    }
}
```

### Doc

- add docs for `Interface` macro

## [0.7.2] - 2023-03-20

### Added

- add `Registry::set_subscription` method

### Doc

- add docs for `SimpleObject` macro
- add docs for `ResolvedObject` and `ResolvedObjectFields` macro

## [0.7.1] - 2023-03-04

### Internal

- add `Registry::apply_into_schema_builder` method

## [0.7.0] - 2023-02-21

### Added

- add Scalar support

```rust
#[derive(Scalar)]
struct MyScalar {
    value: String
}

impl ScalarValue for MyScalar {
    fn from_value(value: dynamic_graphql::Value) -> dynamic_graphql::Result<Self> {
        match value {
            dynamic_graphql::Value::String(value) => Ok(MyScalar { value }),
            _ => Err(dynamic_graphql::Error::new("Invalid value")),
        }
    }

    fn to_value(&self) -> dynamic_graphql::Value {
        dynamic_graphql::Value::String(self.value.clone())
    }
}
```

- support `Result` as input type

### Fixed

- fix silent integer overflow cast in `FromValue` trait. Now it returns error if value is out of range.

### Internal

- move internal types to `internal` module
- simplify `GetOutputTypeRef`/`GetInputTypeRef` signatures
- change signature of `FromValue` trait. Now it returns `InputValueResult<Self>`

## [0.6.1] - 2023-02-08

### Added

- Support generics in `Union` types

## [0.6.0] - 2023-02-08

### Added

- add `#[graphql(get_type_name)` attribute to override type name

```rust
use std::borrow::Cow;

#[derive(SimpleObject)]
#[graphql(get_type_name)]
struct Foo {
    value: String
}

impl TypeName for Foo {
    fn get_type_name() -> Cow<'static, str> {
        "Bar".into()
    }
}
```

### Internal

- remove `MARK` from `Interface` trait
- use function instead constant for type names
- rename `GraphqlType` to `TypeName`
- rename `InputType` to `InputTypeName`
- rename `OutputType` to `OutputTypeName`

## [0.5.4] - 2023-01-30

- remove `.parent()` from expand object
- improve lifetimes for `ExpandObjectFields`

## [0.5.3] - 2023-01-30

### Internal

- Improve `Register`, `GraphqlType`, `OutputType` for refs

## [0.5.2] - 2023-01-30

### Internal

- add `Resolve` trait to unify `ResolveOwned` and `ResolveRef`

## [0.5.1] - 2023-01-29

### Fixed

- dependency `dynamic-graphql-derive` version

## [0.5.0] - 2023-01-29

### Added

- Add `#[graphql(register())]` attribute to register types manually

```rust
#[derive(SimpleObject)]
struct Foo { value: String }

#[derive(SimpleObject)]
#[graphql(register(Foo))] // also register `Foo` type when Example is registered
struct Example { value: String }
```

- Add `#[graphql(auto_register())]` attribute to register types automatically for each instance of interface

```rust
/// call `registry.register::<Foo<T>>()` for each instance of `Node` (T)
#[Interface]
#[graphql(auto_register(Foo))]
trait Node {
    fn id(&self) -> String;
}
```

- Add schema `data` to share data between schema definitions and execution time

```rust
// schema definition time
fn register(mut registry: Registry) -> Registry {
    let my_data: &mut MyStruct = registry.data.get_mut_or_default();
}
// execution time
fn some_fn(ctx: &Context<'_>){
    let my_data = ctx.get_schema_data().get::<MyStruct>(); // Option<&MyStruct>
}
```

### Internal

- Remove the `InterfaceTarget` trait

### [0.4.0] - 2023-01-28

### Added

- Automatic register used types.

```rust
#[derive(SimpleObject)]
struct Example { value: String }

#[derive(SimpleObject)]
struct Query { example: Example }

#[derive(App)]
struct App (Query); // no need to register Example manually
```

### Changed

- force `'static` lifetime for `#[derive(App)]` attribute

```rust
// old
#[derive(App)]
struct ExampleApp<'a>(ExampleQuery<'a>);
```

```rust
// new
#[derive(App)]
struct ExampleApp(ExampleQuery<'static>);
```

### Breaking Changes

- remove support for mark object as interface with string `#[graphql(mark("Node"))]`
- The way of defining the interface is changed. No need to define a new name (e.g., `NodeInterface`) for the interface and
use it in `#[graphql(mark(NodeInterface))]` and `#[graphql(impl(NodeInterface))]` attributes. Now you can use
`#[graphql(mark(Node))]` and `#[graphql(impl(Node))]` attributes.

```rust
// old
#[Interface(NodeInterface)]
trait Node {
    fn id(&self) -> String;
}

#[derive(SimpleObject)]
#[graphql(mark(NodeInterface))]
struct Foo { id: String }

#[derive(SimpleObject)]
#[graphql(impl(NodeInterface))]
struct Bar;

impl Node for Bar {
    fn id(&self) -> String {
        "bar".to_string()
    }
}

#[derive(ResolvedObject)]
struct Query;

#[ResolvedObjectFields]
impl Query {
    async fn node(&self, id: String) -> NodeInterface {
        NodeInterface::new_owned(Foo { id })
    }
}
```

```rust
// new
#[Interface]
trait Node {
    fn id(&self) -> String;
}

#[derive(SimpleObject)]
#[graphql(mark(Node))]
struct Foo {
    id: String,
}

#[derive(SimpleObject)]
#[graphql(impl(Node))]
struct Bar;

impl Node for Bar {
    fn id(&self) -> String {
        "bar".to_string()
    }
}

#[derive(ResolvedObject)]
struct Query;

#[ResolvedObjectFields]
impl Query {
    async fn node(&self, id: String) -> Instance<dyn Node> {
        Instance::new_owned(Foo { id })
    }
}
```

### Internal

- every `GraphQLType` now should implement `Register` trait
- remove `InterfaceRoot`
- add `Instance` struct, `RegisterInstance` trait
- remove constraint `Sized` from `T` in `Register<T>`
- significant changes in `InterfaceMark` trait

### [0.3.0] - 2023-01-25

### Added

- support `Upload` type

### Fixed

- fix `remote` in enum to accept path with `::` separator

### Breaking Change

- `remote` in enum now defined as `graphql(remote(path::to::Other))` instead of `graphql(remote = "path::to::Other")`

```rust
// old
#[derive(Enum)]
#[graphql(remote = "path::to::Other")]
enum MyEnum {}
```

```rust
// new
#[derive(Enum)]
#[graphql(remote(path::to::Other))]
enum MyEnum {}
```

## [0.2.0] - 2023-01-25

### Changed

- `SimpleObject`, `ResolvedObject`: change `graphql(mark_as=)`, `graphql(mark_with=)`, `graphql(implement=)` to `graphql(mark())` and `graphql(impl())`
- change `async-graphql` dependency to `5.0.5'

### Breaking Change

before this release `SimpleObject`, `ResolvedObject` can implement interfaces this way:

```rust
#[derive(SimpleObject)]
#[graphql(mark_as = "Node")]
struct MyType {}

#[derive(SimpleObject)]
#[graphql(mark_with = "NodeInterface")]
struct OtherType {}

#[derive(SimpleObject)]
#[graphql(implement = "NodeInterface")]
struct AnotherType {}
```

after this release `SimpleObject`, `ResolvedObject` can implement interfaces this way:

```rust
#[derive(SimpleObject)]
#[graphql(mark("Node"))]
struct MyType {}

#[derive(SimpleObject)]
#[graphql(mark(NodeInterface))]
struct OtherType {}

#[derive(SimpleObject)]
#[graphql(impl(NodeInterface))]
struct AnotherType {}
```

## [0.1.1] - 2023-01-24

### Added

- support for `MaybeUndefined` input type

### Fixed

- fix error when argument or optional input field is not provided
- fix error when `ResolvedObjectFields` and `ExpandedObjectFields` are used on impl path (e.g. `impl other::MyType`)

### Internal

- remove `GraphqlDoc` trait
- change `FromValue` argument to `Result<dynamic::ValueAccessor>`
- add `Output` associated type to `GetOutputTypeRef` and `GetInputTypeRef`

[unreleased]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.7.2...main
[0.7.2]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.7.1...v0.7.1
[0.7.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.5.4...v0.6.0
[0.5.4]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.5.3...v0.5.4
[0.5.3]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.5.2...v0.5.3
[0.5.2]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.5.1...v0.5.2
[0.5.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.0...v0.1.1
