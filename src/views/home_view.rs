use crate::views::View;
use crate::project::Project;
use eframe::egui;

use super::ViewEvent;

const PROJECT_FILE_EXTENTION: &str = ".txt";


pub struct HomeView{
  projects:Vec<Project>,
  temp_new_proj_name:String,
}

impl HomeView{
  fn load_files() -> Vec<Project>{
    let mut out = vec![];
    if let Some(mut dir) = crate::util::get_data_dir(){
      //println!("MATCHED FILES:");
      for entry in walkdir::WalkDir::new(&dir){
        match entry{
          Ok(entry)=>{
            if entry.path().is_file(){
              //println!("\t{}",entry.path().display());
              match Project::load(entry.path()){
                Some(project) => out.push(project),
                None => (),
              }
            }
            
          }
          _=>()
        }
      };
    }
    out
  }

  pub fn new()->Self{
    Self { 
      projects: Self::load_files(),
      temp_new_proj_name: "".into(),
    }
  }
}

impl View for HomeView{
  fn render(&mut self,ui: &mut egui::Ui) -> ViewEvent{
    ui.label("Welcome home");
    for proj in &self.projects{
      if proj.draw_as_button(ui).clicked(){
        return ViewEvent::SwitchView(Box::new(super::ProjectView::new(proj.clone())));
      };
    }; 
    ui.horizontal(|ui|{
      if ui.button("new").clicked(){
        let mut new_proj = Project::new(self.temp_new_proj_name.clone());
        if let Some(mut dir) = crate::util::get_data_dir(){
          _=new_proj.save(&mut dir);
        }
        self.projects.push(new_proj);
      }
      ui.text_edit_singleline(&mut self.temp_new_proj_name);
    });

    ViewEvent::DoNothing
  }
  
}