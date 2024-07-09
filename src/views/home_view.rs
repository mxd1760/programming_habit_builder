use crate::views::View;
use crate::project::Project;
use eframe::egui;

const PROJECT_FILE_EXTENTION: &str = ".txt";


pub struct HomeView{
  projects:Vec<Project>
}

impl HomeView{
  fn load_files() -> Vec<Project>{
    let mut out = vec![];
    if let Some(mut dir) = dirs::data_dir(){
      dir.push("MarcusDoucette");
      dir.push("Programming Habit Builder");
      std::fs::create_dir_all(&dir).expect("could not access data directory");
      //println!("MATCHED FILES:");
      for entry in walkdir::WalkDir::new(&dir){
        match entry{
          Ok(entry)=>{
            if entry.path().is_file(){
              //println!("\t{}",entry.path().display());
              out.push(Project::new(entry.path()))
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
      projects: Self::load_files()
    }
  }
}

impl View for HomeView{
  fn render(&mut self,ui: &mut egui::Ui){
    ui.label("Welcome home");
    for proj in &self.projects{
      proj.draw(ui);
    }
  }
}