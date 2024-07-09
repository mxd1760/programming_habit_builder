use std::path::Path;

use eframe::egui::{self, Response};


pub struct Project{
  name:String
}

impl Project{
  pub fn draw(&self, ui: &mut egui::Ui)->Response{
    ui.button(&self.name)
  }
  pub fn new(path:&Path)->Self{
    Self { name: path.file_stem().expect("bad file").to_str().expect("bad file").into() }
  }
}