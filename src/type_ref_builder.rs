use async_graphql::dynamic;

pub enum TypeRefBuilder {
    Named(String),
    NamedNN(String),
    List(String),
    ListNN(String),
    NNList(String),
    NNListNN(String),
}

impl TypeRefBuilder {
    pub fn optional(self) -> Self {
        match self {
            TypeRefBuilder::Named(name) => TypeRefBuilder::Named(name),
            TypeRefBuilder::NamedNN(name) => TypeRefBuilder::Named(name),
            TypeRefBuilder::List(name) => TypeRefBuilder::List(name),
            TypeRefBuilder::ListNN(name) => TypeRefBuilder::List(name),
            TypeRefBuilder::NNList(name) => TypeRefBuilder::NNList(name),
            TypeRefBuilder::NNListNN(name) => TypeRefBuilder::NNList(name),
        }
    }

    pub fn list(self) -> Self {
        match self {
            TypeRefBuilder::Named(name) => TypeRefBuilder::ListNN(name),
            TypeRefBuilder::NamedNN(name) => TypeRefBuilder::NNListNN(name),
            TypeRefBuilder::List(name) => TypeRefBuilder::List(name),
            TypeRefBuilder::ListNN(name) => TypeRefBuilder::ListNN(name),
            TypeRefBuilder::NNList(name) => TypeRefBuilder::NNList(name),
            TypeRefBuilder::NNListNN(name) => TypeRefBuilder::NNListNN(name),
        }
    }
}

impl From<TypeRefBuilder> for dynamic::TypeRef {
    fn from(value: TypeRefBuilder) -> Self {
        match value {
            TypeRefBuilder::Named(name) => dynamic::TypeRef::named(name),
            TypeRefBuilder::NamedNN(name) => dynamic::TypeRef::named_nn(name),
            TypeRefBuilder::List(name) => dynamic::TypeRef::named_list(name),
            TypeRefBuilder::ListNN(name) => dynamic::TypeRef::named_list_nn(name),
            TypeRefBuilder::NNList(name) => dynamic::TypeRef::named_nn_list(name),
            TypeRefBuilder::NNListNN(name) => dynamic::TypeRef::named_nn_list_nn(name),
        }
    }
}
