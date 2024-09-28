use egui::Color32;

pub const SIDE_PANEL_WIDTH: f32 = 400.0;
pub const SIZE_IMAGE_WIDTH: f32 = 150.0;
pub const GROUP_WIDTH: f32 = 375.0;
pub const CENTER_GROUP_WIDTH: f32 = 600.0;
pub const SIZE_IMAGE_HEIGHT: f32 = 80.0;
pub const BG_COLOR_SCALING_DARK: u8 = 200;
pub const BG_COLOR_SCALING_LIGHT: u8 = 0;
pub const ICON_SIZE: f32 = 15.0;

// Colors
pub const BG_COLOR: Color32 = Color32::from_rgb(120, 120, 120);
pub const PRIMARY_ORANGE: Color32 = Color32::from_rgb(255, 153, 102);
pub const DARK_ORANGE: Color32 = Color32::from_rgb(255, 127, 80);
pub const LIGHT_ORANGE: Color32 = Color32::from_rgb(255, 204, 153);
pub const DARK_GRAY: Color32 = Color32::from_rgb(74, 74, 74);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const HEADING_COLOR: Color32 = Color32::from_rgb(255, 153, 102);
pub const SUBHEADING_COLOR: Color32 = Color32::from_rgb(255, 153, 102);
pub const TEXT_COLOR: Color32 = Color32::from_rgb(255, 204, 153);
pub const TOOL_COLOR: Color32 = Color32::from_rgb(32, 32, 32);
pub const HYPERLINK_COLOR: Color32 = Color32::from_rgb(255, 255, 153);

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub enum ContentType {
    Pdf,
    Video,
    Link,
}
