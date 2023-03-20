Define a new GraphQL object type.

Resolved objects fields should be used with [`#[derive(ResolvedObject)]`][ResolvedObject] derive macro.

## Macro Attributes

| Attribute       | Description                                                                                                                                                                             | Type     |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `rename_fields` | Rename all the fields according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`.    | `String` |
| `rename_args`   | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |

## Field Attributes

| Attribute       | Description                                                                                                                                                                             | Type     |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `name`          | The name of the field                                                                                                                                                                   | `String` |
| `skip`          | Skip this field                                                                                                                                                                         | `bool`   |
| `deprecation`   | Mark this field as a deprecated                                                                                                                                                         | `bool`   |
| `deprecation`   | Mark this field as deprecated with the reason                                                                                                                                           | `String` |
| `rename_args`   | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |

## Argument Attributes

| Attribute | Description                     | Type     |
|-----------|---------------------------------|----------|
| `name`    | The name of the argument        | `String` |
| `ctx`     | Mark this argument as a context | `bool`   |

## Accepted Output Types

- `String`, `&str`, [`ID`]
- `bool`
- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- `f32`, `f64`
- `Option<T>` where `T` is one of the valid output types
- `Vec<T>`, `&[T]` where `T` is one of the valid output types
- `Result<T, E>` where `T` is one of the valid output types
- [`Instance<dyn Trait>`][Instance] where `Trait` is marked by [`#[Interface]`][Interface]
- types defined by [`#[derive(SimpleObject)]`][SimpleObject]
- types defined by [`#[derive(ResolvedObject)]`][ResolvedObject]
- types defined by [`#[derive(Enum)]`][Enum]
- types defined by [`#[derive(Scalar)]`][Scalar]
- types defined by [`#[derive(Union)]`][Union]
- any type implements [`OutputTypeName`][internal::OutputTypeName] and [`ResolveRef`][internal::ResolveRef] traits if a reference is returned
- any type implements [`OutputTypeName`][internal::OutputTypeName] and [`ResolveOwned`][internal::ResolveOwned] traits if an owned value is returned

## Accepted Argument Types

- `String`, `&str`, [`ID`]
- `bool`
- `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- `f32`, `f64`
- `Option<T>` where `T` is one of the valid argument types
- [`MaybeUndefined<T>`][MaybeUndefined] where `T` is one of the valid argument types
- `Vec<T>` where `T` is one of the valid argument types
- [`Upload`] type
- [`Result<T>`][Result] where `T` is one of the valid argument types (except `Option<T>` and `MaybeUndefined<T>`, use `Option<Result<T>>` or `MaybeUndefined<Result<T>>` instead)
- types defined by [`#[derive(InputObject)]`][InputObject]
- types defined by [`#[derive(Enum)]`][Enum]
- types defined by [`#[derive(Scalar)]`][Scalar]
- any type implements [`InputTypeName`][internal::InputTypeName] and [`FromValue`][internal::FromValue] traits

## Example

### Basic

```rust
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(ResolvedObject)]
struct Foo {
    value: i32,
}

#[ResolvedObjectFields]
impl Foo {
    fn bar(&self) -> i32 {
        self.value
    }
}

#[derive(ResolvedObject)]
#[graphql(root)]
struct Query;

#[ResolvedObjectFields]
impl Query {
    async fn foo() -> Foo {
        Foo { value: 1 }
    }
}


#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
type Foo {
  bar: Int!
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
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(ResolvedObject)]
#[graphql(root)]
#[graphql(name = "RootQuery")]
struct Query;

#[ResolvedObjectFields]
#[graphql(rename_fields = "snake_case")]
#[graphql(rename_args = "PascalCase")]
impl Query {
    fn first_field(&self) -> i32 {
        1
    }
    
    #[graphql(name = "THE_SECOND")]
    fn second_field(&self) -> i32 {
        2
    }
    
    fn third_field(&self, #[graphql(name = "theArg")] arg: i32, _other_arg: i32) -> i32 {
        arg
    }
    
    #[graphql(rename_args = "snake_case")]
    fn fourth_field(&self, #[graphql(name = "theArg")] arg: i32, _other_arg: i32) -> i32 {
        arg
    }
}



#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
type RootQuery {
  first_field: Int!
  THE_SECOND: Int!
  third_field(theArg: Int!, OtherArg: Int!): Int!
  fourth_field(theArg: Int!, other_arg: Int!): Int!
}

schema {
  query: RootQuery
}
"#
);
```

### Skip, Deprecation, Description

```rust
use dynamic_graphql::{ResolvedObject, ResolvedObjectFields, App};

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

/// the query
#[derive(ResolvedObject)]
#[graphql(root)]
struct Query;

#[ResolvedObjectFields]
impl Query {
    #[graphql(skip)]
    fn first_field(&self) -> i32 {
        1
    }
    
    #[graphql(deprecation)]
    fn second_field(&self) -> i32 {
        2
    }

    #[graphql(deprecation = "the old one")]
    fn third_field(&self) -> i32 {
        3
    }
    
    /// the fourth field
    fn fourth_field(&self) -> i32 {
        4
    }
}


#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
"""
  the query
"""
type Query {
  secondField: Int! @deprecated
  thirdField: Int! @deprecated(reason: "the old one")
  """
    the fourth field
  """
  fourthField: Int!
}

schema {
  query: Query
}
"#
);
```

### Arguments

```rust
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::App;
use dynamic_graphql::Context;
use dynamic_graphql::dynamic;
use dynamic_graphql::FieldValue;
use dynamic_graphql::value;
use dynamic_graphql::dynamic::DynamicRequestExt;

# pub fn normalize_schema(sdl: &str) -> String {
#     format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[derive(ResolvedObject)]
#[graphql(root)]
struct Query {
    value: i32,
}

#[ResolvedObjectFields]
impl Query {
    fn simple_arg(arg: i32) -> i32 {
        arg
    }

    // Any argument named `ctx` or `_ctx` will be ignored in the schema, and the
    // graphql context will be passed to the function
    // Also, if any argument is marked with `#[graphql(ctx)]`, it will be ignored
    // in the schema, and the graphql context will be passed to the function
    // If you want to use `ctx` as an argument, you can use `#[graphql(name = "ctx")]`
    // to rename it in the schema
    fn with_context(
        _ctx: &Context,
        #[graphql(ctx)] _context: &Context,
        #[graphql(name = "ctx")] ctx_arg: i32,
    ) -> i32 {
        ctx_arg
    }

    // If you want to use `self` in the root query, you should call `root_value` function
    // on the request object. Only `&self` is supported
    fn with_self(&self, arg: i32) -> i32 {
        self.value + arg
    }
}

#[derive(App)]
struct App(Query);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
type Query {
  simpleArg(arg: Int!): Int!
  withContext(ctx: Int!): Int!
  withSelf(arg: Int!): Int!
}

schema {
  query: Query
}
"#
);

let query = r#"
query {
    withSelf(arg: 1)
}
"#;

let root = Query {
    value: 1,
};
let req = dynamic_graphql::Request::new(query).root_value(FieldValue::owned_any(root));
# let res =  tokio_test::block_on( async {
let res = schema.execute(req).await;
# res
# });
let data = res.data;

assert_eq!(
    data,
    value!({ "withSelf": 2 }),
);
```
