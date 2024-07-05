use super::camera::Camera;
use crate::{app::utils::chunk_size_from_step_idx, utils::AaBb};

use nalgebra::{
    ComplexField, Matrix4, Perspective3, Point3, RealField, UnitQuaternion, Vector2, Vector3,
};
use std::f32::consts::FRAC_PI_2;
use tracing::{debug, info};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent},
    keyboard::ModifiersState,
};

#[derive(Default, Debug, Eq, PartialEq)]
enum ButtonState {
    #[default]
    Released,
    Pressed,
}

impl From<ElementState> for ButtonState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => ButtonState::Pressed,
            ElementState::Released => ButtonState::Released,
        }
    }
}

#[derive(Default, Debug)]
struct Position {
    x: f64,
    y: f64,
}

impl Position {
    fn is_eq_integer(&self, x: f64, y: f64) -> bool {
        (self.x as usize) == (x as usize) && (self.y as usize) == (y as usize)
    }
}

impl From<PhysicalPosition<f64>> for Position {
    fn from(value: PhysicalPosition<f64>) -> Self {
        Position {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Default, Debug)]
struct ButtonDragState {
    prev_state: ButtonState,
    is_dragging: bool,
    drag_start_pos: Option<Position>,
}

#[derive(Default, Debug)]
struct MouseDragState {
    left: ButtonDragState,
    right: ButtonDragState,
}

#[derive(Default, Debug)]
struct MouseButtonsState {
    pub left: ButtonState,
    pub right: ButtonState,
}

#[derive(Default, Debug)]
struct MouseState {
    buttons: MouseButtonsState,
    position: PhysicalPosition<f64>,
}

struct PerspectiveParams {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl From<PerspectiveParams> for Perspective3<f32> {
    fn from(value: PerspectiveParams) -> Self {
        Perspective3::new(value.aspect, value.fovy, value.znear, value.zfar)
    }
}

impl From<&PerspectiveParams> for Perspective3<f32> {
    fn from(value: &PerspectiveParams) -> Self {
        Perspective3::new(value.aspect, value.fovy, value.znear, value.zfar)
    }
}

const MIN_RADIUS: f32 = 0.01;
const MAX_RADIUS: f32 = 20000.0;

/// Controller for the camera
pub struct CameraController {
    mouse_drag_state: MouseDragState,
    current_mouse: MouseState,
    camera: Camera,
    total_rotation: UnitQuaternion<f32>,
    total_translation: Vector3<f32>,
    upside_down: bool,
    radius: f32,
    vert_angle: f32,
    horiz_angle: f32,

    last_mouse_pos_translation: Option<Position>,
    last_mouse_pos_rotation: Option<Position>,

    /// Eye of the camera (where it looks from)
    eye: Vector3<f32>,
    /// Orbit point (where it looks at)
    center: Vector3<f32>,
    /// Up direction (controls tilt)
    up: Vector3<f32>,
    perspective_params: PerspectiveParams,

    radius_sensitivity: f32,
    translate_sensitivity: f32,
    modifiers: ModifiersState,

    pub window_size: PhysicalSize<u32>,
    pub is_dirty: bool,

    pub screen_world_aabb: AaBb,
    pub chunk_size: f32,
    pub chunk_step_idx: usize,
    chunk_size_step: f32,
    base_chunk_size: f32,
}
impl CameraController {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        let radius = 10.0;

        let eye = Vector3::new(0.0, 0.0, -radius);
        let center = Vector3::new(0.0, 0.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);

        let view_matrix = Matrix4::new(
            1.0, 0.0, 0.0, eye[0], 0.0, 1.0, 0.0, eye[1], 0.0, 0.0, 1.0, eye[2], 0.0, 0.0, 0.0, 1.0,
        );

        let perspective_params = PerspectiveParams {
            aspect: (window_size.width as f32 / window_size.height as f32),
            fovy: 3.14 / 4.0,
            znear: 0.001,
            zfar: 100000.0,
        };
        let proj = Perspective3::new(
            perspective_params.aspect,
            perspective_params.fovy,
            perspective_params.znear,
            perspective_params.zfar,
        );

        let screen_world_aabb = get_ss_aabb(&proj, radius, &center);

        let camera = Camera::new(view_matrix, screen_world_aabb.clone(), proj);
        let mouse_drag_state = MouseDragState::default();
        let current_mouse = MouseState::default();
        let total_rotation = UnitQuaternion::default();
        let total_translation = Vector3::new(0.0, 0.0, 0.0);
        let upside_down = false;
        let vert_angle = 0.0;
        let horiz_angle = 0.0;

        let radius_sensitivity = 0.01;
        let translate_sensitivity = 0.01;

        let is_dirty = true;

        let last_mouse_pos_translation = None;
        let last_mouse_pos_rotation = None;
        let modifiers = ModifiersState::default();

        let chunk_size_step = 100.0;
        let base_chunk_size = 10.0;

        let (chunk_size, chunk_step_idx) =
            chunk_size_from_radius(radius, chunk_size_step, base_chunk_size);

        Self {
            mouse_drag_state,
            current_mouse,
            camera,
            total_rotation,
            total_translation,
            upside_down,
            radius,
            vert_angle,
            horiz_angle,

            last_mouse_pos_translation,
            last_mouse_pos_rotation,
            modifiers,

            eye,
            center,
            up,
            perspective_params,

            radius_sensitivity,
            translate_sensitivity,

            window_size,
            is_dirty,

            screen_world_aabb,
            chunk_size,
            chunk_step_idx,
            chunk_size_step,
            base_chunk_size,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    fn check_upside_down(&mut self) {
        self.upside_down = self.vert_angle.abs() > FRAC_PI_2;
    }

    fn update_radius(&mut self, delta: MouseScrollDelta) {
        let camera_right = self.camera.get_right_vector();
        let camera_up = self.camera.get_up_vector();

        match delta {
            MouseScrollDelta::LineDelta(_delta_x, delta_y) => {
                let delta_radius = delta_y * self.radius_sensitivity * 5.0 * self.radius;
                // let new_radius = self.radius + delta_radius;
                let new_radius = (self.radius + delta_radius).min(MAX_RADIUS).max(MIN_RADIUS);

                let diff = mouse_diff_from_radius(
                    &self.current_mouse.position,
                    &self.window_size,
                    &self.camera.get_perspective(),
                    self.radius,
                    new_radius,
                );

                self.center = self.center + camera_right * (diff).x + camera_up * (diff).y;
                self.radius = new_radius;
            }
            MouseScrollDelta::PixelDelta(delta) => {
                if self.modifiers.control_key() {
                    let delta_radius = (delta.y as f32) * self.radius_sensitivity * self.radius;
                    let new_radius = (self.radius + delta_radius).min(MAX_RADIUS).max(MIN_RADIUS);

                    let diff = mouse_diff_from_radius(
                        &self.current_mouse.position,
                        &self.window_size,
                        &self.camera.get_perspective(),
                        self.radius,
                        new_radius,
                    );

                    self.center = self.center + camera_right * (diff).x + camera_up * (diff).y;
                    self.radius = new_radius;
                } else {
                    let width_half = self.window_size.width as f32 / 2.0;
                    let height_half = self.window_size.height as f32 / 2.0;

                    let projection = self.camera.get_perspective();

                    let projected_z = projected_z(self.radius, &projection);

                    let start_ndc_point = Point3::new(0.0, 0.0, projected_z);
                    let end_ndc_point = Point3::new(
                        (delta.x as f32 + width_half - width_half) / width_half,
                        (delta.y as f32 + height_half - height_half) / height_half,
                        projected_z,
                    );

                    let start_unproj = unproject_point(projection, &start_ndc_point);
                    let end_unproj = unproject_point(projection, &end_ndc_point);

                    let diff = (end_unproj - start_unproj);

                    self.center =
                        self.center + camera_right.normalize() * (diff).x - camera_up * (diff).y;

                    // self.center += self.camera.get_right_vector() * delta.x as f32 * 0.03
                    //     - self.camera.get_up_vector() * delta.y as f32 * 0.03;
                }
            }
        }

        self.radius = self.radius.max(MIN_RADIUS).min(MAX_RADIUS);

        let (chunk_size, chunk_step_idx) =
            chunk_size_from_radius(self.radius, self.chunk_size_step, self.base_chunk_size);
        self.chunk_size = chunk_size;
        self.chunk_step_idx = chunk_step_idx;

        self.update_view_matrix();
    }

    fn update_mouse_press(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => {
                self.current_mouse.buttons.left = state.into();
                check_button_drag(
                    &mut self.mouse_drag_state.left,
                    state,
                    &self.current_mouse.position.into(),
                );
            }
            MouseButton::Right => {
                self.current_mouse.buttons.right = state.into();
                check_button_drag(
                    &mut self.mouse_drag_state.right,
                    state,
                    &self.current_mouse.position.into(),
                );
            }
            _ => {}
        }
    }

    fn update_mouse_pos(&mut self, position: PhysicalPosition<f64>) {
        self.current_mouse.position = position;
    }

    fn update_view_matrix(&mut self) {
        let mut view_matrix = self.total_rotation.to_homogeneous();
        let right = view_matrix.fixed_view::<3, 1>(0, 0);
        let up = view_matrix.fixed_view::<3, 1>(0, 1);
        let view_dir = view_matrix.fixed_view::<3, 1>(0, 2);

        let center = self.center;
        let translation = -Vector3::new(right.dot(&center), up.dot(&center), view_dir.dot(&center));

        view_matrix[(0, 3)] = translation.x;
        view_matrix[(1, 3)] = translation.y;
        view_matrix[(2, 3)] = translation.z - self.radius;

        self.camera.set_view_matrix(view_matrix);

        self.screen_world_aabb =
            get_ss_aabb(&self.camera.get_perspective(), self.radius, &self.center);
        self.camera.set_aabb(self.screen_world_aabb.clone());

        self.is_dirty = true;
    }

    fn update_camera(&mut self) {
        if self.mouse_drag_state.right.is_dragging
            && (self.last_mouse_pos_translation.is_none()
                || !self
                    .last_mouse_pos_translation
                    .as_ref()
                    .unwrap()
                    .is_eq_integer(self.current_mouse.position.x, self.current_mouse.position.y))
        {
            let mouse_pos: Position = self.current_mouse.position.into();

            let end_pos = &self.current_mouse.position;
            let start_pos = self.last_mouse_pos_translation.as_ref().unwrap_or(
                self.mouse_drag_state
                    .right
                    .drag_start_pos
                    .as_ref()
                    .unwrap_or(&mouse_pos),
            );

            let width_half = self.window_size.width as f32 / 2.0;
            let height_half = self.window_size.height as f32 / 2.0;

            let projected_z = projected_z(self.radius, &self.camera.get_perspective());

            let end_ndc_point = Point3::new(
                (end_pos.x as f32 - width_half) / width_half,
                (end_pos.y as f32 - height_half) / height_half,
                // -1.0,
                projected_z
            );
            let start_ndc_point = Point3::new(
                (start_pos.x as f32 - width_half) / width_half,
                (start_pos.y as f32 - height_half) / height_half,
                // -1.0,
                projected_z
            );

            let projection = self.camera.get_perspective();

            let start_unproj = unproject_point(projection, &start_ndc_point);
            let end_unproj = unproject_point(projection, &end_ndc_point);

            let diff = (end_unproj - start_unproj);

            let camera_right = self.camera.get_right_vector();
            let camera_up = self.camera.get_up_vector();
            // let camera_view_dir = self.camera.get_view_dir();

            self.center = self.center + camera_right.normalize() * (diff).x - camera_up.normalize() * (diff).y;

            // println!("CENTER: {:#?}", self.center);

            self.update_view_matrix();
            // self.screen_world_aabb = get_ss_aabb(&self.camera.get_perspective(), self.radius, &self.center);
            // self.camera.set_aabb(self.screen_world_aabb.clone());

            println!("view_matrix:\n {:.6}", self.camera.get_view_matrix());

            self.last_mouse_pos_translation = Some(mouse_pos);
        } else if !self.mouse_drag_state.right.is_dragging
            && self.last_mouse_pos_translation.is_some()
        {
            self.last_mouse_pos_translation = None;
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.window_size = size;
        self.perspective_params.aspect = size.width as f32 / size.height as f32;
        self.camera
            .set_perspective((&self.perspective_params).into());
        self.is_dirty = true;
    }

    pub fn event_handler(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::ModifiersChanged(new_modifiers) => {
                self.modifiers = new_modifiers.state();
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.update_radius(delta);
                self.update_camera();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.update_mouse_press(state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.update_mouse_pos(position);
                self.update_camera();
            }
            WindowEvent::Resized(size) => {
                debug!("Resized: {:#?}", size);
                self.resize(size);
            }
            // WindowEvent::Resized { 0: size } => {
            //     self.resize(size);}
            _ => {}
        }
    }
}

fn check_button_drag(
    drag_state: &mut ButtonDragState,
    button_state: ElementState,
    position: &PhysicalPosition<f64>,
) {
    if drag_state.is_dragging {
        if button_state == ElementState::Released && drag_state.prev_state == ButtonState::Pressed {
            drag_state.is_dragging = false;
            drag_state.drag_start_pos = None;
            drag_state.prev_state = ButtonState::Released;
        }
    } else {
        if button_state == ElementState::Pressed && drag_state.prev_state == ButtonState::Released {
            drag_state.is_dragging = true;
            drag_state.drag_start_pos = Some(Position {
                x: position.x,
                y: position.y,
            });
            drag_state.prev_state = ButtonState::Pressed;
        }
    }
}

#[inline]
#[must_use]
pub fn projected_z<T: RealField>(radius: T, perspective: &Perspective3<T>) -> T {
    let perspective = perspective.as_matrix();

    let inverse_denom = -T::one() / radius.clone();
    (perspective[(2, 2)].clone() * radius + perspective[(2, 3)].clone()) * inverse_denom
}

#[inline]
#[must_use]
pub fn unproject_point<T: RealField>(perspective: &Perspective3<T>, p: &Point3<T>) -> Point3<T> {
    let perspective = perspective.as_matrix();
    let inverse_denom = perspective[(2, 3)].clone() / (p[2].clone() + perspective[(2, 2)].clone());

    Point3::new(
        p[0].clone() * inverse_denom.clone() / perspective[(0, 0)].clone(),
        p[1].clone() * inverse_denom.clone() / perspective[(1, 1)].clone(),
        -inverse_denom,
    )
}

#[inline]
#[must_use]
pub fn unproject_point2<T: RealField>(perspective: &Perspective3<T>, p: &Point3<T>) -> Point3<T> {
    let znear = perspective.znear();
    let perspective = perspective.as_matrix();
    let inverse_denom = perspective[(2, 3)].clone() / (p[2].clone() + perspective[(2, 2)].clone());

    Point3::new(
        p[0].clone() * inverse_denom.clone() / perspective[(0, 0)].clone(),
        p[1].clone() * inverse_denom.clone() / perspective[(1, 1)].clone(),
        -inverse_denom,
    ) / znear
}

fn mouse_diff_from_radius(
    mouse_pos: &PhysicalPosition<f64>,
    window_size: &PhysicalSize<u32>,
    perspective: &Perspective3<f32>,
    radius: f32,
    new_radius: f32,
) -> Vector2<f32> {
    let p_z = projected_z(radius, perspective);

    let mouse_ndc = Point3::new(
        mouse_pos.x as f32 / window_size.width as f32 - 0.5,
        -(mouse_pos.y as f32 / window_size.height as f32 - 0.5),
        p_z,
    );

    let p_z_new = projected_z(new_radius, perspective);
    
    let mouse_ndc_new_rad = Point3::new(
        mouse_pos.x as f32 / window_size.width as f32 - 0.5,
        -(mouse_pos.y as f32 / window_size.height as f32 - 0.5),
        p_z_new
    );


    // let start_unproj = unproject_point(&perspective, &mouse_ndc);

    let start = unproject_point(perspective, &mouse_ndc);
    let end = unproject_point(perspective, &mouse_ndc_new_rad);

    let diff = start - end;
    Vector2::new(diff.x, diff.y)
}

/// Get screen space axis aligned bounding box
#[inline]
pub fn get_ss_aabb(perspective: &Perspective3<f32>, radius: f32, center: &Vector3<f32>) -> AaBb {
    let projected_z = projected_z(radius, perspective);

    let min = unproject_point2(perspective, &Point3::new(-0.5, -0.5, 0.0)) * radius + center;
    let max = unproject_point2(perspective, &Point3::new(0.5, 0.5, 0.0)) * radius + center;

    // println!("MIN: {:#?}, MAX: {:#?}", min, max);

    AaBb {
        min: Vector2::new(min.x, min.y),
        max: Vector2::new(max.x, max.y),
    }
}

pub fn chunk_size_from_radius(
    radius: f32,
    chunk_size_step: f32,
    base_chunk_size: f32,
) -> (f32, usize) {
    let chunk_step_idx = (radius.log(10.0).round() - 1.0) as usize;
    (
        chunk_size_from_step_idx(chunk_step_idx as u32),
        (chunk_step_idx as i32 - 1).max(0) as usize,
    )
}
