use bevy::{prelude::*, reflect::ReflectRef, utils::HashMap};
use regex::Regex;

#[derive(Component)]
pub struct SpriteAnimation {
    next_frame_seconds: f32,
    time_passed: f32,
    current_frame: usize,
    indices: Vec<usize>,
}

pub fn sprite_animation_system(
    mut sprite_animation_query: Query<(&mut SpriteAnimation, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (mut sprite_animation, mut sprite) in sprite_animation_query.iter_mut() {
        sprite_animation.time_passed += time.delta_seconds();
        if sprite_animation.time_passed > sprite_animation.next_frame_seconds {
            let frames_forward = (sprite_animation.time_passed / sprite_animation.next_frame_seconds) as usize;
            sprite_animation.current_frame = (sprite_animation.current_frame + frames_forward) % sprite_animation.indices.len();
            sprite.index = sprite_animation.indices[sprite_animation.current_frame];
            sprite_animation.time_passed -= frames_forward as f32 * sprite_animation.next_frame_seconds;
        }
    }
}


pub struct SpriteAnimationStoreItem {
    next_frame_seconds: f32,
    indices: Vec<usize>,
}
impl From<SpriteAnimationStoreItem> for SpriteAnimation {
    fn from(sprite_animation: SpriteAnimationStoreItem) -> Self {
        SpriteAnimation {
            next_frame_seconds: sprite_animation.next_frame_seconds,
            time_passed: 0.0,
            current_frame: 0,
            indices: sprite_animation.indices,
        }
    }
}
impl From<&SpriteAnimationStoreItem> for SpriteAnimation {
    fn from(sprite_animation: &SpriteAnimationStoreItem) -> Self {
        SpriteAnimation {
            next_frame_seconds: sprite_animation.next_frame_seconds,
            time_passed: 0.0,
            current_frame: 0,
            indices: sprite_animation.indices.clone(),
        }
    }
}

#[derive(Default)]
pub struct SpriteAnimationStore {
    animations: HashMap<String, SpriteAnimationStoreItem>,
}
impl SpriteAnimationStore {
    pub fn add(&mut self, name: impl ToString, next_frame_seconds: f32, indices: Vec<usize>) {
        self.animations.insert(name.to_string(), SpriteAnimationStoreItem {
            next_frame_seconds, indices,
        });
    }

    pub fn get(&self, name: &str) -> Option<SpriteAnimation> {
        self.animations.get(name).map(|item| item.into())
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct AnimationFrameExtract {
    animation_order: usize,
    sprite_index: usize,
}

pub fn generate_sprite_animation_store<T: Reflect>(sprites: &T) -> SpriteAnimationStore {
    let animation_pattern = Regex::new("^(?P<name>.*)_(?P<animation_order>\\d+)").expect("Animation regex pattern didn't compile");
    let mut sprite_animation_store = SpriteAnimationStore::default();
    if let ReflectRef::Struct(reflect_struct) = sprites.reflect_ref() {
        let names = (0..)
            .zip(reflect_struct.iter_fields())
            .filter(|(_, reflect)| reflect.type_name() == 0usize.type_name())
            .map(|(i, _)| {
                (reflect_struct
                    .name_at(i)
                    .expect("name_at should not be out of range"),
                    *reflect_struct.field_at(i).as_ref().unwrap().downcast_ref().unwrap())
            });
    
        let mut animation_frame_extracts  = HashMap::default();
        for (name, sprite_index) in names {
            if let Some(cap) = animation_pattern.captures(name) {
                if let Ok(animation_order) = &cap["animation_order"].parse() {
                    let animation_order = *animation_order;
                    let name = &cap["name"];
                    let frame = AnimationFrameExtract {
                        animation_order, sprite_index,
                    };
                    let entry = animation_frame_extracts.entry(name.to_string())
                        .or_insert_with(|| Vec::new());
                    entry.push(frame);
                }
            }
        }

        for (name, mut frames) in animation_frame_extracts {
            frames.sort();
            sprite_animation_store.add(name, 0.2, frames.into_iter()
                .map(|frame| frame.sprite_index).collect());
        }
    }
    sprite_animation_store
}
