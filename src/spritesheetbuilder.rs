use bevy::prelude::*;

#[derive(Clone)]
pub struct NoTextureAtlas;
#[derive(Clone)]
pub struct NoIndex;

#[derive(Clone)]
pub struct SpriteSheetBundleBuilder<TextureAtlasParameter, IndexParameter> {
    pub texture_atlas: TextureAtlasParameter,
    pub index: IndexParameter,
    pub transform: Transform,
    pub flip_x: bool,
    pub flip_y: bool,
    pub custom_size: Option<Vec2>,
    pub color: Color,
}

impl SpriteSheetBundleBuilder<NoTextureAtlas, NoIndex> {
    pub fn new() -> Self {
        SpriteSheetBundleBuilder {
            texture_atlas: NoTextureAtlas,
            index: NoIndex,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            flip_x: false,
            flip_y: false,
            custom_size: None,
            color: Color::default(),
        }
    }
}

impl<TextureAtlasParameter, IndexParameter> SpriteSheetBundleBuilder<TextureAtlasParameter, IndexParameter> {
    pub fn texture_atlas(self, texture_atlas: Handle<TextureAtlas>) -> SpriteSheetBundleBuilder<Handle<TextureAtlas>, IndexParameter> {
        SpriteSheetBundleBuilder {
            texture_atlas,
            index: self.index,
            transform: self.transform,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            custom_size: self.custom_size,
            color: self.color,
        }
    }

    pub fn index(self, index: usize) -> SpriteSheetBundleBuilder<TextureAtlasParameter, usize> {
        SpriteSheetBundleBuilder {
            texture_atlas: self.texture_atlas,
            index: index,
            transform: self.transform,
            flip_x: self.flip_x,
            flip_y: self.flip_y,
            custom_size: self.custom_size,
            color: self.color,
        }
    }

    pub fn transform(self, transform: Transform) -> Self {
        SpriteSheetBundleBuilder {
            transform,
            ..self
        }
    }

    pub fn flip_x(self, flip_x: bool) -> Self {
        SpriteSheetBundleBuilder {
            flip_x,
            ..self
        }
    }

    pub fn flip_y(self, flip_y: bool) -> Self {
        SpriteSheetBundleBuilder {
            flip_y,
            ..self
        }
    }

    pub fn custom_size(self, custom_size: impl Into<Option<Vec2>>) -> Self {
        SpriteSheetBundleBuilder {
            custom_size: custom_size.into(),
            ..self
        }
    }

    pub fn color(self, color: Color) -> Self {
        SpriteSheetBundleBuilder {
            color,
            ..self
        }
    }
}

impl SpriteSheetBundleBuilder<Handle<TextureAtlas>, usize> {
    pub fn build(self) -> SpriteSheetBundle {
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: self.color,
                index: self.index,
                flip_x: self.flip_x,
                flip_y: self.flip_y,
                custom_size: self.custom_size,
            },
            texture_atlas: self.texture_atlas,
            transform: self.transform,
            ..Default::default()
        }
    }
}
