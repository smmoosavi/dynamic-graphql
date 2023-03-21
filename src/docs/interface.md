Define a new GraphQL interface type.

To define an interface, you need to define a rust trait and mark it with the `#[Interface]` attribute. then you
can mark object types with `#[graphql(mark(TraitName))]` or `#[graphql(impl(TraitName))]` attribute to implement the
interface in the GraphQL schema.

If you mark the object type with `#[graphql(mark(TraitName))]`, you need to implement fields yourself.

If you mark the object type with `#[graphql(impl(TraitName))]`, you should implement the trait for the object type and
fields will be resolved automatically.

you can use [`Instance<dyn TraitName>`][Instance] as the return type of the field to set the interface as the output type in the GraphQL
schema.

## Macro Attributes

| Attribute       | Description                                                                                                                                                                             | Type     |
|-----------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `name`          | The name of the interface                                                                                                                                                               | `String` |
| `rename_fields` | Rename all the fields according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`.    | `String` |
| `rename_args`   | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |
| `get_type_name` | If true, it allows the user to implement [`TypeName`][internal::TypeName] trait                                                                                                         | `bool`   |
| `register`      | Register type                                                                                                                                                                           | `Path`   |
| `auto_register` | Register types for each instance                                                                                                                                                        | `Path`   |

## Field Attributes

same as [`ResolvedObjectFields`] fields

| Attribute     | Description                                                                                                                                                                             | Type     |
|---------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `name`        | The name of the field                                                                                                                                                                   | `String` |
| `skip`        | Skip this field                                                                                                                                                                         | `bool`   |
| `deprecation` | Mark this field as a deprecated                                                                                                                                                         | `bool`   |
| `deprecation` | Mark this field as deprecated with the reason                                                                                                                                           | `String` |
| `rename_args` | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |

## Argument Attributes

same as [`ResolvedObjectFields`] arguments

| Attribute | Description                     | Type     |
|-----------|---------------------------------|----------|
| `name`    | The name of the argument        | `String` |
| `ctx`     | Mark this argument as a context | `bool`   |

## Accepted Output and Arguments Types

Same as [`ResolvedObjectFields`]

## Example

### Basic

```rust
use dynamic_graphql::Interface;
use dynamic_graphql::SimpleObject;
use dynamic_graphql::ResolvedObject;
use dynamic_graphql::ResolvedObjectFields;
use dynamic_graphql::Instance;
use dynamic_graphql::App;

# pub fn normalize_schema(sdl: &str) -> String {
#   format!("\n{}", graphql_parser::schema::parse_schema::<String>(sdl).unwrap().to_owned())
# }

#[Interface]
trait Character {
    fn id(&self) -> String;
    fn name(&self) -> String;
}

// Character trait is not implemented for the Human struct, so it can be marked with
// `#[graphql(mark(Character))]` and interface fields should be resolved manually.

#[derive(SimpleObject)]
#[graphql(mark(Character))]
struct Human {
    // id and name are defined here, they should be exactly the same as the fields
    // defined in the Character interface.
    id: String,
    name: String,
    age: i32,
}

// Character trait is implemented for Droid, so it can be marked with
// `#[graphql(impl(Character))]` and interface fields will be resolved automatically.

#[derive(ResolvedObject)]
#[graphql(impl(Character))]
struct Droid {
    id: String,
    name: String,
    power: i32,
}

#[ResolvedObjectFields]
impl Droid {
    fn power(&self) -> i32 {
        self.power
    }
}

impl Character for Droid {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(ResolvedObject)]
#[graphql(root)]
struct Query;

#[ResolvedObjectFields]
impl Query {
    async fn character(&self, id: String) -> Option<Instance<dyn Character>> {
        if id == "1" {
            Some(Instance::new_owned(Human {
                id: "1".to_string(),
                name: "Luke".to_string(),
                age: 20,
            }))
        } else if id == "2" {
            Some(Instance::new_owned(Droid {
                id: "2".to_string(),
                name: "R2-D2".to_string(),
                power: 100,
            }))
        } else {
            None
        }
    }
}


// Human and Droid types never appeared in the Query type signature, so they should be
// registered manually.
#[derive(App)]
struct App(Query, Human, Droid);

let schema = App::create_schema().finish().unwrap();

assert_eq!(
    normalize_schema(&schema.sdl()),
    r#"
interface Character {
  id: String!
  name: String!
}

type Droid implements Character {
  power: Int!
  id: String!
  name: String!
}

type Human implements Character {
  id: String!
  name: String!
  age: Int!
}

type Query {
  character(id: String!): Character
}

schema {
  query: Query
}
"#
);
```
