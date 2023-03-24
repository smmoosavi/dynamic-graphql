Add fields to an existing object.

It allows you to reverse the direction of dependency between objects. For example, you can add
fields to the `Query` object from the `user` module instead of defining user-related fields in the query module. it will help you to
avoid circular dependencies. Also, you can manage your code in the way you want, not the way forced by the schema.

ExpandObjectFields should be used with #[derive(ExpandObject)] derive macro.

## Macro Attributes

same as [`ResolvedObjectFields`] fields

| Attribute       | Description                                                                                                                                                                             | Type     |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `rename_fields` | Rename all the fields according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`.    | `String` |
| `rename_args`   | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |

## Field Attributes

same as [`ResolvedObjectFields`] fields

| Attribute     | Description                                                                                                                                                                             | Type     |
| ------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| `name`        | The name of the field                                                                                                                                                                   | `String` |
| `skip`        | Skip this field                                                                                                                                                                         | `bool`   |
| `deprecation` | Mark this field as a deprecated                                                                                                                                                         | `bool`   |
| `deprecation` | Mark this field as deprecated with the reason                                                                                                                                           | `String` |
| `rename_args` | Rename all the arguments according to the given case convention. The possible values are `lowercase`, `UPPERCASE`, `PascalCase`, `camelCase`, `snake_case`, and `SCREAMING_SNAKE_CASE`. | `String` |

## Argument Attributes

same as [`ResolvedObjectFields`] arguments

| Attribute | Description                     | Type     |
| --------- | ------------------------------- | -------- |
| `name`    | The name of the argument        | `String` |
| `ctx`     | Mark this argument as a context | `bool`   |

## Accepted Output and Arguments Types

Same as [`ResolvedObjectFields`]

## Example

### Basic

```rust

```
