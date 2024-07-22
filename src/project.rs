use std::path::{Path, PathBuf};
use serde::{Serialize,Deserialize};

use eframe::egui::{self, Response};


#[derive(Clone,Serialize,Deserialize)]
pub struct Project{
  pub name:String
}

impl Project{
  pub fn draw_as_button(&self, ui: &mut egui::Ui)->Response{
    ui.button(&self.name)
  }
  pub fn load(path:&Path)->Option<Self>{
    let reader: std::fs::File;
    match std::fs::File::open(path){
      Ok(r) => reader = r,
      Err(e) => {
        println!("Cannot open File: {}",e);
        println!("Deleting invalid file");
        std::fs::remove_file(path);
        return None;
      }
    }
    let project:Self;
    match serde_yaml::from_reader(reader){
      Ok(p) => project = p,
      Err(e) => {
        println!("File contents corrupted: {}",e);
        println!("Deleting invalid file");
        std::fs::remove_file(path);
        return None;
      }
    }
    Some(project)
  }

  pub fn save(&mut self,path:&mut PathBuf) -> Result<i32,Box<dyn std::error::Error>>{
    path.push(format!("{}.proj",self.name.clone()));
    let writer = std::fs::File::create(path)?;
    serde_yaml::to_writer(writer, self)?;
    Ok(0)
  }
  pub fn new(name:String)->Self{
    Self { name: name }
  }
}