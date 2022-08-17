use eframe::{
    egui::{self, CentralPanel, Color32, CtxRef, FontDefinitions, FontFamily, Label, Separator},
    epi::App,
    run_native, NativeOptions,
};
use std::borrow::Cow;

use std::thread;

use std::process::Command;
pub struct Headlinex {
    pub video_start: String,
    pub video_end: String,
}
impl Default for Headlinex {
    fn default() -> Self {
        Self {
            video_start: Default::default(),
            video_end: Default::default(),
        }
    }
}
impl Headlinex {
    fn new() -> Self {
        Default::default()
    }
    fn configure_fonts(&self, ctx: &CtxRef) {
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "simhei".to_string(),
            Cow::Borrowed(include_bytes!("./simhei.ttf")),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Heading,
            (FontFamily::Proportional, 28.),
        );
        font_def.family_and_size.insert(
            eframe::egui::TextStyle::Body,
            (FontFamily::Proportional, 18.),
        );

        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "simhei".to_string());
        ctx.set_fonts(font_def);
    }
}
impl App for Headlinex {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &mut eframe::epi::Frame<'_>,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(ctx)
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                let head = Label::new("视频转换首页").text_style(eframe::egui::TextStyle::Heading);
                ui.colored_label(Color32::from_rgb(0, 0, 255), head);
            });
            ui.add_space(15.);
            let sep = Separator::default();
            ui.add(sep);

            //视频开始地址
            ui.add_space(15.);
            ui.add(Label::new("请输入视频地址：回车结束"));

            let input_video_start = ui.text_edit_singleline(&mut self.video_start);

            if input_video_start.lost_focus() & ui.input().key_pressed(egui::Key::Enter) {
                println!("start: {}", self.video_start);
            }

            //视频结束地址
            ui.add_space(15.);

            ui.add(Label::new("要保存的地址：回车结束"));
            let input_video_end = ui.text_edit_singleline(&mut self.video_end);

            if input_video_end.lost_focus() & ui.input().key_pressed(egui::Key::Enter) {
                println!("end: {}", self.video_end);
            }

            //开始转换
            ui.add_space(15.);
            let btn_increment = ui
                .add(
                    egui::Button::new("开始转换")
                        .text_style(egui::TextStyle::Body)
                        .text_color(Color32::from_rgb(0, 255, 0)),
                )
                .on_hover_cursor(egui::CursorIcon::PointingHand);

            if btn_increment.clicked() {
                println!("开始转换");
                thread::spawn(||{
                    let mut command_video =  Command::new("ffmpeg");
                    println!("线程开始了");
                    let result = command_video.status().unwrap();
                    println!("{}",result);
                    
                });
            }
            ui.label(format!(
                "转换视频地址：{}\n视频保存地址：{}",
                self.video_start, self.video_end
            ));
        });
    }
    fn name(&self) -> &str {
        "视频转换"
    }
}

fn main() {
    let app = Headlinex::new();
    let win_option = NativeOptions::default();
    run_native(Box::new(app), win_option);
    let a = String::from("sdfdsa");
    let c = &a;
    println!("{}",a);
}
