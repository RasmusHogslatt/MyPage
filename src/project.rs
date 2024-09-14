use egui::{Color32, Image, RichText, Vec2, Widget};

use crate::{
    app::LoadedImages, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH,
    SIDE_PANEL_WIDTH, SIZE_IMAGE_HEIGHT, SIZE_IMAGE_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub has_image: bool,
    pub has_link: bool,
    pub link_path: String,
    pub image_index: usize,
    pub tools: Vec<String>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}

// New struct to wrap Experience and LoadedImages
pub struct ProjectWidget<'a> {
    project: &'a Project,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> ProjectWidget<'a> {
    pub fn new(project: &'a Project, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            project,
            loaded_images,
        }
    }
}

impl<'a> Widget for ProjectWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.set_width(GROUP_WIDTH);
            if let Some(image_source) = self.loaded_images.images.get(self.project.image_index) {
                let mut bg_fill: u8 = 0;
                if ui.visuals().dark_mode {
                    bg_fill = BG_COLOR_SCALING_DARK;
                } else {
                    bg_fill = BG_COLOR_SCALING_LIGHT;
                }
                let image = Image::new(image_source.clone())
                    .shrink_to_fit()
                    .bg_fill(Color32::from_additive_luminance(bg_fill));
                ui.add_sized(Vec2::new(SIZE_IMAGE_WIDTH, SIZE_IMAGE_HEIGHT), image);
            }

            egui::ScrollArea::vertical()
                .id_source(format!("{}", self.project.uuid))
                .auto_shrink(true)
                .show(ui, |ui| {
                    ui.label(RichText::new(&self.project.title).strong().underline());
                    ui.label(self.project.description.clone());
                    let mut tool_string: String = "".to_string();
                    for tool in self.project.tools.clone() {
                        // ui.label(RichText::new(tool).strong());
                        tool_string.push_str(tool.as_str());
                        tool_string.push_str("  ");
                    }
                    ui.label(RichText::new(tool_string).strong());
                });
        })
        .response
    }
}
