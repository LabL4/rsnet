use std::borrow::Borrow;

use rsnet_derive::Widget;
use tracing::info;

use crate::gui::state::{widget, WidgetId, WidgetSystem};

use super::settings::Settings;

#[derive(Default, Widget)]
pub struct TopPanel {
    settings_open: bool,
    debug_open: bool,
}

impl WidgetSystem for TopPanel {
    fn system(
        app_state: &mut crate::app::State,
        ui_state: &mut crate::gui::State,
        ui: Option<&mut egui::Ui>,
        context: Option<&egui::Context>,
        id: crate::gui::state::WidgetId,
    ) {

        if !context.is_some() {
            return;
        }

        let context = context.unwrap();

        let state = ui_state.get_widget_state_mut::<Self>(id);
        egui::TopBottomPanel::top("Top Panel").show(context, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(state.settings_open, "Settings").clicked() {
                    state.settings_open = !state.settings_open;
                }
    
                if ui.selectable_label(state.debug_open, "Debug").clicked() {
                    state.debug_open = !state.debug_open;
                }
            });

        });
        
        let state = ui_state.get_widget_state::<Self>(id);
        let settings_open = state.settings_open;
        let debug_open = state.debug_open;

        if settings_open {
            let mut open = settings_open;
            egui::Window::new("Settings")
                .collapsible(true)
                .open(&mut open)
                .show(context, |ui| {
                    widget::<Settings>(app_state, ui_state, Some(ui), Some(context), WidgetId::new("Settings"));
                });
            if !open {
                let state = ui_state.get_widget_state_mut::<Self>(id);
                state.settings_open = false;
            }
        }

        if debug_open {
            let mut open = debug_open;
            egui::Window::new("Debug")
                .collapsible(true)
                .open(&mut open)
                .show(context, |ui| {
                    widget::<crate::gui::builder::debug_gui::DebugGui>(app_state, ui_state, Some(ui), Some(context), WidgetId::new("Debug"));
                });
            if !open {
                let state = ui_state.get_widget_state_mut::<Self>(id);
                state.debug_open = false;
            }
        }

        // egui::Window::new("Settings")
        //     .collapsible(false)
        //     .frame(frame)
        //     .show(ui.ctx(), |ui| {
        //         widget::<Settings>(app_state, ui_state, ui, WidgetId::new("Settings"));
        //     });
        // egui::CentralPanel::default().show(ui.ctx(), |ui| {
        //     egui::Window::new("Settings")
        //         .collapsible(false)
        //         .frame(frame)
        //         .show(ui.ctx(), |ui| {
        //             widget::<Settings>(app_state, ui_state, ui, WidgetId::new("Settings"));
        //         });
        //     // widget::<Settings>(app_state, ui_state, ui, WidgetId::new("Settings"));
        // });
        // widget::<Settings>(app_state, ui_state, ui, WidgetId::new("Settings"));
    }

    fn init(&mut self, app_state: &mut crate::app::State) {}
}
