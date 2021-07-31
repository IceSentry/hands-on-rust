use super::Layer;

#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AsciiTilemapSettings {
    /// The asset path to the tilesheet texture
    pub(crate) tilesheet_asset_path: String,
    /// The amount of tiles displayed on the screen horizontally
    pub(crate) width: u32,
    /// The amount of tiles displayed on the screen horizontally
    pub(crate) height: u32,
    /// The amount of pixels horizontally for a single tile
    pub(crate) tile_width: u32,
    /// The amount of pixels vertically for a single tile
    pub(crate) tile_height: u32,
    /// The amount of tiles horizontally in the spritesheet
    pub(crate) tilesheet_width: u32,
    /// The amount of tiles vertically in the spritesheet
    pub(crate) tilesheet_height: u32,
    /// The amount of chunks horizontally
    pub(crate) horizontal_chunks: u32,
    /// The amount of chunks vertically
    pub(crate) vertical_chunks: u32,
    pub(crate) layers: Vec<Layer>,
}

impl Default for AsciiTilemapSettings {
    fn default() -> Self {
        Self {
            tilesheet_asset_path: "tilesheet.png".into(),
            width: 80,
            height: 50,
            tile_width: 16,
            tile_height: 16,
            tilesheet_height: 16,
            tilesheet_width: 16,
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

    pub fn window_width(&self) -> f32 {
        (self.width * self.tile_width) as f32
    }

    pub fn window_height(&self) -> f32 {
        (self.height * self.tile_height) as f32
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

    pub fn with_tile_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.tile_width = width;
        self.settings.tile_height = height;
        self
    }

    pub fn with_tilesheet_path<S: ToString>(&mut self, path: S) -> &mut Self {
        self.settings.tilesheet_asset_path = path.to_string();
        self
    }

    /// The dimension of the tilesheet
    /// WARN in tiles not in pixels
    pub fn with_tilesheet_dimensions(&mut self, width: u32, height: u32) -> &mut Self {
        self.settings.tilesheet_width = width;
        self.settings.tile_height = height;
        self
    }

    pub fn with_chunks(&mut self, horizontal: u32, vertical: u32) -> &mut Self {
        self.settings.horizontal_chunks = horizontal;
        self.settings.vertical_chunks = vertical;
        self
    }

    pub fn with_layer(
        &mut self,
        layer_id: u8,
        is_transparent: bool,
        is_glyph_background_transparent: bool,
    ) -> &mut Self {
        self.settings.layers.push(Layer::new(
            layer_id,
            is_transparent,
            is_glyph_background_transparent,
        ));
        self
    }

    pub fn build(&self) -> AsciiTilemapSettings {
        self.settings.clone()
    }
}
