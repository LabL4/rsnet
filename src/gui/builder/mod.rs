mod debug_gui;
mod settings;
mod top_panel;

use crate::{app, gui};

use egui::RichText;
use egui::{Color32, Context};

use self::settings::Settings;
use self::top_panel::TopPanel;

use super::state::{widget, WidgetId};

pub fn build(context: &Context, app_state: &mut app::State, ui_state: &mut gui::State) {
    if !ui_state.is_style_applied {
        context.set_style(style());
        ui_state.is_style_applied = true;
    }

    // egui::Window::new("Settings")
    //     .collapsible(false)
    //     .show(context, |ui| {
    //         widget::<Settings>(app_state, ui_state, ui, WidgetId::new("Settings"));
    //     });

    widget::<TopPanel>(
        app_state,
        ui_state,
        None,
        Some(context),
        WidgetId::new("Top Panel"),
    );
}

pub fn style() -> egui::Style {
    let mut visuals = egui::Visuals::dark();
    visuals.window_shadow = egui::epaint::Shadow {
        extrusion: 0.0,
        color: egui::Color32::TRANSPARENT,
        ..Default::default()
    };
    let mut style = egui::Style::default();
    style.visuals = visuals;
    style
}
