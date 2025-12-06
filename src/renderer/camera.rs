use glam::{Mat4, Vec3, Quat};

pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,
    pub fov_y_radians: f32,
    pub z_near: f32,
    pub z_far: f32
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(5.0, 5.0, 4.0),
            rotation: Quat::IDENTITY,
            fov_y_radians: 90.0_f32.to_radians(),
            z_near: 0.1,
            z_far: 100.0
        }
    }
}

#[allow(dead_code)]
impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_fov(mut self, fov_y_radians: f32) -> Self {
        self.fov_y_radians = fov_y_radians;
        self
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn look_at(&mut self, target: Vec3) {
        let dir = (target - self.position).normalize();

        self.rotation = Quat::from_rotation_arc(Vec3::Z, dir)
    }

    pub fn view_matrix(&self) -> Mat4 {
        let target = self.position + self.forward();

        Mat4::look_at_lh(self.position, target, Vec3::Y)
    }

    pub fn projection_matrix(&self, aspect: f32) -> Mat4 {
        Mat4::perspective_lh(self.fov_y_radians, aspect, self.z_near, self.z_far)
    }

    pub fn orthographic_matrix(&self, aspect: f32, scale: f32) -> Mat4 {
        Mat4::orthographic_lh(
            -scale * aspect, scale * aspect,
            -scale, scale,
            self.z_near, self.z_far
        )
    }

    pub fn view_projection(&self, aspect: f32) -> Mat4 {
        self.projection_matrix(aspect) * self.view_matrix()
    }

    pub fn view_orthographic(&self, aspect: f32, scale: f32) -> Mat4 {
        self.orthographic_matrix(aspect, scale) * self.view_matrix()
    }
}