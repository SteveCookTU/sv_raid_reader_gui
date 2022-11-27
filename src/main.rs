#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::IconData;
use sv_raid_reader_gui::app::SVRaidReader;

const ICON_RAW: &[u8] = include_bytes!("../compass_icon.png");

fn main() {
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        icon_data: Some(load_icon()),
        vsync: false,
        ..Default::default()
    };
    eframe::run_native(
        "SV Raid Reader",
        native_options,
        Box::new(|cc| Box::new(SVRaidReader::new(cc))),
    );
}

fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_RAW).unwrap().into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}
