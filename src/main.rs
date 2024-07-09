use eframe::egui;

mod views;
mod project;


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
        self.current_view.render(ui);
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
