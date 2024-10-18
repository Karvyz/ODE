#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: glam::Mat4 = glam::Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
]);

pub struct Camera {
    pub eye: glam::Vec3,
    pub target: glam::Vec3,
    up: glam::Vec3,
    aspect: f32,
    fovy: f32, // In degrees, will be converted to radians
    znear: f32,
    zfar: f32,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: glam::Vec3::new(0.0, 1.0, 2.0),
            target: glam::Vec3::new(0.0, 0.0, 0.0),
            up: glam::Vec3::Y, // Equivalent to unit_y()
            aspect,
            fovy: 45.0, // In degrees, needs to be converted to radians later
            znear: 0.1,
            zfar: 100.0,
        }
    }

    pub fn up(&self) -> glam::Vec3 {
        self.up
    }

    fn build_view_projection_matrix(&self) -> glam::Mat4 {
        // 1. Create the view matrix
        let view = glam::Mat4::look_at_rh(self.eye, self.target, self.up);

        // 2. Create the projection matrix (note: converting fovy to radians)
        let proj =
            glam::Mat4::perspective_rh(self.fovy.to_radians(), self.aspect, self.znear, self.zfar);

        // 3. Combine matrices with the OpenGL-to-WGPU matrix
        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().to_cols_array_2d();
    }
}
