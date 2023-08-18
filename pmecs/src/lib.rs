use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type Entity = usize;

struct ComponentStorage<T: Send + Sync> {
    data: RwLock<HashMap<Entity, T>>,
}

impl<T: Send + Sync> ComponentStorage<T> {
    fn new() -> Self {
        ComponentStorage {
            data: RwLock::new(HashMap::new()),
        }
    }

    // todo: methods for manipulating components
}

pub struct Storage {
    components: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            components: HashMap::new(),
        }
    }


    pub fn insert<T: 'static + Send + Sync>(&mut self, entity: Entity, component: T) {
        let storage_entry = self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(|| Box::new(ComponentStorage::<T>::new()));

        if let Some(storage) = storage_entry.downcast_mut::<ComponentStorage<T>>() {
            storage.data.write().unwrap().insert(entity, component);
        }
    }

    pub fn get<T: 'static + Send + Sync>(&self, entity: Entity) -> Option<RwLockReadGuard<HashMap<Entity, T>>> {
        self.components.get(&TypeId::of::<T>())
            .and_then(|storage| storage.downcast_ref::<ComponentStorage<T>>())
            .map(|s| s.data.read().unwrap())
            .and_then(|guard| if guard.contains_key(&entity) { Some(guard) } else { None })
    }

    pub fn get_mut<T: 'static + Send + Sync>(&self, entity: Entity) -> Option<RwLockWriteGuard<HashMap<Entity, T>>> {
        self.components.get(&TypeId::of::<T>())
            .and_then(|storage| storage.downcast_ref::<ComponentStorage<T>>())
            .map(|s| s.data.write().unwrap())
            .and_then(|guard| if guard.contains_key(&entity) { Some(guard) } else { None })
    }

    // todo: additional methods for removal, querying, etc
}

pub struct World {
    next_entity: Entity,
    storage: Storage,
    current_id: usize,
}


impl World {
    pub fn new() -> Self {
        World {
            next_entity: 0,
            storage: Storage::new(),
            current_id:0,
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity = self.next_entity;
        self.next_entity += 1;
        entity
    }

    pub fn add_component<T: 'static + Send + Sync>(&mut self, entity: Entity, component: T) {
        self.storage.insert(entity, component);
    }

    pub fn get_component<T: 'static + Send + Sync>(&self, entity: Entity) -> Option<RwLockReadGuard<HashMap<Entity, T>>> {
        self.storage.get(entity)
    }

    pub fn get_component_mut<T: 'static + Send + Sync>(&mut self, entity: Entity) -> Option<RwLockWriteGuard<HashMap<Entity, T>>> {
        self.storage.get_mut(entity)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_entity_test() {
        let mut world = World::new();
        let e1 = world.create_entity();
        let e2 = world.create_entity();
        assert_eq!(e1, 0);
        assert_eq!(e2, 1);
    }

    #[test]
    fn add_and_get_component_test() {
        let mut world = World::new();
        let e = world.create_entity();

        world.add_component(e, "TestComponent");

        let comp = world.get_component::<&str>(e).unwrap();
        assert_eq!(*comp.get(&e).unwrap(), "TestComponent");
    }

    #[test]
    fn add_and_get_mut_component_test() {
        let mut world = World::new();
        let e = world.create_entity();

        world.add_component(e, "TestComponent");

        {
            let mut comp = world.get_component_mut::<&str>(e).unwrap();
            *comp.get_mut(&e).unwrap() = "ChangedComponent";
        }

        let comp = world.get_component::<&str>(e).unwrap();
        assert_eq!(*comp.get(&e).unwrap(), "ChangedComponent");
    }

    #[test]
    fn multiple_component_test() {
        let mut world = World::new();
        let e = world.create_entity();

        world.add_component(e, "StringComponent");
        world.add_component(e, 42i32);

        let comp_str = world.get_component::<&str>(e).unwrap();
        let comp_int = world.get_component::<i32>(e).unwrap();

        assert_eq!(*comp_str.get(&e).unwrap(), "StringComponent");
        assert_eq!(*comp_int.get(&e).unwrap(), 42);
    }

    #[test]
    fn no_component_test() {
        let mut world = World::new();
        let e = world.create_entity();

        // No component was added for this entity
        let comp = world.get_component::<&str>(e);
        assert!(comp.is_none());
    }
}






