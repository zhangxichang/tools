use eframe::{
    egui,
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};
use eyre::Result;

pub struct Viewport {
    about_open: bool,
    android_key_generation_open: bool,
}
impl Viewport {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Result<Self> {
        cc.egui_ctx.add_font(FontInsert::new(
            "SourceHanSansCN-Normal",
            egui::FontData::from_static(include_bytes!(
                "../assets/fonts/SourceHanSansCN-Normal.otf"
            )),
            vec![
                InsertFontFamily {
                    family: egui::FontFamily::Proportional,
                    priority: FontPriority::Highest,
                },
                InsertFontFamily {
                    family: egui::FontFamily::Monospace,
                    priority: FontPriority::Highest,
                },
            ],
        ));
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Ok(Self {
            about_open: false,
            android_key_generation_open: false,
        })
    }
}
impl eframe::App for Viewport {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            ui.add_space(1.);
            ui.horizontal(|ui| {
                if ui.button("✨ 关于").clicked() {
                    self.about_open = true;
                }
            });
            ui.add_space(1.);
        });
        if self.about_open {
            egui::Modal::new("about".into()).show(ctx, |ui| {
                ui.heading("关于");
                ui.label("这里有各种各样的小工具，持续更新");
                if ui.button("关闭").clicked() {
                    self.about_open = false;
                }
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.toggle_value(&mut self.android_key_generation_open, "安卓密钥生成");
        });
        egui::Window::new("安卓密钥生成")
            .collapsible(false)
            .resizable([false, false])
            .open(&mut self.android_key_generation_open)
            .show(ctx, |ui| {
                ui.heading("内容");
            });
    }
}
