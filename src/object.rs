use glam::Vec3;

pub struct Object {
    pub position: Vec3,
}

impl Object {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}
