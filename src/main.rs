use eframe::{egui, HardwareAcceleration, Theme};
use core::default::Default;
use std::env::current_exe;
use egui::{Color32, Context, Id, Sense, Vec2, Visuals};
use egui::ecolor::rgb_from_hsv;


fn main() {
let nativeoption=eframe::NativeOptions{
    always_on_top: false,
    maximized: false,
    decorated: false,
    fullscreen: false,
    drag_and_drop_support: true,
    icon_data: None,
    initial_window_pos: None,
    initial_window_size: Option::from(
        Vec2::new(300f32,300f32)
    ),
    min_window_size: None,
    max_window_size: None,
    resizable: true,
    transparent: false,
    mouse_passthrough: false,
    active: false,
    vsync: true,
    multisampling: 0,
    depth_buffer: 0,
    stencil_buffer: 0,
    hardware_acceleration: HardwareAcceleration::Off,
    renderer: Default::default(),
    follow_system_theme: false,
    default_theme: Theme::Dark,
    run_and_return: false,
    event_loop_builder: None,
    window_builder: None,
    shader_version: None,
    centered: false,
    app_id: None,
    persist_window: false,
};
    eframe::run_native("my egui app", nativeoption, Box::new(|cc| Box::new(MyEguiApp::new(cc)))).expect("TODO: panic message");

}
#[derive(Default)]
struct MyEguiApp{
    name:String,
    age:String,

}


impl MyEguiApp {
    fn new(cc:&eframe::CreationContext<'_>) -> Self {
       MyEguiApp {
           name:String::new(),
           age:String::new(),

       };
        Self::default()
    }
}


impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx,|ui| {
           if ui.interact(ui.max_rect(),Id::new("window-drag"),Sense::drag()).dragged() {
               frame.drag_window();
           }
         ui.heading("hello  welcome to egui");

           ui.separator();

ui.horizontal(|ui| {


    let _ = ui.button("HOME");
    let _ = ui.button("ABOUT");
    let _ = ui.button("RESUME");
    let _ = ui.button("CONTACT");
});
           ui.horizontal(|ui| {
               ui.label("Enter your name");
              let res=  ui.text_edit_singleline(&mut self.name);
           });

           ui.horizontal(|ui| {

               ui.label("enter your age");
               ui.text_edit_singleline(&mut self.age);

          });
               ui.label(format!("my name is {} and my age is {}",&self.name,&self.age));

           ui.separator();

           egui::CollapsingHeader::new("show")
               .show(ui,|ui| {
                   ui.button("click me ");
                   ui.spinner()
               })











       });
    }
}