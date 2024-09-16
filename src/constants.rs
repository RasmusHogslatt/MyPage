use std::default;

pub const SIDE_PANEL_WIDTH: f32 = 400.0;
pub const SIZE_IMAGE_WIDTH: f32 = 150.0;
pub const GROUP_WIDTH: f32 = 375.0;
pub const SIZE_IMAGE_HEIGHT: f32 = 80.0;
pub const BG_COLOR_SCALING_DARK: u8 = 200;
pub const BG_COLOR_SCALING_LIGHT: u8 = 0;
pub const ICON_SIZE: f32 = 15.0;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum ContentType {
    Pdf,
    Video,
    Github,
}
