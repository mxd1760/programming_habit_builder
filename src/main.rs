use eframe::egui;

mod views;
mod data;
mod util;


struct MyApp{
  current_view:Box<dyn views::View>
}

impl Default for MyApp{
  fn default() -> Self {
    Self{
      current_view:Box::new(views::HomeView::new()),
    }
  }
}

impl eframe::App for MyApp{
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
      egui::TopBottomPanel::top("menus").show(ctx,|ui|{
        
      });
      egui::CentralPanel::default().show(ctx,|ui|{
        match self.current_view.render(ui){
          views::ViewEvent::SwitchView(new_view) => self.current_view = new_view,
          views::ViewEvent::DoNothing => (),
        }
      });
  }
}



fn main() -> Result<(),eframe::Error> {
  let native_options = eframe::NativeOptions{
    viewport:eframe::egui::ViewportBuilder::default().with_inner_size([600.,600.]),
    ..Default::default()
  };
  eframe::run_native(
    "My egui App",
    native_options,
    Box::new(|_|{
      Box::<MyApp>::default()
    }),
  )
}
