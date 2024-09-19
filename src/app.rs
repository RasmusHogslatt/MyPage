#[cfg(target_arch = "wasm32")]
use crate::constants::PROJECT_NAME;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

use egui::{global_dark_light_mode_switch, ImageSource};
use egui_extras::install_image_loaders;
use uuid::Uuid;

use crate::{
    AboutMe, AboutMeWidget, ContentType, Education, EducationWidget, Experience, ExperienceWidget,
    Info, InfoWidget, Project, ProjectWidget, SIDE_PANEL_WIDTH,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PersonalPortfolio<'a> {
    name: String,
    title: String,
    skills: Vec<String>,
    #[serde(skip)]
    info: Info,
    #[serde(skip)]
    about_me: AboutMe,
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
        let me = egui::include_image!("../assets/me_lego2.png");
        let donaldduck = egui::include_image!("../assets/donaldduck.png");
        let crab = egui::include_image!("../assets/crab.png");
        let mcpt = egui::include_image!("../assets/mcpt.png");
        let pw = egui::include_image!("../assets/pw.png");
        let thesis = egui::include_image!("../assets/thesis.png");
        let mut images = Vec::new();
        images.push(saab); // 0
        images.push(liu); // 1
        images.push(voysys); // 2
        images.push(easylaser); // 3
        images.push(me); // 4
        images.push(donaldduck); // 5
        images.push(crab); // 6
        images.push(mcpt); // 7
        images.push(pw); // 8
        images.push(thesis); // 9
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
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Voysys".to_string(),
            position: "Software Engineer".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 2,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Easy Laser".to_string(),
            position: "Software Engineer".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "asdfasd".to_string(),
            image_index: 3,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Linköping University".to_string(),
            position: "Research assistant".to_string(),
            start: "June 2024".to_string(),
            end: "Current".to_string(),
            description: "Worked through summer and part time during coming semesters on the open source project Inviwo. It is an open source scientific visualization software developed mainly in C++ and OpenGL".to_string(),
            image_index: 1,
            link_path: None,
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
            academic_record_path: Some("assets/Intyg.pdf".to_string()),
            uuid: Uuid::new_v4(),
        });
        let mut projects = Vec::new();
        projects.push(Project {
            title: "Visualizing RC car in Mixed Reality".to_string(),
            description: "Visualization and control of an RC car in Mixed Reality. Implemented in Rust, involving contributions to the open source Bevy Engine. This was part of my master's thesis.".to_string(),
            has_image: true,
            has_link: true,
            link_paths: vec![("assets/MasterThesis.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://www.youtube.com/watch?v=vJKHNgr7sD4".to_string(), "Video".to_string(), ContentType::Video)],
            image_index: 9,
            tools: vec![
                "Rust".to_string(),
                "C++".to_string(),
                "MR".to_string(),
                "Android".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Monte Carlo Pathtracer".to_string(),
            description: "A Monte Carlo pathtracer that renders realisitc reflections, refractions and direct and indirect illumination of diffuse objects.".to_string(),
            has_image: true,
            has_link: true,
            link_paths: vec![("assets/MCPT.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://github.com/RasmusHogslatt/raytracer/tree/master".to_string(), "Github".to_string(), ContentType::Github)],
            image_index: 7,
            tools: vec![
                "C++".to_string(),
                "Rendering".to_string(),
                "Math".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Procedural waves and geometry".to_string(),
            description: "Using C++ and OpenGL, a flat surface was rendered as waves on the GPU. Single vertices were also rendered as boxes by utilizing geometry shaders.".to_string(),
            has_image: true,
            has_link: true,
            link_paths: vec![("assets/pw.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://github.com/RasmusHogslatt/Procedural-waves".to_string(), "Github".to_string(), ContentType::Github), ("https://www.youtube.com/watch?v=N_k3nFntPOg&t=2s".to_string(), "Video".to_string(), ContentType::Video)],
            image_index: 8,
            tools: vec![
                "C++".to_string(),
                "OpenGL".to_string(),
                "GLSL".to_string(),
                "Rendering".to_string(),
                "Math".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        let infos: Vec<(String, String)> = vec![
            ("Age".to_string(), "25".to_string()),
            ("Age".to_string(), "25".to_string()),
        ];
        let info = Info {
            infos,
            link_paths: vec![],
            image_indices: Some(vec![5]),
            uuid: Uuid::new_v4(),
        };
        let about_me = AboutMe {
            description: vec![("Hi, I'm Rasmus!".to_string(), None), ("I'm a software engineer from Sweden. My favourite programming language is rust, the language of the crabs!".to_string(), Some(6))
             ,("I play the guitar, solve Rubik's cube and love working out. I am also member Nr. 3150 of NAFS(K), the Swedish Donaldism association.".to_string(), Some(5))],
            uuid: Uuid::new_v4(),
        };

        Self {
            name: "Rasmus Hogslätt".to_owned(),
            title: "Software developer".to_owned(),
            about_me,
            skills: vec![
                "Skill 1".to_owned(),
                "Skill 2".to_owned(),
                "Skill 3".to_owned(),
            ],
            experiences,
            educations,
            projects,
            info,
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
        //
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
                    //ui.heading(&self.name);
                    ui.add(AboutMeWidget::new(&self.about_me, &self.images));
                    ui.add(InfoWidget::new(&self.info, &self.images));
                },
            );
        });
    }
}

pub fn open_pdf(file_path: String) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            // let _ = window.open_with_url_and_target(
            //     format!("{}/{}", PROJECT_NAME, &file_path).as_str(),
            //     "_self",
            // );
            let _ = window.open_with_url_and_target(format!("/{}", &file_path).as_str(), "_self");
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
