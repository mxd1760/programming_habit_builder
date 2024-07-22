
pub fn get_data_dir() -> Option<std::path::PathBuf>{
  if let Some(mut dir) = dirs::data_dir(){
    dir.push("MarcusDoucette");
    dir.push("Programming Habit Builder");
    std::fs::create_dir_all(&dir).expect("could not access data directory");
    Some(dir)
  }else{
    None
  }
}