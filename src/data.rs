use std::any::Any;
use std::any::TypeId;

use async_graphql::Context;
use fnv::FnvHashMap;

pub struct SchemaData(FnvHashMap<TypeId, Box<dyn Any + Sync + Send>>);

impl SchemaData {
    pub fn new() -> Self {
        Self(Default::default())
    }
}

impl Default for SchemaData {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaData {
    pub fn insert<T: Any + Sync + Send>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }
    pub fn get<T: Any + Sync + Send>(&self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|v| v.downcast_ref::<T>())
    }
    pub fn get_mut<T: Any + Sync + Send>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .and_then(|v| v.downcast_mut::<T>())
    }
    pub fn get_or_default<T: Any + Sync + Send + Default>(&mut self) -> &mut T {
        self.0
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::<T>::default())
            .downcast_mut()
            .unwrap()
    }
    pub fn get_mut_or_default<T: Any + Sync + Send + Default>(&mut self) -> &mut T {
        self.0
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::<T>::default())
            .downcast_mut()
            .unwrap()
    }
}

pub trait GetSchemaData {
    fn get_schema_data(&self) -> &SchemaData;
}

impl GetSchemaData for Context<'_> {
    fn get_schema_data(&self) -> &SchemaData {
        self.data_unchecked()
    }
}
