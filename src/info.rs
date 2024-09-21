use chrono::{Datelike, Utc};
use egui::{Color32, Image, RichText, Vec2, Widget};

use crate::{
    app::LoadedImages, ContentType, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT,
    CENTER_GROUP_WIDTH, ICON_SIZE,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Info {
    #[serde(skip)]
    pub birth_year: i32,
    pub infos: Vec<(String, String, Option<usize>)>, // Type, Value, Image index
    #[serde(skip)]
    pub uuid: uuid::Uuid,
    pub link_paths: Vec<(String, String, ContentType, Option<usize>)>, // Url, Name, Type, Image index
}

pub struct InfoWidget<'a> {
    info: &'a Info,
    loaded_images: &'a LoadedImages<'a>,
}

impl<'a> InfoWidget<'a> {
    pub fn new(info: &'a Info, loaded_images: &'a LoadedImages<'a>) -> Self {
        Self {
            info,
            loaded_images,
        }
    }
}

impl<'a> Widget for InfoWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical_centered_justified(|ui| {
            let bg_fill = if ui.visuals().dark_mode {
                BG_COLOR_SCALING_DARK
            } else {
                BG_COLOR_SCALING_LIGHT
            };
            ui.group(|ui| {
                ui.set_width(CENTER_GROUP_WIDTH);

                egui::ScrollArea::vertical()
                    .id_source(format!("{}", self.info.uuid))
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading(RichText::new("Information").underline().strong());
                        ui.columns(3, |ui| {
                            // Social links
                            for (url, name, content_type, image_index) in &self.info.link_paths {
                                match content_type {
                                    ContentType::Pdf => {}
                                    ContentType::Video => {}
                                    ContentType::Link => {
                                        if let Some(image_index) = image_index {
                                            if let Some(image_source) =
                                                self.loaded_images.images.get(*image_index)
                                            {
                                                let image = Image::new(image_source.clone())
                                                    .max_size(Vec2::new(ICON_SIZE, ICON_SIZE))
                                                    .bg_fill(Color32::from_additive_luminance(
                                                        bg_fill,
                                                    ));
                                                ui[2].add_sized(
                                                    Vec2::new(ICON_SIZE, ICON_SIZE),
                                                    image,
                                                );
                                            }
                                        }
                                        ui[0].hyperlink_to(name, url);
                                        ui[1].label("");
                                    }
                                }
                            }
                            // General information
                            for (label, value, image_index) in &self.info.infos {
                                ui[0].label(label);
                                ui[1].label(value);
                                if let Some(image_index) = image_index {
                                    if let Some(image_source) =
                                        self.loaded_images.images.get(*image_index)
                                    {
                                        let image = Image::new(image_source.clone())
                                            .shrink_to_fit()
                                            .bg_fill(Color32::from_additive_luminance(bg_fill));
                                        ui[2].add_sized(Vec2::new(ICON_SIZE, ICON_SIZE), image);
                                    }
                                } else {
                                    ui[2].label("");
                                }
                            }
                            // Age
                            ui[0].label(RichText::new("Age:"));
                            ui[1].label(RichText::new(format!(
                                "{}",
                                calculate_age(self.info.birth_year)
                            )));
                        });
                    });
            });
        })
        .response
    }
}

fn calculate_age(birth_year: i32) -> i32 {
    Utc::now().year() - birth_year
}
