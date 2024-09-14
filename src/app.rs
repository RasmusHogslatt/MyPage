use std::{fmt::format, process::Command};

use egui::{global_dark_light_mode_switch, ImageSource};
use egui_extras::install_image_loaders;
use uuid::Uuid;

use crate::{
    Education, EducationWidget, Experience, ExperienceWidget, Project, ProjectWidget,
    SIDE_PANEL_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PersonalPortfolio<'a> {
    name: String,
    title: String,
    about_me: String,
    skills: Vec<String>,
    #[serde(skip)]
    experiences: Vec<Experience>,
    #[serde(skip)]
    educations: Vec<Education>,
    #[serde(skip)]
    projects: Vec<Project>,
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
        let liu = egui::include_image!("../assets/liu.png");
        let voysys = egui::include_image!("../assets/voysys.png");
        let easylaser = egui::include_image!("../assets/easylaser.png");
        let mut images = Vec::new();
        images.push(saab);
        images.push(liu);
        images.push(voysys);
        images.push(easylaser);
        LoadedImages { images }
    }
}

impl<'a> Default for PersonalPortfolio<'a> {
    fn default() -> Self {
        let mut experiences: Vec<Experience> = Vec::new();
        experiences.push(Experience {
            company: "Saab AB".to_string(),
            position: "Software Engineer".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 0,
            has_link: false,
            link_path: "".to_string(),
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Voysys".to_string(),
            position: "Software Engineer".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 2,
            has_link: false,
            link_path: "".to_string(),
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Easy Laser".to_string(),
            position: "Software Engineer".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 3,
            has_link: false,
            link_path: "".to_string(),
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Linköping University".to_string(),
            position: "Research assistant".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "Worked through summer and part time during coming semesters on the open source project Inviwo. It is an open source scientific visualization software developed mainly in C++ and OpenGL".to_string(),
            image_index: 1,
            has_link: false,
            link_path: "".to_string(),
            uuid: Uuid::new_v4(),
        });
        let mut educations = Vec::new();
        educations.push(Education {
            university: "Linköping University".to_string(),
            degree: "MSc Technology of Media".to_owned(),
            start: "2019".to_string(),
            end: "2024".to_string(),
            description: "A degree similar to computer science, with stronger emphasis on the math and coding of computer graphics.".to_string(),
            grade_score: "4.0/5.0".to_string(),
            image_index: 1,
            academic_record_path: "assets/Intyg.pdf".to_string(),
            has_link: true,
            uuid: Uuid::new_v4(),
        });
        let mut projects = Vec::new();
        projects.push(Project {
            title: "Visualizing RC car in Mixed Reality".to_string(),
            description: "sfaf".to_string(),
            has_image: true,
            has_link: true,
            link_path: "".to_string(),
            image_index: 2,
            tools: vec![
                "Rust".to_string(),
                "C++".to_string(),
                "MR".to_string(),
                "Android".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });

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
            educations,
            projects,
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

        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                global_dark_light_mode_switch(ui);
            });

        egui::SidePanel::left("left_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("left_scroll_area")
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading("Education");
                        for education in &self.educations {
                            ui.add(EducationWidget::new(education, &self.images));
                        }
                        ui.heading("Experience");
                        for experience in &self.experiences {
                            ui.add(ExperienceWidget::new(experience, &self.images));
                        }
                    });
            });

        egui::SidePanel::right("right_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Projects");
                for project in &self.projects {
                    ui.add(ProjectWidget::new(project, &self.images));
                }
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

pub fn open_pdf(file_path: String) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(format!("/{}", &file_path), "_blank");
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = std::path::Path::new(&file_path);
        if path.exists() {
            #[cfg(target_os = "windows")]
            {
                Command::new("cmd")
                    .args(&["/C", "start", "", &file_path])
                    .spawn()
                    .expect("Failed to open PDF");
            }
            #[cfg(target_os = "macos")]
            {
                Command::new("open")
                    .arg(file_path)
                    .spawn()
                    .expect("Failed to open PDF");
            }
            #[cfg(target_os = "linux")]
            {
                Command::new("xdg-open")
                    .arg(&self.resume_pdf_path)
                    .spawn()
                    .expect("Failed to open PDF");
            }
        } else {
            eprintln!("PDF file not found: {}", file_path);
        }
    }
}
