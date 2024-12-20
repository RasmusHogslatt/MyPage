use egui::{Align, Color32, Hyperlink, Image, RichText, Vec2, Widget};

use crate::{
    app::{open_pdf, LoadedImages},
    BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, SIZE_IMAGE_HEIGHT,
    SIZE_IMAGE_WIDTH, SUBHEADING_COLOR, TEXT_COLOR,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Education {
    pub university: String,
    pub degree: String,
    pub start: String,
    pub end: String,
    pub description: String,
    pub grade_score: String,
    pub image_index: usize,
    pub academic_record_path: Option<String>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}

pub struct EducationWidget<'a> {
    education: &'a Education,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> EducationWidget<'a> {
    pub fn new(education: &'a Education, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            education,
            loaded_images,
        }
    }
}

impl<'a> Widget for EducationWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.group(|ui| {
            ui.set_width(GROUP_WIDTH);
            if let Some(image_source) = self.loaded_images.images.get(self.education.image_index) {
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
            ui.label(
                RichText::new(&self.education.university)
                    .strong()
                    .underline()
                    .color(SUBHEADING_COLOR),
            );
            ui.columns(2, |ui| {
                ui[0].label(RichText::new(&self.education.degree).color(SUBHEADING_COLOR));
                ui[1].with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                    ui.label(
                        RichText::new(&self.education.grade_score)
                            .strong()
                            .color(SUBHEADING_COLOR),
                    )
                });
            });
            egui::ScrollArea::vertical()
                .id_source(format!("{}", self.education.uuid))
                .auto_shrink(true)
                .show(ui, |ui| {
                    ui.label(RichText::new(self.education.description.clone()).color(TEXT_COLOR));
                });
            if let Some(url) = &self.education.academic_record_path {
                if ui
                    .add(Hyperlink::from_label_and_url("Academic Record", url))
                    .clicked()
                {
                    open_pdf(url.to_string());
                }
            }
        })
        .response
    }
}
