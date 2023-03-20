Define a new GraphQL object type.

Resolved objects should be used with [`#[ResolvedObjectFields]`][ResolvedObjectFields] macro.

## Macro Attributes

| Attribute       | Description                                                                             | Type     |
|-----------------|-----------------------------------------------------------------------------------------|----------|
| `root`          | Mark this type as a root query.                                                         | `bool`   |
| `name`          | The name of the object                                                                  | `String` |
| `mark`          | Mark the object as implement Interface, all interface fields should be defined manually | `Path`   |
| `impl`          | Mark the object as implement Interface, the interface trait should be implemented       | `Path`   | 
| `get_type_name` | If true, it allows the user to implement [`TypeName`][internal::TypeName] trait         | `bool`   |
| `register`      | Register other types that implement [`Register`][internal::Register] trait              | `Path`   |
