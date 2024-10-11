use crate::player::Player;
use winit::event::VirtualKeyCode;
use winit::window::Window;
use std::f32::consts::PI;

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 10.0;

    if window.is_key_down(VirtualKeyCode::Left) {
        // Rota el ángulo de vista hacia la izquierda
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(VirtualKeyCode::Right) {
        // Rota el ángulo de vista hacia la derecha
        player.a += ROTATION_SPEED;
    }
    if window.is_key_down(VirtualKeyCode::Up) {
        // Mueve al jugador hacia adelante en la dirección de la vista
        player.pos.x += player.a.cos() * MOVE_SPEED;
        player.pos.y += player.a.sin() * MOVE_SPEED;
    }
    if window.is_key_down(VirtualKeyCode::Down) {
        // Mueve al jugador hacia atrás en la dirección de la vista
        player.pos.x -= player.a.cos() * MOVE_SPEED;
        player.pos.y -= player.a.sin() * MOVE_SPEED;
    }
}
