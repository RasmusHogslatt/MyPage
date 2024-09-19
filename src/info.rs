use egui::{Color32, Image, RichText, Vec2, Widget};

use crate::{
    app::LoadedImages, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, ICON_SIZE,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Info {
    pub infos: Vec<(String, String)>,
    pub link_paths: Vec<String>,
    pub image_indices: Option<Vec<usize>>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
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
                ui.set_width(GROUP_WIDTH);

                egui::ScrollArea::vertical()
                    .id_source(format!("{}", self.info.uuid))
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading(RichText::new("Information").underline().strong());
                        ui.columns(3, |ui| {
                            for (index, info) in self.info.infos.iter().enumerate() {
                                ui[0].label(&info.0);
                                ui[1].label(&info.1);
                                if let Some(image_indices) = &self.info.image_indices {
                                    if let Some(&image_index) = image_indices.get(index) {
                                        if let Some(image_source) =
                                            self.loaded_images.images.get(image_index)
                                        {
                                            let image = Image::new(image_source.clone())
                                                .shrink_to_fit()
                                                .bg_fill(Color32::from_additive_luminance(bg_fill));
                                            ui[2].add_sized(Vec2::new(ICON_SIZE, ICON_SIZE), image);
                                        }
                                    }
                                }
                            }
                        });
                    });
            });
        })
        .response
    }
}
