use std::path::Path;

use eframe::egui::Button;

use crate::{data::{Project, Task}, views::View};

use super::ViewEvent;

enum PVMode{
  View,
  Edit,
}

pub struct ProjectView{
  mode:PVMode,
  project:Project
}

impl ProjectView{
  pub fn new(p: Project)->Self{
    Self { 
      mode:PVMode::View,
      project: p
    }
  }
}

impl View for ProjectView{
  fn render(&mut self, ui:&mut eframe::egui::Ui) -> ViewEvent {
    match self.mode{
      PVMode::View => {
        if ui.button("back").clicked(){
          return super::ViewEvent::SwitchView(Box::new(super::HomeView::new()))
        }
        ui.label(&self.project.name);
        
        // filter by match
        // TODO how to apply mutliple filters at once
          // nested loop with variable for pattern?
        for t in &mut self.project.tasks{
          if !t.done {
            t.draw_checkbox(ui);
          }
        }
        for t in &mut self.project.tasks{
          if t.done {
            t.draw_checkbox(ui);
          }
        }

        if ui.button("Edit").clicked(){
          self.mode = PVMode::Edit;
        }
      },
      PVMode::Edit =>{
        ui.label("please save before going back");
        ui.horizontal(|ui|{
          ui.label("Project Name: ");
          ui.text_edit_singleline(&mut self.project.name);
        });

        for t in &mut self.project.tasks{
          t.draw_edit(ui);
        }
        if ui.button("add").clicked(){
          self.project.tasks.push(Task::new());
        }
        if ui.button("Save").clicked(){
          if let Some(mut dir) = crate::util::get_data_dir(){
            _=self.project.save(&mut dir);
          }
          self.mode = PVMode::View;
        }
      }
    }

    super::ViewEvent::DoNothing
  }
}
