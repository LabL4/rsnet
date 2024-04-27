use rsnet_derive::Widget;

use crate::{app, gui::state::WidgetSystem};

#[derive(Default, Widget)]
pub struct DebugGui {}

impl WidgetSystem for DebugGui {
    fn system(
        app_state: &mut crate::app::State,
        ui_state: &mut crate::gui::State,
        ui: Option<&mut egui::Ui>,
        context: Option<&egui::Context>,
        id: crate::gui::state::WidgetId,
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
                let frame_time = app_state.current_frame_time();

                ui.label("Frame Time");
                ui.label(format!("{:.2} ms", frame_time));
                ui.end_row();

                ui.label("FPS");
                ui.label(format!("{:.1}", 1e3 / frame_time));
                ui.end_row();

                ui.label("Num of primitives in storage buffer");
                ui.label(format!("{}", app_state.n_primitives_in_fragment_storage()));
                ui.end_row();

                ui.label("Num of wires in storage buffer");
                ui.label(format!("{}", app_state.n_wires_in_buffer()));
                ui.end_row();

                ui.label("Num of components in storage buffer");
                ui.label(format!("{}", app_state.n_components_in_buffer()));
                ui.end_row();

                ui.label("Chunk step idx");
                ui.label(format!("{}", app_state.chunk_step_idx()));
                ui.end_row();

                ui.label("Chunk size");
                ui.label(format!("{:.2}", app_state.chunk_size()));
                ui.end_row();

                ui.label("Screen chunk range");
                ui.label(format!("{:?}, {:?}", app_state.screen_chunk_range().min_chunk, app_state.screen_chunk_range().max_chunk));
                ui.end_row();

            });
    }

    fn init(&mut self, app_state: &mut crate::app::State) {}
}
