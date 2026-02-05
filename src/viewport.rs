use eframe::{
    egui,
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
};
use eyre::Result;

pub struct Viewport {
    about_open: bool,
    window_open: bool,
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
            window_open: false,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.group(|ui| {
                ui.label("按时工具");
                ui.toggle_value(&mut self.window_open, "一个工具");
            })
        });
        if self.about_open {
            egui::Modal::new("about".into()).show(ctx, |ui| {
                ui.heading("关于");
                ui.label("这是一个包含各种各样工具的网站");
                if ui.button("关闭").clicked() {
                    self.about_open = false;
                }
            });
        }
        egui::Window::new("窗口")
            .collapsible(false)
            .resizable([false, false])
            .open(&mut self.window_open)
            .show(ctx, |ui| {
                ui.heading("工具");
            });
    }
}
