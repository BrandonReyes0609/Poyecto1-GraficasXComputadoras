use nalgebra_glm::Vec2;
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use rodio::{Decoder, OutputStream, Sink, Source}; // Para manejar el audio

mod framebuffer;
mod cast_ray;
mod maze;
mod player;
mod texture;

use cast_ray::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use maze::load_maze;
use player::Player;
use rusttype::{Font, Scale};
use once_cell::sync::Lazy;
use std::sync::Arc;
use std::fs::File;

use texture::Texture;

// Texturas de las paredes
static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/cerca3a.png")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/puerta3.png")));
static CARROT: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/Z3.png")));
static CAT_TEXTURE: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/sprite/gatoM.png")));

fn render_image(framebuffer: &mut Framebuffer, img: &image::DynamicImage) {
    let (img_width, img_height) = img.dimensions();
    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = img.get_pixel(x, y);
            framebuffer.point(x as usize, y as usize, [pixel[0], pixel[1], pixel[2], pixel[3]]);
        }
    }
}

fn main() {
    let width = 1300;
    let height = 900;
    let width_framebuffer = 1300;
    let height_framebuffer = 900;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Graphics - Maze Example")
        .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width as u32, height as u32, surface_texture).unwrap()
    };

    // Cargar la imagen `inicio.gif`
    let img = ImageReader::open("assets/inicio.gif")
        .unwrap()
        .decode()
        .unwrap();
    // Load the victory screen image `Fin.png`
    let fin_img = ImageReader::open("assets/Fin.png")
        .unwrap()
        .decode()
        .unwrap();
    //--------------------
    // Cargar la música de caminar "Walking_Forest.mp3"
    let sound_file = File::open("assets/sounds/Walking_Forest.mp3").unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink_walk = Sink::try_new(&stream_handle).unwrap();
    let sound_decoder = Decoder::new(sound_file).unwrap();
    sink_walk.append(sound_decoder.repeat_infinite());
    sink_walk.pause(); // El sonido de caminar empieza en pausa

    // **Controlar el volumen del sonido de pasos**:
    sink_walk.set_volume(0.9); // Volumen de pasos al 90%

    // Cargar el archivo de sonido de fondo "Jumpin_June.mp3"
    let music_file = File::open("assets/sounds/Jumpin_June.mp3").unwrap();
    let music_sink = Sink::try_new(&stream_handle).unwrap();
    let music_decoder = Decoder::new(music_file).unwrap();
    music_sink.append(music_decoder.repeat_infinite());
    music_sink.play(); // Iniciar la música desde el comienzo

    // **Controlar el volumen de la música de fondo**:
    music_sink.set_volume(0.1); // Volumen de la música de fondo al 10%

    //-------------------

    //let maze = load_maze("maze.txt");
    let mut maze = load_maze("maze.txt");

    let block_size_x = width / maze[0].len();
    let block_size_y = height / maze.len();
    let block_size = block_size_x.min(block_size_y);

    let player_pos = Vec2::new(100.0 + block_size as f32, 100.0 + block_size as f32);
    let player_fov = std::f32::consts::PI / 3.0;
    let mut player = Player::new(player_pos, std::f32::consts::PI / 3.0, player_fov);

    let mut score = 0;
    let mut mode = "2D";
    let mut last_mouse_x = width as f64 / 2.0;
    let mouse_sensitivity = 0.005;
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0;
    let mut fps = 0;
    let fps_update_interval = Duration::from_secs(1);
    let mut show_intro = true;
    let mut game_won = false; // Track game state (won or not)

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(keycode),
                        state,
                        ..
                    },
                    ..
                } => {
                    if show_intro && state == ElementState::Pressed {
                        show_intro = false;
                    } else if !show_intro {
                        let mut new_pos = player.pos;
                        let mut moving = false;

                        // Verificar si se presiona una de las teclas W, A, S o D
                        match (keycode, state) {
                            (VirtualKeyCode::Left | VirtualKeyCode::A, ElementState::Pressed) => {
                                player.a -= std::f32::consts::PI / 10.0;
                                moving = true;
                            }
                            (VirtualKeyCode::Right | VirtualKeyCode::D, ElementState::Pressed) => {
                                player.a += std::f32::consts::PI / 10.0;
                                moving = true;
                            }
                            (VirtualKeyCode::Up | VirtualKeyCode::W, ElementState::Pressed) => {
                                new_pos.x += player.a.cos() * 10.0;
                                new_pos.y += player.a.sin() * 10.0;
                                moving = true;
                            }
                            (VirtualKeyCode::Down | VirtualKeyCode::S, ElementState::Pressed) => {
                                new_pos.x -= player.a.cos() * 10.0;
                                new_pos.y -= player.a.sin() * 10.0;
                                moving = true;
                            }
                            (VirtualKeyCode::M, ElementState::Pressed) => {
                                mode = if mode == "2D" { "3D" } else { "2D" };
                                window.request_redraw();
                            }
                            _ => {}
                        }

                        //if !check_collision(new_pos, &maze, block_size) {
                        //    player.pos = new_pos;
                        //}


                        // Check if the player reached the goal 'g'
                        let collision = check_collision(new_pos, &maze, block_size);
                        if collision == 'g' {
                            game_won = true;
                        } else if collision == ' ' {
                            player.pos = new_pos;
                        } else if collision == 'z' {
                            player.pos = new_pos;
                            score += 1;
                            maze[new_pos.y as usize / block_size][new_pos.x as usize / block_size] = ' ';
                        }

                        if moving {
                            sink_walk.play();
                        } else {
                            sink_walk.pause();
                        }

                        if state == ElementState::Released {
                            sink_walk.pause();
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    if !show_intro {
                        let mouse_x = position.x;
                        let delta_x = mouse_x - last_mouse_x;
                        player.a += (delta_x as f32) * (mouse_sensitivity as f32);
                        last_mouse_x = mouse_x;
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();
                let mut framebuffer = Framebuffer::new(width_framebuffer, height_framebuffer, frame);
                framebuffer.clear([0, 0, 0, 0xFF]);

                if show_intro {
                    render_image(&mut framebuffer, &img);
                } else if game_won {
                    // Show the "Fin.png" screen when the player wins
                    render_image(&mut framebuffer, &fin_img);
                } else {
                    if mode == "2D" {
                        render2d(&mut framebuffer, &player, &maze, width, height, block_size);
                    } else {
                        render3d(&mut framebuffer, &player, block_size, &maze);
                        let distance_to_projection_plane = width_framebuffer as f32 / 2.0 / (player.fov / 2.0).tan();
                        //let cat_pos = Vec2::new(300.0, 200.0);  // Posición del gato en el mundo
                        let cat_pos = Vec2::new(300.26, 218.68);  // Ajusta esta posición según tu laberinto
                        //(290.26, 218.68)
                        //(299.13,267.41)
                        //render_cat_sprite(&mut framebuffer, &player, cat_pos, distance_to_projection_plane, block_size, &maze);                        
                        render_cat_sprite(&mut framebuffer, &player, cat_pos, distance_to_projection_plane, block_size, &maze);

                    }

                    frame_count += 1;
                    if last_frame_time.elapsed() >= fps_update_interval {
                        fps = frame_count;
                        frame_count = 0;
                        last_frame_time = Instant::now();
                    }

                    //render_text(&mut framebuffer, &format!("FPS: {}", fps), 10, 10, 40.0);
                    framebuffer.draw_text(&format!("FPS: {}", fps), 10, 10, 40.0);
                    framebuffer.draw_text(&format!("Puntos: {}", score), width - 150, 10, 30.0);
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}

fn render_text(framebuffer: &mut Framebuffer, text: &str, x: usize, y: usize, scale: f32) {
    let font_data = include_bytes!("../assets/Typold-Book500.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, v| {
                let x = (gx as i32 + bounding_box.min.x) as usize;
                let y = (gy as i32 + bounding_box.min.y) as usize;
                if x < framebuffer.width && y < framebuffer.height {
                    let intensity = (v * 255.0) as u8;
                    framebuffer.point(x, y, [intensity, intensity, intensity, 255]);
                }
            });
        }
    }
}

fn render2d(
    framebuffer: &mut framebuffer::Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    width: usize,
    height: usize,
    block_size: usize,
) {
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' => WALL1.get_pixel_color(0, 0),
                '-' => WALL1.get_pixel_color(0, 0),
                '|' => WALL1.get_pixel_color(0, 0),
                'g' => WALL1.get_pixel_color(0, 0),
                'z' => CARROT.get_pixel_color(0, 0), // Nueva condición para la zanahoria
                ' ' => [0xFF, 0xD7, 0xB3, 0xFF],
                'p' => [0x00, 0xFF, 0x00, 0xFF],
                _ => [0x00, 0x00, 0x00, 0xFF],
            };

            framebuffer.draw_rect(col_idx * block_size, row_idx * block_size, block_size, block_size, color);
        }
    }

    let num_rays = width;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}

fn render3d(
    framebuffer: &mut framebuffer::Framebuffer,
    player: &Player,
    block_size: usize,
    maze: &Vec<Vec<char>>,
) {
    let num_rays = framebuffer.get_width();
    let hw = framebuffer.get_width() as f32 / 2.0;
    let hh = framebuffer.get_height() as f32 / 2.0;
    let distance_to_projection_plane = hw / (player.fov / 2.0).tan();

    framebuffer.draw_sky_and_ground();

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);

        let intersect = cast_ray(framebuffer, maze, &player, a, block_size, false);
        let distance_to_wall = intersect.distance;

        if distance_to_wall < 0.01 {
            continue;
        }

        let stake_height = (distance_to_projection_plane / distance_to_wall) * block_size as f32;
        let stake_top = (hh - (stake_height / 2.0)) as isize;
        let stake_bottom = (hh + (stake_height / 2.0)) as isize;

        let stake_top = stake_top.max(0) as usize;
        let stake_bottom = stake_bottom.min(framebuffer.get_height() as isize) as usize;

        for y in stake_top..stake_bottom {
            let ty = ((y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0) as u32;
            let color = match intersect.impact {
                '+' => WALL1.get_pixel_color(intersect.tx.try_into().unwrap(), ty),
                '-' => WALL1.get_pixel_color(intersect.tx.try_into().unwrap(), ty),
                '|' => WALL1.get_pixel_color(intersect.tx.try_into().unwrap(), ty),
                'g' => WALL2.get_pixel_color(intersect.tx.try_into().unwrap(), ty),
                'z' => CARROT.get_pixel_color(intersect.tx.try_into().unwrap(), ty), // Textura de zanahoria

                _ => [0x00, 0x00, 0x00, 0xFF],
            };

            framebuffer.point(i, y, color);
        }
    }

    let minimap_size = 200;
    render_minimap(framebuffer, player, maze, minimap_size, block_size);
}
fn render_minimap(
    framebuffer: &mut framebuffer::Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    minimap_size: usize,
    block_size: usize,
) {
    let rows = maze.len();
    let cols = maze[0].len();

    let scale_factor_x = minimap_size as f32 / cols as f32;
    let scale_factor_y = minimap_size as f32 / rows as f32;

    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let color = match cell {
                ' ' => [0xFF, 0xD7, 0xB3, 0xFF],
                'p' => [0x00, 0xFF, 0x00, 0xFF],
                'g' => [0xFF, 0x00, 0x00, 0xFF],
                _ => [0x00, 0x00, 0x00, 0xFF],
            };

            let x = (col_idx as f32 * scale_factor_x) as usize;
            let y = (row_idx as f32 * scale_factor_y) as usize;

            framebuffer.draw_rect(
                framebuffer.get_width() - minimap_size + x,
                framebuffer.get_height() - minimap_size + y,
                scale_factor_x.ceil() as usize,
                scale_factor_y.ceil() as usize,
                color,
            );
        }
    }

    let player_x = (player.pos.x / block_size as f32 * scale_factor_x) as usize;
    let player_y = (player.pos.y / block_size as f32 * scale_factor_y) as usize;
    framebuffer.draw_rect(
        framebuffer.get_width() - minimap_size + player_x,
        framebuffer.get_height() - minimap_size + player_y,
        3,
        3,
        [0x00, 0xFF, 0x00, 0xFF],
    );
}

fn check_collision(pos: Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> char {
    let i = pos.x as usize / block_size;
    let j = pos.y as usize / block_size;
    maze[j][i]
}
fn check_cat_collision(cat_pos: Vec2, maze: &Vec<Vec<char>>, block_size: usize) -> bool {
    let i = (cat_pos.x / block_size as f32) as usize;
    let j = (cat_pos.y / block_size as f32) as usize;
    
    // Verifica si la celda actual es una pared
    maze[j][i] == '+' || maze[j][i] == '-' || maze[j][i] == '|'
}

fn render_cat_sprite(
    framebuffer: &mut framebuffer::Framebuffer,
    player: &Player,
    cat_pos: Vec2,  // Posición del gato
    distance_to_projection_plane: f32,
    block_size: usize,
    maze: &Vec<Vec<char>>,  // Referencia al laberinto
) {
    // Verificar si el gato está colisionando con una pared
    if check_cat_collision(cat_pos, maze, block_size) {
        return;  // Si el gato está en una pared, no se renderiza
    }

    let direction = cat_pos - player.pos;
    let distance = direction.magnitude();

    if distance < 0.01 {
        return;
    }

    let angle_to_player = direction.y.atan2(direction.x) - player.a;

    let sprite_height = (distance_to_projection_plane / distance) * block_size as f32;
    let sprite_top = (framebuffer.get_height() as f32 / 2.0 - sprite_height / 2.0).max(0.0) as usize;
    let sprite_bottom = (framebuffer.get_height() as f32 / 2.0 + sprite_height / 2.0).min(framebuffer.get_height() as f32) as usize;

    let sprite_width = sprite_height;  // Suponiendo que el sprite es cuadrado
    let sprite_left = (framebuffer.get_width() as f32 / 2.0 + angle_to_player * distance_to_projection_plane).max(0.0) as usize;
    let sprite_right = (sprite_left as f32 + sprite_width).min(framebuffer.get_width() as f32) as usize;

    for y in sprite_top..sprite_bottom {
        for x in sprite_left..sprite_right {
            let tx = ((x - sprite_left) as f32 / (sprite_right - sprite_left) as f32 * CAT_TEXTURE.width as f32) as u32;
            let ty = ((y - sprite_top) as f32 / (sprite_bottom - sprite_top) as f32 * CAT_TEXTURE.height as f32) as u32;
            let color = CAT_TEXTURE.get_pixel_color(tx, ty);

            framebuffer.point(x, y, color);
        }
    }
}
