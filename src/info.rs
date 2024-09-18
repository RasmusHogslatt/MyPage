use egui::{Color32, Hyperlink, Image, RichText, Vec2, Widget};

use crate::{
    app::{open_pdf, LoadedImages},
    BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, ICON_SIZE, SIDE_PANEL_WIDTH,
    SIZE_IMAGE_HEIGHT, SIZE_IMAGE_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Info {
    pub infos: Vec<(String, String)>,
    pub has_image: bool,
    pub link_paths: Vec<String>,
    pub image_indices: Vec<usize>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}

// New struct to wrap Experience and LoadedImages
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
            let mut bg_fill: u8 = 0;
            if ui.visuals().dark_mode {
                bg_fill = BG_COLOR_SCALING_DARK;
            } else {
                bg_fill = BG_COLOR_SCALING_LIGHT;
            }
            ui.group(|ui| {
                ui.set_width(GROUP_WIDTH);

                egui::ScrollArea::vertical()
                    .id_source(format!("{}", self.info.uuid))
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading(RichText::new("Information").underline().strong());
                        ui.columns(3, |ui| {
                            for info in self.info.infos.clone() {
                                ui[0].label(info.0);
                                ui[1].label(info.1);
                                if let Some(image_source) =
                                    self.loaded_images.images.get(self.info.image_indices[0])
                                {
                                    let image = Image::new(image_source.clone())
                                        .shrink_to_fit()
                                        .bg_fill(Color32::from_additive_luminance(bg_fill));
                                    ui[2].add_sized(Vec2::new(ICON_SIZE, ICON_SIZE), image);
                                } else {
                                    // No image
                                }
                            }
                        });
                    });
            });
        })
        .response
    }
}
