use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::rc::Rc;
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use std::intrinsics;

use bs_audio::AudioSource;

use ecs::{EntityManager, ComponentManager};
use input::Input;
use super::component::{TransformManager, CameraManager, MeshManager, LightManager, AudioSourceManager};
use resource::ResourceManager;

/// Contains all the data that defines the current state of the world.
///
/// This is passed into systems in System::update(). It can be used access component
/// managers and input.
#[derive(Debug)]
pub struct Scene {
    pub entity_manager: EntityManager,
    component_managers: HashMap<TypeId, Box<Any>>,
    /// This value is only needed in debug builds.
    manager_id_by_name: HashMap<String, TypeId>,
    pub input: Input,
    pub audio_source: AudioSource,
}

impl Scene {
    pub fn new(resource_manager: &Rc<RefCell<ResourceManager>>, audio_source: AudioSource) -> Scene {
        let mut scene = Scene {
            entity_manager: EntityManager::new(),
            component_managers: HashMap::new(),
            manager_id_by_name: HashMap::new(),
            input: Input::new(),
            audio_source: audio_source,
        };

        scene.register_manager(Box::new(TransformManager::new()));
        scene.register_manager(Box::new(CameraManager::new()));
        scene.register_manager(Box::new(MeshManager::new(resource_manager.clone())));
        scene.register_manager(Box::new(LightManager::new()));
        scene.register_manager(Box::new(AudioSourceManager::new(resource_manager.clone())));

        scene
    }

    pub fn register_manager<T: Any + ComponentManager>(&mut self, manager: Box<T>) {
        let manager_id = TypeId::of::<T>();
        assert!(!self.component_managers.contains_key(&manager_id),
                "Manager {} with ID {:?} already registered", type_name::<T>(), manager_id);

        self.component_managers.insert(manager_id, manager);

        // TODO: Only do this when hotloading support is enabled.
        self.manager_id_by_name.insert(type_name::<T>().into(), manager_id);
    }

    pub fn get_manager<T: Any + ComponentManager>(&self) -> &T {
        let manager_id = TypeId::of::<T>();
        let manager = self.component_managers
            .get(&manager_id)
            .expect(&format!("Tried to retrieve manager {} with ID {:?} but none exists", type_name::<T>(), manager_id));
        manager.deref().downcast_ref().unwrap()
    }

    pub fn get_manager_mut<T: Any + ComponentManager>(&mut self) -> &mut T {
        let manager_id = TypeId::of::<T>();
        let mut manager = self.component_managers
            .get_mut(&manager_id)
            .expect(&format!("Tried to retrieve manager {} with ID {:?} but none exists", type_name::<T>(), manager_id));
        manager.deref_mut().downcast_mut().unwrap()
    }

    // pub fn remove_manager<T: Any + ComponentManager>(&mut self) -> ManagerHandle<T> {
    //     let manager_id = TypeId::of::<T>();
    //     let manager = self.component_managers
    //         .remove(&manager_id)
    //         .expect(&format!("Tried to remove manager {} with ID {:?} but none exists", type_name::<T>(), manager_id));
    //
    //     // TODO: Only do this when hotloading is enabled.
    //     self.manager_id_by_name.remove(type_name::<T>().into());
    //
    //     ManagerHandle::new(manager)
    // }
    //
    // /// TODO: This is only needed if hotloading is enabled.
    // pub fn remove_by_name<T: Any + ComponentManager>(&mut self) -> ManagerHandle<T> {
    //     let manager_id = self.manager_id_by_name
    //         .remove(type_name::<T>().into())
    //         .expect(&format!("Tried to remove manager {} by name but none exists", type_name::<T>()));
    //
    //     let manager = self.component_managers
    //         .remove(&manager_id)
    //         .expect(&format!("Tried to remove manager {} with ID {:?} but none exists", type_name::<T>(), manager_id));
    //
    //     ManagerHandle::new(manager)
    // }
    //
    // /// TODO: This is only needed if hotloading is enabled.
    // pub fn reload_internal_managers(&mut self) {
    //     let transform_handle = self.remove_by_name::<TransformManager>();
    //     let new_transform = *transform_handle.get();
    //
    //     // scene.register_manager(Box::new(TransformManager::new()));
    //     // scene.register_manager(Box::new(CameraManager::new()));
    //     // scene.register_manager(Box::new(MeshManager::new(resource_manager.clone())));
    //     // scene.register_manager(Box::new(LightManager::new()));
    //     // scene.register_manager(Box::new(AudioSourceManager::new(resource_manager.clone())));
    // }
}

fn type_name<T>() -> &'static str {
    unsafe {
        intrinsics::type_name::<T>()
    }
}
