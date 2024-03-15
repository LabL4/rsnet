use egui::Ui;

use crate::{
    app,
    gui::state::{WidgetId, WidgetSystem},
};
use rsnet_derive::Widget;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
enum MsaaCount {
    #[default]
    One,
    Two,
    Four,
    Eight,
}

// impl from u32
impl From<u32> for MsaaCount {
    fn from(value: u32) -> Self {
        match value {
            1 => MsaaCount::One,
            2 => MsaaCount::Two,
            4 => MsaaCount::Four,
            8 => MsaaCount::Eight,
            _ => MsaaCount::One,
        }
    }
}

impl MsaaCount {
    fn to_u32(&self) -> u32 {
        match self {
            MsaaCount::One => 1,
            MsaaCount::Two => 2,
            MsaaCount::Four => 4,
            MsaaCount::Eight => 8,
        }
    }
}

#[derive(Widget, Default)]
pub struct AaSelector {
    msaa_count: MsaaCount,
}

impl WidgetSystem for AaSelector {
    fn system(
        app_state: &mut app::State,
        ui_state: &mut crate::gui::State,
        ui: Option<&mut Ui>,
        context: Option<&egui::Context>,
        id: WidgetId,
    ) {
        if !ui.is_some() {
            return;
        }

        let ui = ui.unwrap();

        let state = ui_state.get_widget_state_mut::<Self>(id);
        let mut msaa_count = &mut state.msaa_count;

        egui::ComboBox::from_label("")
            .selected_text(format!("x{:?}", msaa_count.to_u32()))
            // .width(ui.available_width() * 0.5)
            .show_ui(ui, |ui| {
                ui.selectable_value(msaa_count, MsaaCount::One, "1");
                ui.selectable_value(msaa_count, MsaaCount::Two, "2");
                ui.selectable_value(msaa_count, MsaaCount::Four, "4");
                ui.selectable_value(msaa_count, MsaaCount::Eight, "8");
            });

        // ui.horizontal(|ui| {
        //     ui.selectable_value(msaa_count, MsaaCount::One, "1");
        //     ui.selectable_value(msaa_count, MsaaCount::Two, "2");
        //     ui.selectable_value(msaa_count, MsaaCount::Four, "4");
        //     ui.selectable_value(msaa_count, MsaaCount::Eight, "8");
        // });

        if msaa_count.to_u32() != app_state.msaa_count() {
            app_state.set_msaa_count(msaa_count.to_u32());
        }
    }

    fn init(&mut self, app_state: &mut app::State) {}
}

// impl AsAny for AaSelector {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self
//     }

//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self
//     }
// }

// impl Widget for AaSelector{
// }
