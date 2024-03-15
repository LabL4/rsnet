pub mod aa_selector;

use aa_selector::AaSelector;

use crate::gui::state::{widget, WidgetId, WidgetSystem};
use rsnet_derive::Widget;

#[derive(Debug, Default, Widget)]
pub struct Settings {}

impl WidgetSystem for Settings {
    fn system(
        app_state: &mut crate::app::State,
        ui_state: &mut crate::gui::State,
        ui: Option<&mut egui::Ui>,
        context: Option<&egui::Context>,
        id: WidgetId,
    ) {
        if !ui.is_some() {
            return;
        }

        let ui = ui.unwrap();

        egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.add(egui::Label::new("MSAA"));
                    widget::<AaSelector>(app_state, ui_state, Some(ui), context, WidgetId::new("Aa Selector"));
                    ui.end_row();
                });

    }

    fn init(&mut self, app_state: &mut crate::app::State)
    where
        Self: Sized,
    {
    }
}
