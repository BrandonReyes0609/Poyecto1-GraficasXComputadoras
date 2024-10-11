use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,     // Ángulo de dirección
    pub fov: f32,   // Campo de visión
}

impl Player {
    pub fn new(pos: Vec2, a: f32, fov: f32) -> Self {
        Self { pos, a, fov }
    }
}
