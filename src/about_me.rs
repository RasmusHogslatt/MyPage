use egui::{Color32, Image, RichText, Vec2, Widget};

use crate::{
    app::LoadedImages, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, SIZE_IMAGE_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct AboutMe {
    pub description: Vec<(String, Option<usize>)>, // Text, image index
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
        ui.vertical_centered_justified(|ui| {
            let bg_fill = if ui.visuals().dark_mode {
                BG_COLOR_SCALING_DARK
            } else {
                BG_COLOR_SCALING_LIGHT
            };
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
                            ui.heading(
                                RichText::new(self.about_me.description[0].0.clone()).strong(),
                            );
                        }
                        egui::Grid::new("about_me_grid")
                            .max_col_width(GROUP_WIDTH * 0.7)
                            .num_columns(2)
                            .show(ui, |ui| {
                                for (index, description) in
                                    self.about_me.description.iter().enumerate()
                                {
                                    if index != 0 {
                                        ui.label(RichText::new(description.0.clone()));
                                    }
                                    if description.1.is_some() {
                                        if let Some(image_source) =
                                            self.loaded_images.images.get(description.1.unwrap())
                                        {
                                            let image = Image::new(image_source.clone())
                                                .shrink_to_fit()
                                                .bg_fill(Color32::from_additive_luminance(bg_fill));
                                            ui.add_sized(
                                                Vec2::new(
                                                    SIZE_IMAGE_WIDTH / 2.0,
                                                    SIZE_IMAGE_WIDTH / 2.0,
                                                ),
                                                image,
                                            );
                                        } else {
                                        }
                                    }
                                    ui.end_row();
                                }
                            });
                    });
            });
        })
        .response
    }
}
