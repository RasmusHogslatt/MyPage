use std::default;

use egui::{include_image, ImageSource};
use egui_extras::install_image_loaders;

use crate::{Education, Experience, ExperienceWidget, SIDE_PANEL_WIDTH};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PersonalPortfolio<'a> {
    name: String,
    title: String,
    about_me: String,
    skills: Vec<String>,
    experiences: Vec<Experience>,
    education: Vec<Education>,
    contact_email: String,
    #[serde(skip)]
    images: LoadedImages<'a>,
}

pub struct LoadedImages<'a> {
    pub images: Vec<ImageSource<'a>>,
}

impl<'a> Default for LoadedImages<'a> {
    fn default() -> Self {
        let saab = egui::include_image!("../assets/saab.png");
        let mut images = Vec::new();
        images.push(saab);
        LoadedImages { images }
    }
}

impl<'a> Default for PersonalPortfolio<'a> {
    fn default() -> Self {
        let mut experiences: Vec<Experience> = Vec::new();
        experiences.push(Experience {
            company: "Saab AB".to_string(),
            position: "Software Engineer".to_string(),
            start: "2024 June".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 0,
        });

        // let mut images = LoadedImages::default();
        // images
        //     .images
        //     .push(ImageSource::Uri("../assets/saab.png".into()));

        Self {
            name: "Rasmus Hogslätt".to_owned(),
            title: "Software developer".to_owned(),
            about_me: "Write a brief introduction about yourself here.".to_owned(),
            skills: vec![
                "Skill 1".to_owned(),
                "Skill 2".to_owned(),
                "Skill 3".to_owned(),
            ],
            experiences,
            education: vec![Education {
                institution: "Linköping University".to_owned(),
                degree: "Technology of Media".to_owned(),
                year: "2024".to_owned(),
            }],
            contact_email: "r.hogslatt@gmail.com".to_owned(),
            images: LoadedImages::default(),
        }
    }
}

impl<'a> PersonalPortfolio<'a> {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl<'a> eframe::App for PersonalPortfolio<'a> {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        install_image_loaders(ctx);
        egui::SidePanel::left("left_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Education");
                ui.heading("Experience");
                for experience in &self.experiences {
                    ui.add(ExperienceWidget::new(experience, &self.images));
                }
            });

        egui::SidePanel::right("right_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .show(ctx, |ui| {
                ui.heading("Projects");
                ui.image(egui::include_image!("../assets/saab.png"));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::top_down_justified(egui::Align::Center),
                |ui| {
                    ui.heading(&self.name);
                },
            );
        });
    }
}
