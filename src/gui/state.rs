use crate::app::{self, App};

use egui::{Context, Ui};
use fxhash::FxHasher32;
use std::{collections::HashMap, hash::Hasher};
use uuid::timestamp::context;

pub trait Widget: WidgetSystem + AsAny {}

#[derive(Default)]
pub struct State {
    pub widgets: HashMap<WidgetId, Box<dyn Widget>>,
    pub is_style_applied: bool,
    msaa_count: usize,

    rebuild_bundles: bool, // Controls whether to rebuild the render pipelines and texture views
}

impl State {
    pub fn get_widget_state_mut<T: 'static + Widget>(&mut self, id: WidgetId) -> &mut T {
        let widget = self.widgets.get_mut(&id).unwrap();
        widget.as_any_mut().downcast_mut::<T>().unwrap()
    }

    pub fn get_widget_state<T: 'static + Widget>(&self, id: WidgetId) -> &T {
        let widget = self.widgets.get(&id).unwrap();
        widget.as_any().downcast_ref::<T>().unwrap()
    }
}

pub trait WidgetSystem {
    fn system(
        app_state: &mut app::State,
        ui_state: &mut State,
        ui: Option<&mut Ui>,
        context: Option<&Context>,
        id: WidgetId,
    ) where
        Self: Sized;
    fn init(&mut self, app_state: &mut app::State)
    where
        Self: Sized;
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub fn widget<S: 'static + Widget + Default>(
    app_state: &mut app::State,
    ui_state: &mut State,
    ui: Option<&mut Ui>,
    context: Option<&Context>,
    id: WidgetId,
) {
    let mut cached_state;
    if !ui_state.widgets.contains_key(&id) {
        // debug!(
        //     "Registering system state for widget {id:?} of type {}",
        //     std::any::type_name::<S>()
        // );
        let mut s = S::default();
        s.init(app_state);
        ui_state.widgets.insert(id, Box::new(s));
        cached_state = ui_state.widgets.get_mut(&id).unwrap();
    } else {
        cached_state = ui_state.widgets.get_mut(&id).unwrap();
    }

    if let Some(cached_state) = cached_state.as_any_mut().downcast_mut::<S>() {
        S::system(app_state, ui_state, ui, context, id);
    } else {
        // print type name of cached_state
        panic!(
            "Widget state type mismatch for id {:?} and type {:?}",
            id,
            std::any::type_name::<S>()
        );
    }

    // S::system(app_state, cached_state, ui, id);
}
// trait DownCastable {
//     fn as_any(&self) -> &dyn std::any::Any;
//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
// }

// impl DownCastable for Box<dyn WidgetSystem> {
//     fn as_any(&self) -> &dyn std::any::Any {
//         self as &dyn std::any::Any
//     }

//     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
//         self as &mut dyn std::any::Any
//     }
// }

/// A UI widget may have multiple instances. We need to ensure the local state of these instances is
/// not shared. This hashmap allows us to dynamically store instance states.
// struct StateInstances<T: 'static + WidgetSystem> {
//     instances: HashMap<WidgetId, T>,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidgetId(pub u64);
impl WidgetId {
    pub fn new(name: &str) -> Self {
        let bytes = name.as_bytes();
        let mut hasher = FxHasher32::default();
        hasher.write(bytes);
        WidgetId(hasher.finish())
    }

    #[allow(unused)]
    pub fn with(&self, name: &str) -> WidgetId {
        Self::new(&format!("{}{name}", self.0))
    }
}
