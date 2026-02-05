use eyre::Result;

use crate::viewport::Viewport;

#[cfg(not(target_family = "wasm"))]
pub fn run(name: impl AsRef<str>) -> Result<()> {
    use eframe::egui;
    use eyre::eyre;
    use std::sync::Arc;

    flexi_logger::Logger::with(flexi_logger::LogSpecification::info())
        .log_to_file(
            flexi_logger::FileSpec::default()
                .directory("logs")
                .suppress_basename(),
        )
        .format(flexi_logger::json_format)
        .rotate(
            flexi_logger::Criterion::Size(1024 * 1024),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepCompressedFiles(10),
        )
        .start()
        .unwrap();
    log::info!("日志开始记录");
    eframe::run_native(
        name.as_ref(),
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_icon(Arc::new(eframe::icon_data::from_png_bytes(
                    include_bytes!("../assets/images/icon.png"),
                )?))
                .with_inner_size(egui::Vec2::new(800., 600.)),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(Viewport::new(cc)?))),
    )
    .map_err(|err| eyre!("{}", err))?;
    Ok(())
}

#[cfg(target_family = "wasm")]
pub fn run(name: impl AsRef<str>) -> Result<()> {
    use web_sys::{HtmlCanvasElement, wasm_bindgen::JsCast, window};

    wasm_bindgen_futures::spawn_local({
        let name = name.as_ref().to_string();
        async move {
            console_error_panic_hook::set_once();
            wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
            log::info!("日志开始记录");
            let document = window()
                .expect("window不存在")
                .document()
                .expect("document不存在");
            document.set_title(&name);
            let canvas = document
                .get_element_by_id("viewport")
                .expect("viewport不存在")
                .dyn_into::<HtmlCanvasElement>()
                .expect("viewport不是canvas元素");
            eframe::WebRunner::new()
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(|cc| Ok(Box::new(Viewport::new(cc)?))),
                )
                .await
                .expect("eframe运行出错");
        }
    });
    Ok(())
}
