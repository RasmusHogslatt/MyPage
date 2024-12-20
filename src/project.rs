use egui::{Color32, Hyperlink, Image, RichText, Vec2, Widget};

use crate::{
    app::{open_pdf, LoadedImages},
    ContentType, BG_COLOR_SCALING_DARK, BG_COLOR_SCALING_LIGHT, GROUP_WIDTH, SIZE_IMAGE_HEIGHT,
    SIZE_IMAGE_WIDTH, SUBHEADING_COLOR, TEXT_COLOR, TOOL_COLOR,
};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub link_paths: Vec<(String, String, ContentType)>, // Url, Name, Type
    pub image_index: Option<usize>,
    pub tools: Vec<String>,
    #[serde(skip)]
    pub uuid: uuid::Uuid,
}

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
            if let Some(image_index) = self.project.image_index {
                if let Some(image_source) = self.loaded_images.images.get(image_index) {
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
            }

            ui.label(
                RichText::new(&self.project.title)
                    .strong()
                    .underline()
                    .color(SUBHEADING_COLOR),
            );
            ui.label(RichText::new(self.project.description.clone()).color(TEXT_COLOR));
            ui.horizontal(|ui| {
                for (url, name, content_type) in &self.project.link_paths {
                    match content_type {
                        ContentType::Pdf => {
                            if ui
                                .add(Hyperlink::from_label_and_url(RichText::new(name), url))
                                .clicked()
                            {
                                open_pdf(url.to_string());
                            }
                        }
                        ContentType::Video => {
                            ui.hyperlink_to(name, url);
                        }

                        ContentType::Link => {
                            ui.hyperlink_to(name, url);
                        }
                    }
                }
            });
            let mut tool_string: String = "".to_string();
            for tool in self.project.tools.clone() {
                tool_string.push_str(tool.as_str());
                tool_string.push_str("  ");
            }
            ui.label(RichText::new(tool_string).strong().color(TOOL_COLOR));
        })
        .response
    }
}
