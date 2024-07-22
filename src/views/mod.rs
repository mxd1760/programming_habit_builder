use eframe::egui;

pub trait View{
  fn render(&mut self,ui:&mut egui::Ui) -> ViewEvent;
}

pub enum ViewEvent{
  SwitchView(Box<dyn View>),
  DoNothing,
}

mod home_view;
pub use home_view::*;

mod project_view;
pub use project_view::*;