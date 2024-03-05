#![allow(unused)]

use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tray_icon::{
    menu::{AboutMetadata, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    TrayIconBuilder, TrayIconEvent,
};

use crate::core::config::Config;
use crate::core::prayer::Prayer;
use crate::core::prayertime::PrayerTime;

pub mod core;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/icon.png");
    let icon = load_icon(std::path::Path::new(path));

    let event_loop = EventLoopBuilder::new().build();

    let tray_menu = Menu::new();

    let quit_i = MenuItem::new("Quit", true, None);
    tray_menu.append_items(&[
        &PredefinedMenuItem::about(
            None,
            Some(AboutMetadata {
                name: Some("tao".to_string()),
                copyright: Some("Copyright tao".to_string()),
                ..Default::default()
            }),
        ),
        &PredefinedMenuItem::separator(),
        &quit_i,
    ]);

    let mut tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_tooltip("tao - awesome windowing lib")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    let prayers = vec![PrayerTime::new("Fajr", false, 4, 54),
                       PrayerTime::new("dhuhr", true, 12, 2),
                       PrayerTime::new("Asr", true, 3, 27),
                       PrayerTime::new("Maghrib", true, 5, 58),
                       PrayerTime::new("Isha", true, 6, 44)];

    let mut prayer = Prayer::new(Config {prayers});

    event_loop.run(move |_event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        prayer.handle_time();

        if let Ok(event) = menu_channel.try_recv() {
            if event.id == quit_i.id() {
                tray_icon.take();
                *control_flow = ControlFlow::Exit;
            }
        }

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
    })
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}