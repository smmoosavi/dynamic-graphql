# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.3.0...main
[0.3.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.2.0...0.3.0
[0.2.0]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.0...v0.1.1
