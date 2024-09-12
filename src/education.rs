use egui::{Color32, Image, RichText, Vec2, Widget};

use crate::{
    app::LoadedImages, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, SIZE_IMAGE_HEIGHT,
    SIZE_IMAGE_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Education {
    pub university: String,
    pub degree: String,
    pub start: String,
    pub end: String,
    pub description: String,
    pub image_index: usize,
}

// New struct to wrap Experience and LoadedImages
pub struct EducationWidget<'a> {
    experience: &'a Education,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> EducationWidget<'a> {
    pub fn new(experience: &'a Education, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            experience,
            loaded_images,
        }
    }
}

impl<'a> Widget for EducationWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            if let Some(image_source) = self.loaded_images.images.get(self.experience.image_index) {
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
            ui.columns(2, |ui| {
                ui[0].label(
                    RichText::new(&self.experience.university)
                        .strong()
                        .underline(),
                );
                ui[0].label(&self.experience.degree);
            });
        })
        .response
    }
}
