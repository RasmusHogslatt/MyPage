use egui::{Color32, Hyperlink, Image, RichText, Vec2, Widget};

use crate::{
    app::{open_pdf, LoadedImages},
    BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, ICON_SIZE, SIDE_PANEL_WIDTH,
    SIZE_IMAGE_HEIGHT, SIZE_IMAGE_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AboutMe {
    pub description: Vec<(String)>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}

// New struct to wrap Experience and LoadedImages
pub struct AboutMeWidget<'a> {
    about_me: &'a AboutMe,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> AboutMeWidget<'a> {
    pub fn new(about_me: &'a AboutMe, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            about_me,
            loaded_images,
        }
    }
}

impl<'a> Widget for AboutMeWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut bg_fill: u8 = 0;
        if ui.visuals().dark_mode {
            bg_fill = BG_COLOR_SCALING_DARK;
        } else {
            bg_fill = BG_COLOR_SCALING_LIGHT;
        }
        ui.group(|ui| {
            ui.set_width(GROUP_WIDTH);

            egui::ScrollArea::vertical()
                .id_source(format!("{}", self.about_me.uuid))
                .auto_shrink(true)
                .show(ui, |ui| {
                    if let Some(image_source) = self.loaded_images.images.get(4) {
                        let image = Image::new(image_source.clone())
                            .shrink_to_fit()
                            .bg_fill(Color32::from_additive_luminance(bg_fill));
                        ui.add_sized(Vec2::new(200.0, 300.0), image);
                    } else {
                        // No image
                    }

                    for (index, description) in self.about_me.description.iter().enumerate().clone()
                    {
                        if index == 0 {
                            ui.heading(RichText::new(description).strong());
                        } else {
                            ui.label(RichText::new(description));
                        }
                    }
                    if let Some(image_source) = self.loaded_images.images.get(6) {
                        let image = Image::new(image_source.clone())
                            .shrink_to_fit()
                            .bg_fill(Color32::from_additive_luminance(bg_fill));
                        ui.add_sized(Vec2::new(SIZE_IMAGE_WIDTH, SIZE_IMAGE_WIDTH), image);
                    } else {
                        // No image
                    }
                    if let Some(image_source) = self.loaded_images.images.get(5) {
                        let image = Image::new(image_source.clone())
                            .shrink_to_fit()
                            .bg_fill(Color32::from_additive_luminance(bg_fill));
                        ui.add_sized(Vec2::new(SIZE_IMAGE_WIDTH, SIZE_IMAGE_WIDTH), image);
                    } else {
                        // No image
                    }
                });
        })
        .response
    }
}
