use super::Layer;
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct TilemapBuilder {
    pub layers: Vec<LayerDataBuilder>,
}

impl TilemapBuilder {
    pub fn new() -> Self {
        Self { layers: vec![] }
    }

    pub fn build(&mut self) -> Self {
        self.clone()
    }

    pub fn with_layer(&mut self, layer: &mut LayerDataBuilder) -> &mut Self {
        self.layers.push(layer.clone());
        self
    }
}

#[derive(Debug, Clone)]
pub struct LayerDataBuilder {
    pub texture_path: Option<String>,
    pub size: Option<UVec2>,
    pub tile_size: Option<Vec2>,
    /// WARN dimension in tiles
    pub tilesheet_size: Option<Vec2>,
    pub id: u16,
    pub is_transparent: bool,
    pub is_background_transparent: bool,
}

impl LayerDataBuilder {
    pub fn new(id: u16) -> Self {
        Self {
            texture_path: None,
            size: None,
            tile_size: None,
            tilesheet_size: Some(Vec2::new(16., 16.)),
            id,
            is_transparent: false,
            is_background_transparent: false,
        }
    }

    pub fn texture_path(&mut self, path: &str) -> &mut Self {
        self.texture_path = Some(path.to_string());
        self
    }

    pub fn size(&mut self, width: u32, height: u32) -> &mut Self {
        self.size = Some(UVec2::new(width, height));
        self
    }

    pub fn tile_size(&mut self, width: f32, height: f32) -> &mut Self {
        self.tile_size = Some(Vec2::new(width, height));
        self
    }

    #[allow(unused)]
    pub fn tilesheet_size(&mut self, tilesheet_size: Vec2) -> &mut Self {
        self.tilesheet_size = Some(tilesheet_size);
        self
    }

    pub fn is_transparent(&mut self, is_transparent: bool) -> &mut Self {
        self.is_transparent = is_transparent;
        self
    }

    pub fn is_background_transparent(&mut self, is_background_transparent: bool) -> &mut Self {
        self.is_background_transparent = is_background_transparent;
        self
    }

    pub(super) fn build_layer(&self) -> Layer {
        Layer {
            background_id: self.id * 2,
            foreground_id: self.id * 2 + 1,
            command_buffer: vec![],
            size: self.size.expect("layer.size not set"),
            is_background_transparent: self.is_background_transparent,
            is_transparent: self.is_transparent,
        }
    }
}
