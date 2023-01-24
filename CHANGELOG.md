# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres
to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.1...main
[0.1.1]: https://github.com/smmoosavi/dynamic-graphql/compare/v0.1.0...v0.1.1
