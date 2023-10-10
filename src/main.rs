use eframe::{egui, HardwareAcceleration, Theme};
use eframe::epaint::Color32;
use egui::{Id, Sense, Vec2,RichText};

use webbrowser;

fn main() {
    let nativeoption = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(Vec2::new(300f32, 300f32)),
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
    eframe::run_native(
        "Portfolio",
        nativeoption,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    )
    .expect("TODO: panic message");
}
#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn open_url(&self, url: &str) {
        if let Err(err) = webbrowser::open(url) {
            eprintln!("failed to open link {:?} ", err);
        }
    }
}
impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {



        egui::CentralPanel::default().show(ctx,|ui| {

         egui::ScrollArea::vertical().show(ui,|ui| {



           if ui.interact(ui.max_rect(),Id::new("window-drag"),Sense::drag()).dragged() {
               frame.drag_window();
           }

           ui.heading("Hi! I am pretham Muthappa");

           ui.separator();
           ui.horizontal(|ui| {

        let _= ui.button("HOME");
       if ui.button("ABOUT").clicked() {
        self.open_url("https://tidersky.me");
        }

        if ui.button("RESUME").clicked() {
        self.open_url("https://drive.google.com/file/d/1jgWFzMDIrpW9odLQMoljTGtEeofRvf6W/view");
        }

        if ui.button("CONTACT").clicked() {

          self.open_url("https://tidersky.me")

       } });

               ui.separator();

              ui.heading("About");

              ui.label("Hello I am pretham muthappa , I am a final year computer science student. I like coding and creating open source projects and apart from coding i do pixel art illstration");

              ui.separator();
              ui.heading("project");

             egui::CollapsingHeader::new("SnapCrawler")
                 .show(ui,|ui| {
                     ui.label("Snapercrawler is a webscrapper tool built using Nodejs and puppeteer , it can scrape any images from most of the website and save it in a directory")
                 });

           egui::CollapsingHeader::new("Event-Hive")
               .show(ui,|ui| {
                   ui.label("Event-Hive is a full stack concert management app, it can perform CRUD operation, built using python and flask using SQLite as a db")
               });

           egui::CollapsingHeader::new("Geek-Notes")
               .show(ui,|ui| {
                   ui.label("An android app used for providing resources to student such as books , ebooks , pdf and textbook , Built using python and kivy framework")
               });

           egui::CollapsingHeader::new("Vulkan-rust")
               .show(ui,|ui| {
                   ui.label("Built a simple rust app using vulkano for the purpose of studying vulkan")
               });

           ui.separator();

           ui.heading(RichText::new("Tech Stack").color(Color32::WHITE));



         });

          });


    }
}
