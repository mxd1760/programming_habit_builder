use std::path::Path;

use crate::{project::Project, views::View};

use super::ViewEvent;

pub struct ProjectView{
  project:Project
}

impl ProjectView{
  pub fn new(p: Project)->Self{
    Self { project: p }
  }
}

impl View for ProjectView{
  // TODO make edit mode
  // TODO make todo elements
  fn render(&mut self, ui:&mut eframe::egui::Ui) -> ViewEvent {
    if ui.button("back").clicked{
      return super::ViewEvent::SwitchView(Box::new(super::HomeView::new()))
    }
    ui.label(&self.project.name);

    super::ViewEvent::DoNothing
  }
}
