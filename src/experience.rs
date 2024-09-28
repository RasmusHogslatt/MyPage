use egui::{Align, Color32, Hyperlink, Image, RichText, Vec2, Widget};

use crate::{
    app::{open_pdf, LoadedImages},
    BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, SIZE_IMAGE_HEIGHT,
    SIZE_IMAGE_WIDTH, SUBHEADING_COLOR, TEXT_COLOR,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Experience {
    pub company: String,
    pub position: String,
    pub start: String,
    pub end: String,
    pub description: String,
    pub image_index: usize,
    pub link_path: Option<(String, String)>, // URL, Name
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}
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
            ui.set_width(GROUP_WIDTH);
            if let Some(image_source) = self.loaded_images.images.get(self.experience.image_index) {
                let bg_fill = if ui.visuals().dark_mode {
                    BG_COLOR_SCALING_DARK
                } else {
                    BG_COLOR_SCALING_LIGHT
                };
                let image = Image::new(image_source.clone())
                    .shrink_to_fit()
                    .bg_fill(Color32::from_additive_luminance(bg_fill));
                ui.add_sized(Vec2::new(SIZE_IMAGE_WIDTH, SIZE_IMAGE_HEIGHT), image);
            }
            ui.columns(2, |ui| {
                ui[0].label(
                    RichText::new(&self.experience.company)
                        .strong()
                        .underline()
                        .color(SUBHEADING_COLOR),
                );

                ui[1].with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} - {}",
                            self.experience.start, self.experience.end
                        ))
                        .color(SUBHEADING_COLOR),
                    );
                });
                ui[0].label(RichText::new(&self.experience.position).color(SUBHEADING_COLOR));
            });
            ui.label(RichText::new(self.experience.description.clone()).color(TEXT_COLOR));
            if let Some((url, label)) = &self.experience.link_path {
                let hyperlink = Hyperlink::from_label_and_url(label, url);
                if ui.add(hyperlink).clicked() {
                    open_pdf(url.clone());
                }
            }
        })
        .response
    }
}
