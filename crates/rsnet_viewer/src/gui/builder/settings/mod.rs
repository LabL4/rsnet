pub mod aa_selector;

use aa_selector::AaSelector;

use crate::gui::{
    state::{widget, WidgetId, WidgetSystem},
    widgets::toggle_switch,
};
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
                widget::<AaSelector>(
                    app_state,
                    ui_state,
                    Some(ui),
                    context,
                    WidgetId::new("Aa Selector"),
                );
                ui.end_row();

                let mut grid = app_state.grid();

                ui.add(egui::Label::new("Grid"));

                ui.add(toggle_switch::toggle(&mut grid))
                    .on_hover_text("Toggle grid visibility.");

                if grid != app_state.grid() {
                    app_state.set_grid(grid);
                }
            });
    }

    fn init(&mut self, app_state: &mut crate::app::State)
    where
        Self: Sized,
    {
    }
}
