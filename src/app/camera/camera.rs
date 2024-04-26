use nalgebra::{Matrix4, MatrixView3x1, Perspective3, U1, U4};

use crate::utils::AaBb;

// type Matrix4ViewColumn;
pub struct Camera {
    /// Final camera matrix
    view_matrix: Matrix4<f32>,
    aabb: AaBb,
    /// Perspective matrix,
    perspective: Perspective3<f32>, // perspective: Matrix4<f32>,
}

impl Camera {
    pub fn new(view_matrix: Matrix4<f32>, aabb: AaBb, perspective: Perspective3<f32>) -> Self {
        Self {
            view_matrix,
            aabb,
            perspective,
        }
    }

    pub fn build_view_proj(&self) -> Matrix4<f32> {
        self.perspective.as_matrix() * self.view_matrix
    }

    // pub fn get_eye(&self) -> &Vector3<f32> {
    //     &self.eye
    // }
    //
    // pub fn get_center(&self) -> &Vector3<f32> {
    //     &self.center
    // }
    //
    // pub fn get_up(&self) -> &Vector3<f32> {
    //     &self.up
    // }

    pub fn get_right_vector(&self) -> MatrixView3x1<f32, U1, U4> {
        self.view_matrix.fixed_view::<3, 1>(0, 0)
    }

    pub fn get_up_vector(&self) -> MatrixView3x1<f32, U1, U4> {
        self.view_matrix.fixed_view::<3, 1>(0, 1)
    }

    pub fn get_view_dir(&self) -> MatrixView3x1<f32, U1, U4> {
        self.view_matrix.fixed_view::<3, 1>(0, 2)
    }

    pub fn get_view_matrix(&self) -> &Matrix4<f32> {
        &self.view_matrix
    }

    pub fn get_perspective(&self) -> &Perspective3<f32> {
        &self.perspective
    }

    // pub fn set_eye(&mut self, eye: Vector3<f32>) {
    //     self.eye = eye;
    // }
    //
    // pub fn set_center(&mut self, center: Vector3<f32>) {
    //     self.center = center;
    // }
    //
    // pub fn set_up(&mut self, up: Vector3<f32>) {
    //     self.up = up;
    // }

    pub fn set_view_matrix(&mut self, view_matrix: Matrix4<f32>) {
        self.view_matrix = view_matrix;
    }

    pub fn set_perspective(&mut self, perspective: Perspective3<f32>) {
        self.perspective = perspective;
    }

    pub fn aabb(&self) -> &AaBb {
        &self.aabb
    }

    pub fn set_aabb(&mut self, aabb: AaBb) {
        self.aabb = aabb;
    }
}
