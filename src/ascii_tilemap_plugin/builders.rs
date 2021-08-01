use bevy::math::UVec2;
use bevy_ecs_tilemap::LayerBuilder;

use super::LayerInfo;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AsciiTilemapSettings {
    /// The amount of tiles displayed on the screen horizontally
    pub(crate) width: u32,
    /// The amount of tiles displayed on the screen horizontally
    pub(crate) height: u32,
    /// The amount of pixels horizontally for a single tile
    pub(crate) window_width: f32,
    /// The amount of pixels vertically for a single tile
    pub(crate) window_height: f32,
    /// The amount of tiles horizontally in the spritesheet
    pub(crate) tilesheet_width: u32,
    /// The amount of tiles vertically in the spritesheet
    pub(crate) tilesheet_height: u32,
    /// The amount of chunks horizontally
    pub(crate) horizontal_chunks: u32,
    /// The amount of chunks vertically
    pub(crate) vertical_chunks: u32,
    pub(crate) layers: Vec<LayerInfoBuilder>,
}

impl Default for AsciiTilemapSettings {
    fn default() -> Self {
        Self {
            width: 80,
            height: 50,
            window_width: 80. * 16.,
            window_height: 50. * 16.,
            tilesheet_width: 16,
            tilesheet_height: 16,
            horizontal_chunks: 1,
            vertical_chunks: 1,
            layers: vec![],
        }
    }
}

impl AsciiTilemapSettings {
    pub fn builder() -> AsciiTilemapSettingsBuilder {
        AsciiTilemapSettingsBuilder::default()
    }
}

pub struct AsciiTilemapSettingsBuilder {
    settings: AsciiTilemapSettings,
}

impl Default for AsciiTilemapSettingsBuilder {
    fn default() -> Self {
        Self {
            settings: AsciiTilemapSettings::default(),
        }
    }
}

impl AsciiTilemapSettingsBuilder {
    pub fn with_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.width = width;
        self.settings.height = height;
        self
    }

    pub fn with_window_dimensions(&mut self, width: f32, height: f32) -> &mut Self {
        self.settings.window_width = width;
        self.settings.window_height = height;
        self
    }

    /// The dimension of the tilesheet
    /// WARN in tiles not in pixels
    pub fn with_tilesheet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.tilesheet_width = width;
        self.settings.tilesheet_height = height;
        self
    }

    pub fn with_chunks(&mut self, horizontal: u32, vertical: u32) -> &mut Self {
        self.settings.horizontal_chunks = horizontal;
        self.settings.vertical_chunks = vertical;
        self
    }

    pub fn with_layer(&mut self, layer_builder: &mut LayerInfoBuilder) -> &mut Self {
        self.settings.layers.push(layer_builder.clone());
        self
    }

    pub fn build(&self) -> AsciiTilemapSettings {
        self.settings.clone()
    }
}

#[derive(Clone)]
pub struct LayerInfoBuilder {
    pub tilesheet_path: String,
    pub tile_dimension: UVec2,
    pub layer_info: LayerInfo,
}

impl LayerInfoBuilder {
    pub fn new(layer_id: u8) -> Self {
        Self {
            tilesheet_path: "".to_string(),
            tile_dimension: UVec2::new(16, 16),
            layer_info: LayerInfo::new(layer_id, true, true),
        }
    }

    pub fn tile_dimension(&mut self, width: u32, height: u32) -> &mut Self {
        self.tile_dimension = UVec2::new(width, height);
        self
    }

    pub fn tilesheet_path(&mut self, path: &str) -> &mut Self {
        self.tilesheet_path = path.to_string();
        self
    }

    pub fn is_transparent(&mut self, value: bool) -> &mut Self {
        self.layer_info.is_transparent = value;
        self
    }

    pub fn is_background_transparent(&mut self, value: bool) -> &mut Self {
        self.layer_info.is_glyph_background_transparent = value;
        self
    }
}
