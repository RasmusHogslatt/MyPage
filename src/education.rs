use egui::Widget;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub year: String,
}

impl Widget for Education {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        todo!()
    }
}
