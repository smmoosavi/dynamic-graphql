use std::any::TypeId;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::mem;

use crate::data::SchemaData;
use crate::dynamic;
use crate::types::Register;

pub struct Registry {
    pub data: SchemaData,
    root: Option<String>,
    mutation: Option<String>,
    subscription: Option<String>,
    objects: HashMap<String, dynamic::Object>,
    types: Vec<dynamic::Type>,
    // name of all registered types
    names: HashSet<TypeId>,
    pending_expand_objects: VecDeque<PendingExpandObject>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Self {
        Self {
            data: Default::default(),
            root: None,
            mutation: None,
            subscription: None,
            objects: Default::default(),
            types: Default::default(),
            names: Default::default(),
            pending_expand_objects: Default::default(),
        }
    }
}

struct PendingExpandObject {
    target: String,
    expansion: String,
    map_fn: Box<dyn FnOnce(dynamic::Object) -> dynamic::Object>,
}

impl Registry {
    #[inline]
    pub fn set_root(mut self, name: &str) -> Self {
        self.root = Some(name.to_string());
        self
    }
    #[inline]
    pub fn set_mutation(mut self, name: &str) -> Self {
        self.mutation = Some(name.to_string());
        self
    }
    #[inline]
    pub fn set_subscription(mut self, name: &str) -> Self {
        self.subscription = Some(name.to_string());
        self
    }
    pub fn register_type(mut self, ty: impl Into<dynamic::Type>) -> Self {
        let ty = ty.into();
        match ty {
            dynamic::Type::Object(object) => {
                self.objects.insert(object.type_name().to_string(), object);
            }
            _ => {
                self.types.push(ty);
            }
        }
        self
    }
    pub fn update_object<F>(mut self, target: &str, expansion_name: &str, f: F) -> Self
    where
        F: FnOnce(dynamic::Object) -> dynamic::Object + 'static,
    {
        self.pending_expand_objects.push_back(PendingExpandObject {
            target: target.to_string(),
            expansion: expansion_name.to_string(),
            map_fn: Box::new(f),
        });
        self
    }
}

impl Registry {
    pub fn register<T: Register + ?Sized + 'static>(mut self) -> Self {
        let ty = TypeId::of::<T>();
        if self.names.contains(&ty) {
            return self;
        }
        self.names.insert(ty);
        T::register(self)
    }

    fn apply_pending_objects(&mut self) {
        loop {
            if self.pending_expand_objects.is_empty() {
                break;
            }
            let mut changed = false;
            let pending_expand_objects = mem::take(&mut self.pending_expand_objects);
            self.pending_expand_objects = pending_expand_objects
                .into_iter()
                .filter_map(|pending| {
                    if let Some(object) = self.objects.remove(&pending.target) {
                        self.objects
                            .insert(pending.target, (pending.map_fn)(object));
                        changed = true;
                        None
                    } else {
                        Some(pending)
                    }
                })
                .collect();
            if !changed {
                let keys = self
                    .pending_expand_objects
                    .iter()
                    .map(|p| format!("{} when defining {}", p.target, p.expansion))
                    .collect::<Vec<_>>()
                    .join(", ");
                panic!("Can't find object: {:?}", keys);
            }
        }
    }
    pub fn create_schema(self) -> dynamic::SchemaBuilder {
        let Some(ref root) = self.root else {
            panic!("No root object defined");
        };
        let schema =
            dynamic::Schema::build(root, self.mutation.as_deref(), self.subscription.as_deref());
        self.apply_into_schema_builder(schema)
    }

    pub fn apply_into_schema_builder(
        mut self,
        schema: dynamic::SchemaBuilder,
    ) -> dynamic::SchemaBuilder {
        self.apply_pending_objects();
        let schema = self
            .objects
            .into_iter()
            .fold(schema, |schema, (_, object)| schema.register(object));
        let schema = self
            .types
            .into_iter()
            .fold(schema, |schema, object| schema.register(object));
        schema.data(self.data)
    }
}
