use egui::{Image, RichText, Widget};

use crate::app::LoadedImages;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Experience {
    pub company: String,
    pub position: String,
    pub start: String,
    pub end: String,
    pub description: String,
    pub image_index: usize,
}

// New struct to wrap Experience and LoadedImages
pub struct ExperienceWidget<'a> {
    experience: &'a Experience,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> ExperienceWidget<'a> {
    pub fn new(experience: &'a Experience, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            experience,
            loaded_images,
        }
    }
}

impl<'a> Widget for ExperienceWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.columns(2, |ui| {
                ui[0].label(RichText::new(&self.experience.company).strong().underline());
                if let Some(image_source) =
                    self.loaded_images.images.get(self.experience.image_index)
                {
                    let image = Image::new(image_source.clone());
                    ui[1].add(image);
                }
                ui[0].label(&self.experience.position);
            });
        })
        .response
    }
}
