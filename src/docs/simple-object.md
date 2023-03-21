Define a new GraphQL object type.

To implement a graphql object, you need to define a rust struct and mark it with the `#[derive(SimpleObject)]`
attribute. `Simpleobject`s are used to represent simple object types in GraphQL, which means it doesn't have any
arguments or context, or resolve functions andsimply define object based on struct fields.

## Macro Attributes

| Attribute       | Description                                                                                                                                                                          | Type     |
|-----------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `root`          | Mark this type as a root query.                                                                                                                                                      | `bool`   |
| `name`          | The name of the object                                                                                                                                                               | `String` |
| `rename_fields` | Rename all the fields according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |
| `mark`          | Mark the object as implement Interface, all interface fields should be defined manually                                                                                              | `Path`   |
| `impl`          | Mark the object as implement Interface, the interface trait should be implemented                                                                                                    | `Path`   | 
| `get_type_name` | If true, it allows the user to implement [`TypeName`][internal::TypeName] trait                                                                                                      | `bool`   |
| `register`      | Register other types that implement [`Register`][internal::Register] trait                                                                                                           | `Path`   |

## Field Attributes

| Attribute       | Description                                   | Type     |
|-----------------|-----------------------------------------------|----------|
| `name`          | The name of the field                         | `String` |
| `skip`          | Skip this field                               | `bool`   |
| `deprecation`   | Mark this field as a deprecated               | `bool`   |
| `deprecation`   | Mark this field as deprecated with the reason | `String` |

## Accepted Output Types

- `String`, `&str`, [`ID`]
- `bool`
- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- `f32`, `f64`
- `Option<T>` where `T` is one of the valid output types
- `Vec<T>` where `T` is one of the valid output types
- types defined by [`#[derive(SimpleObject)]`][SimpleObject]
- types defined by [`#[derive(ResolvedObject)]`][ResolvedObject]
- types defined by [`#[derive(Enum)]`][Enum]
- types defined by [`#[derive(Scalar)]`][Scalar]
- types defined by [`#[derive(Union)]`][Union]
- any type implements [`OutputTypeName`][internal::OutputTypeName] and [`ResolveRef`][internal::ResolveRef] traits

## Examples

### Basic

```rust
use dynamic_graphql::{SimpleObject, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(SimpleObject)]
struct Foo {
    value: String,
}


#[derive(SimpleObject)]
#[graphql(root)]
struct Query {
    foo: Foo,
}

#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
type Foo {
  value: String!
}

type Query {
  foo: Foo!
}

schema {
  query: Query
}
"#
);
```

### Rename

```rust
use dynamic_graphql::{SimpleObject, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(SimpleObject)]
#[graphql(name = "RootQuery")]
#[graphql(root, rename_fields = "snake_case")]
struct Query {
    hello_world: String,
    #[graphql(name = "MyField")]
    other_field: String,
}

#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
type RootQuery {
  hello_world: String!
  MyField: String!
}

schema {
  query: RootQuery
}
"#
);
```

### Skip, Deprecation, Description

```rust
use dynamic_graphql::{SimpleObject, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(SimpleObject)]
#[graphql(root)]
/// This is my object
struct Query {
    #[graphql(skip)]
    hello_world: String,
    #[graphql(deprecation)]
    deprecated_field: String,
    
    #[graphql(deprecation = "this is the old one")]
    with_reason: String,
    
    /// This is my field
    my_field: String,
}

#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
"""
  This is my object
"""
type Query {
  deprecatedField: String! @deprecated
  withReason: String! @deprecated(reason: "this is the old one")
  """
    This is my field
  """
  myField: String!
}

schema {
  query: Query
}
"#
);
```