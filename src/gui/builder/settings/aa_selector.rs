use egui::Ui;
use smaa::SmaaMode;

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

fn smaa_mode_to_str<'a>(mode: &'a SmaaMode) -> &'a str {
    match mode {
        SmaaMode::Disabled => "Disabled",
        SmaaMode::Smaa1X => "1x",
        _ => "Disabled"
    }
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

#[derive(Widget)]
pub struct AaSelector {
    msaa_count: MsaaCount,
    smaa_mode: SmaaMode
}

impl Default for AaSelector {
    fn default() -> Self {
        Self {
            msaa_count: MsaaCount::One,
            smaa_mode: SmaaMode::Disabled
        }
    }
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
        let mut smaa_mode = &mut state.smaa_mode;

        ui.add(egui::Label::new("MSAA"));

        egui::ComboBox::new(0, "")
            .selected_text(format!("x{:?}", msaa_count.to_u32()))
            // .width(ui.available_width() * 0.5)
            .show_ui(ui, |ui| {
                ui.selectable_value(msaa_count, MsaaCount::One, "1");
                ui.selectable_value(msaa_count, MsaaCount::Two, "2");
                ui.selectable_value(msaa_count, MsaaCount::Four, "4");
                ui.selectable_value(msaa_count, MsaaCount::Eight, "8");
            });

        ui.end_row();

        ui.add(egui::Label::new("SMAA"));

        egui::ComboBox::new(1, "")
            .selected_text(smaa_mode_to_str(smaa_mode))
            // .width(ui.available_width() * 0.5)
            .show_ui(ui, |ui| {
                ui.selectable_value(smaa_mode, SmaaMode::Disabled, "Disabled");
                ui.selectable_value(smaa_mode, SmaaMode::Smaa1X, "1x");
            });


        if msaa_count.to_u32() != app_state.msaa_count() {
            app_state.set_msaa_count(msaa_count.to_u32());
        }

        if smaa_mode != &app_state.smaa_mode() {
            app_state.set_smaa_mode(*smaa_mode);
        }




    }

    fn init(&mut self, app_state: &mut app::State) {
        self.msaa_count = app_state.msaa_count().into();
        self.smaa_mode = app_state.smaa_mode();
    }
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
