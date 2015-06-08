use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use scene::Scene;
use ecs::{Entity, ComponentManager, System};
use resource::ResourceManager;
use wav::Wave;

pub struct AudioSource {
    audio_clip: Rc<Wave>,
    offset:     usize,
    is_playing: bool,
    looping:    bool,
}

impl AudioSource {
    /// Start playing the current audio clip from where it left off.
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause the clip without resetting it to the beginning.
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Stop the current audio clip and reset it to the beginning.
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.offset = 0;
    }

    /// Reset the audio clip the start without stoping it.
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Retrieve whether the audio clip is currently playing.
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

pub struct AudioSourceManager {
    resource_manager: Rc<RefCell<ResourceManager>>,
    audio_sources:    Vec<AudioSource>,
    entities:         Vec<Entity>,
    indices:          HashMap<Entity, usize>,
}

impl AudioSourceManager {
    pub fn new(resource_manager: Rc<RefCell<ResourceManager>>) -> AudioSourceManager {
        AudioSourceManager {
            resource_manager: resource_manager,
            audio_sources:    Vec::new(),
            entities:         Vec::new(),
            indices:          HashMap::new(),
        }
    }

    pub fn assign(&mut self, entity: Entity, clip_name: &str) -> &mut AudioSource {
        assert!(!self.indices.contains_key(&entity));

        let mut resource_manager = self.resource_manager.borrow_mut();
        let audio_clip = resource_manager.get_audio_clip(clip_name);
        let index = self.audio_sources.len();
        self.audio_sources.push(AudioSource {
            audio_clip: audio_clip,
            offset:     0,
            is_playing: false,
            looping:    false,
        });
        self.entities.push(entity);
        self.indices.insert(entity, index);

        &mut self.audio_sources[index]
    }

    pub fn get(&mut self, entity: Entity) -> &AudioSource {
        assert!(self.indices.contains_key(&entity));

        let index = *self.indices.get(&entity).unwrap();
        &self.audio_sources[index]
    }

    pub fn get_mut(&mut self, entity: Entity) -> &mut AudioSource {
        assert!(self.indices.contains_key(&entity));

        let index = *self.indices.get(&entity).unwrap();
        &mut self.audio_sources[index]
    }
}

impl ComponentManager for AudioSourceManager {
}

pub struct AudioSystem;

impl System for AudioSystem {
    fn update(&mut self, scene: &mut Scene, delta: f32) {
        // let audio_source_manager = scene.get_manager::<AudioSourceManager>();
        //
        // let mut audio_sources = &mut audio_source_manager.audio_sources;
        // // TODO: Use a better method to filter out audio sources that aren't playing.
        // for audio_source in audio_sources.iter_mut().filter(|audio_source| audio_source.is_playing) {
        //     // Create an iterator over the samples using the data from the audio clip.
        //     let total_samples = {
        //         let mut stream = audio_source.audio_clip.data.samples[audio_source.offset..].iter()
        //             .map(|sample| *sample);
        //
        //         // Sream the samples to the audio card.
        //         let samples_written = scene.audio_source.stream(&mut stream, delta);
        //
        //         // Determine if we're done playing the clip yet.
        //         audio_source.offset + samples_written
        //     };
        //     if total_samples >= audio_source.audio_clip.data.samples.len() {
        //         audio_source.offset = 0;
        //
        //         if !audio_source.looping {
        //             audio_source.stop();
        //         }
        //     } else {
        //         audio_source.offset = total_samples;
        //     }
        // }
    }
}
