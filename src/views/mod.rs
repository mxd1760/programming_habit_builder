use eframe::egui;

pub trait View{
  fn render(&mut self,ui:&mut egui::Ui);
}


mod home_view;
pub use home_view::*;