use serde::{Deserialize, Serialize};
use eframe::egui::{Checkbox, Label, Ui};


#[derive(Clone,Serialize,Deserialize)]
pub struct Task{
  pub content:String,
  pub done:bool,
  //TODO tags?
  //TODO priority?
}

impl Task{
  pub fn draw_checkbox(&mut self, ui: &mut Ui){
    ui.horizontal(|ui|{
      ui.set_enabled(!self.done);
      ui.checkbox(&mut self.done,"");
      ui.scope(|ui|{
        ui.visuals_mut().override_text_color = if self.done {Some(eframe::egui::Color32::GRAY)} else {None};
        ui.label(&self.content);
      })
    });
  }
  pub fn draw_edit(&mut self, ui: &mut Ui){
    ui.horizontal(|ui|{
      ui.label("Task: ");
      ui.checkbox(&mut self.done,"");
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