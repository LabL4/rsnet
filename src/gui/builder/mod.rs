use crate::app::State;

use egui::RichText;
use egui::{Color32, Context};

pub fn build(context: &Context, state: &mut State) {
    if !state.ui.is_style_applied {
        context.set_style(style());
        state.ui.is_style_applied = true;
    }

    egui::Window::new("Controls")
        .collapsible(false)
        .show(context, |ui| {
            let btn = egui::Button::new(
                RichText::new("Reset scene")
                .color(Color32::WHITE)
                .size(20.0)
                // .family("")
            );
            if ui.add(btn.fill(Color32::from_rgb((252.0*0.8) as u8, (186.0*0.8) as u8, 3))).clicked() {
               
            }
        });
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
