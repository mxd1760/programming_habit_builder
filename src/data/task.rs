use serde::{Deserialize, Serialize};
use eframe::egui::Ui;


#[derive(Clone,Serialize,Deserialize)]
pub struct Task{
  content:String,
  done:bool,
  //TODO tags?
  //TODO priority?
}

impl Task{
  pub fn draw_checkbox(&mut self, ui: &mut Ui){
    ui.checkbox(&mut self.done, &self.content);
  }
  pub fn draw_edit(&mut self, ui: &mut Ui){
    ui.horizontal(|ui|{
      ui.label("Task: ");
      ui.text_edit_singleline(&mut self.content);
    });
    
  }
  pub fn new()->Self{
    Self { 
      content: "".to_string(), 
      done: false 
    }
  }
}