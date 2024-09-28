use egui::FontData;
use egui::FontDefinitions;
use egui::FontFamily;

use egui::Rounding;
#[cfg(not(target_arch = "wasm32"))]
use std::process::Command;

use egui::{ImageSource, RichText};
use egui_extras::install_image_loaders;
use uuid::Uuid;

use crate::PRIMARY_ORANGE;
use crate::{
    constants::*, AboutMe, AboutMeWidget, ContentType, Education, EducationWidget, Experience,
    ExperienceWidget, Info, InfoWidget, Project, ProjectWidget,
};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PersonalPortfolio<'a> {
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
        let me = egui::include_image!("../assets/me.jpg");
        let donaldduck = egui::include_image!("../assets/donaldduck.png");
        let crab = egui::include_image!("../assets/crab.png");
        let mcpt = egui::include_image!("../assets/mcpt.png");
        let pw = egui::include_image!("../assets/pw.png");
        let thesis = egui::include_image!("../assets/thesis.png");
        let linkedin = egui::include_image!("../assets/linkedin.png");
        let licenses = egui::include_image!("../assets/licenses.png");
        let sweden = egui::include_image!("../assets/sweden.png");
        let flappybird = egui::include_image!("../assets/flappybird.png");
        let legonization = egui::include_image!("../assets/legonization.png");
        let dncnn = egui::include_image!("../assets/dncnn.png");
        let gameoflife = egui::include_image!("../assets/gameoflife.png");
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
        images.push(linkedin); // 10
        images.push(licenses); // 11
        images.push(sweden); // 12
        images.push(flappybird); // 13
        images.push(legonization); // 14
        images.push(dncnn); // 15
        images.push(gameoflife); // 16
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
            description:
                "Maintaining and developing software used in some of Saab's underwater products."
                    .to_string(),
            image_index: 0,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Voysys".to_string(),
            position: "Thesis Project".to_string(),
            start: "January 2024".to_string(),
            end: "May 2024".to_string(),
            description: "Developed a mixed reality system for controlling and visualizing the path of a remote controlled car in mixed reality on a Meta Quest 3. Implementation was done in Rust, involving contributions to the open source project \"Bevy Engine\".".to_string(),
            image_index: 2,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Linköping University".to_string(),
            position: "Research Assistant".to_string(),
            start: "2022".to_string(),
            end: "2023".to_string(),
            description: "Worked through summer and part time during coming semesters on the open source project Inviwo. It is an open source scientific visualization software developed mainly in C++ and OpenGL".to_string(),
            image_index: 1,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Easy Laser".to_string(),
            position: "Software Developer & Electronics Assembly".to_string(),
            start: "2018".to_string(),
            end: "2022".to_string(),
            description: "Worked full time before university, assembling laser based measuring devices. Here I saw how all steps of the supply chain worked, given that everything everything was done in house.\nThroughout studies, I worked part time during summer as software developer".to_string(),
            image_index: 3,
            link_path: None,
            uuid: Uuid::new_v4(),
        });
        experiences.push(Experience {
            company: "Linköping University".to_string(),
            position: "Math assistant".to_string(),
            start: "2019".to_string(),
            end: "2020".to_string(),
            description: "I assisted in teaching fundamental math, and single- and multivariable calculus for first year university students. This entailed having my own classes where I held presentations and students were able to ask me questions.\nI also assisted in coding courses, such as object oriented programming and immersive visualization.".to_string(),
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
            description: "A degree similar to computer science, with stronger emphasis on the math and coding of computer graphics.\nIn 2024, I received a scholarship for academic performance.".to_string(),
            grade_score: "4.17/5.0".to_string(),
            image_index: 1,
            academic_record_path: Some("assets/Intyg.pdf".to_string()),
            uuid: Uuid::new_v4(),
        });
        let mut projects = Vec::new();
        projects.push(Project {
            title: "Visualizing RC car in Mixed Reality".to_string(),
            description: "Visualization and control of an RC car in Mixed Reality. Implemented in Rust, involving contributions to the open source Bevy Engine. This was part of my master's thesis.".to_string(),
            link_paths: vec![("assets/MasterThesis.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://www.youtube.com/watch?v=vJKHNgr7sD4".to_string(), "Video".to_string(), ContentType::Video)],
            image_index: Some(9),
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
            link_paths: vec![("assets/MCPT.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://github.com/RasmusHogslatt/raytracer/tree/master".to_string(), "Github".to_string(), ContentType::Link)],
            image_index: Some(7),
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
            link_paths: vec![("assets/pw.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://github.com/RasmusHogslatt/Procedural-waves".to_string(), "Github".to_string(), ContentType::Link), ("https://www.youtube.com/watch?v=N_k3nFntPOg&t=2s".to_string(), "Video".to_string(), ContentType::Video)],
            image_index: Some(8),
            tools: vec![
                "C++".to_string(),
                "OpenGL".to_string(),
                "GLSL".to_string(),
                "Rendering".to_string(),
                "Math".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Genetic Traing of Flappy Birds".to_string(),
            description: "Implemented my own genetic training algorithm and neural networks from scratch in Rust and used these to train birds in my own Bevy based implementation of the Flappy Bird game.".to_string(),
            link_paths: vec![("assets/Flappy_Bird.pdf".to_string(), "Report".to_string(), ContentType::Pdf), ("https://www.youtube.com/watch?v=uUAlo93hbfk".to_string(), "Video".to_string(), ContentType::Video)],
            image_index: Some(13),
            tools: vec![
                "Rust".to_string(),
                "AI".to_string(),
                "ML".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Legonization of images".to_string(),
            description: "A program that converts an uploaded image into a lego mosaic and calculates pieces required to reproduce with actual bricks. Support multiple brick types. Color matching was done using CIELAB.".to_string(),
            link_paths: vec![("assets/Legonization.pdf".to_string(), "Report".to_string(), ContentType::Pdf)],
            image_index: Some(14),
            tools: vec![
                "Color spaces".to_string(),
                "Matlab".to_string(),
                "Typescript".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Custom model for denoising images".to_string(),
            description: "Implemented my own deep learning convolutional neural network, DnCNN, for denoising images.".to_string(),
            link_paths: vec![("assets/DNCNN.pdf".to_string(), "Report".to_string(), ContentType::Pdf)],
            image_index: Some(15),
            tools: vec![
                "Python".to_string(),
                "ML".to_string(),
                "Deep Learning".to_string(),
                "Convolutional Neural Networks".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Multithreaded  MD5-based password cracker".to_string(),
            description:
                "Explored multithreading in Rust by implementing a MD5 based password cracker."
                    .to_string(),
            link_paths: vec![(
                "https://github.com/RasmusHogslatt/Password-Cracker".to_string(),
                "Github".to_string(),
                ContentType::Link,
            )],
            image_index: None,
            tools: vec!["Rust".to_string()],
            uuid: Uuid::new_v4(),
        });
        projects.push(Project {
            title: "Conway's Game of Life".to_string(),
            description: "For my first project in Rust, I implemented Conway's Game of Life. The user clicks on the tiles that should be active and then starts the simulation. It is fascinating how simple rules can produce very complex patterns."
                .to_string(),
            link_paths: vec![(
                "https://github.com/RasmusHogslatt/GameOfLife?tab=readme-ov-file".to_string(),
                "Github".to_string(),
                ContentType::Link,
            )],
            image_index: Some(16),
            tools: vec![
                "Rust".to_string(),
            ],
            uuid: Uuid::new_v4(),
        });
        let infos: Vec<(String, String, Option<usize>)> = vec![
            (
                "Driver's licenses:".to_string(),
                "Car, Motorcycle".to_string(),
                Some(11),
            ),
            ("Nationality:".to_string(), "Swedish".to_string(), Some(12)),
        ];

        let info = Info {
            infos,
            link_paths: vec![(
                "https://www.linkedin.com/in/rasmushogslatt/".to_string(),
                "LinkedIn".to_string(),
                ContentType::Link,
                Some(10),
            )],
            uuid: Uuid::new_v4(),
            birth_year: 1999,
        };
        let about_me = AboutMe {
            description: vec![("Hi, I'm Rasmus!".to_string(), None), ("I'm a software engineer from Sweden. My favourite programming language is rust, the language of the crabs!".to_string(), Some(6))
             ,("I play the guitar, solve Rubik's cube and love working out. I am also member Nr. 3150 of NAFS(K), the Swedish Donaldism association.".to_string(), Some(5))],
            uuid: Uuid::new_v4(),
        };

        Self {
            about_me,
            experiences,
            educations,
            projects,
            info,
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

        // egui::TopBottomPanel::top("top_panel")
        //     .resizable(false)
        //     .show(ctx, |ui| {
        //global_dark_light_mode_switch(ui);

        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "my_font".to_owned(),
            FontData::from_static(include_bytes!("../fonts/FiraCode-Regular.otf")),
        ); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());

        ctx.set_fonts(fonts);

        // Get current context style
        let mut style = (*ctx.style()).clone();
        style.visuals.widgets.noninteractive.bg_stroke.color = PRIMARY_ORANGE;
        style.visuals.extreme_bg_color = WHITE;
        style.visuals.panel_fill = BG_COLOR;

        style.visuals.widgets.noninteractive.rounding = Rounding {
            nw: 10.0,
            ne: 10.0,
            sw: 10.0,
            se: 10.0,
        };
        style.visuals.clip_rect_margin = 0.0;
        style.visuals.hyperlink_color = HYPERLINK_COLOR;

        // Mutate global style with above changes
        ctx.set_style(style);
        // });

        egui::SidePanel::left("left_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("left_scroll_area")
                    .auto_shrink(true)
                    .show(ui, |ui| {
                        ui.heading(
                            RichText::new("Education")
                                // .underline()
                                .strong()
                                .color(HEADING_COLOR),
                        );
                        for education in &self.educations {
                            ui.add(EducationWidget::new(education, &self.images));
                        }
                        ui.heading(
                            RichText::new("Experiences")
                                // .underline()
                                .strong()
                                .color(HEADING_COLOR),
                        );
                        for experience in &self.experiences {
                            ui.add(ExperienceWidget::new(experience, &self.images));
                        }
                    });
            });

        egui::SidePanel::right("right_panel")
            .exact_width(SIDE_PANEL_WIDTH)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("right_scroll_area")
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        ui.heading(
                            RichText::new("Projects")
                                // .underline()
                                .strong()
                                .color(HEADING_COLOR),
                        );
                        for project in &self.projects {
                            ui.add(ProjectWidget::new(project, &self.images));
                        }
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .id_source("center_scroll_area")
                .auto_shrink(false)
                .show(ui, |ui| {
                    ui.with_layout(
                        egui::Layout::top_down_justified(egui::Align::Center),
                        |ui| {
                            ui.add(AboutMeWidget::new(&self.about_me, &self.images));
                            ui.add(InfoWidget::new(&self.info, &self.images));
                        },
                    );
                });
        });
    }
}

pub fn open_pdf(file_path: String) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            let _ = window.open_with_url_and_target(format!("/{}", &file_path).as_str(), "_blank");
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
