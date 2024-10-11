use rusttype::{Font, Scale};

pub struct Framebuffer<'a> {
    pub width: usize,
    pub height: usize,
    pub buffer: &'a mut [u8],
}

impl<'a> Framebuffer<'a> {
    pub fn new(width: usize, height: usize, buffer: &'a mut [u8]) -> Self {
        Self { width, height, buffer }
    }

    pub fn point(&mut self, x: usize, y: usize, color: [u8; 4]) {
        let index = (y * self.width + x) * 4;
        if index + 3 < self.buffer.len() {
            self.buffer[index..index + 4].copy_from_slice(&color);
        }
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.point(x, y, color);
            }
        }
    }

    pub fn draw_rect(&mut self, x: usize, y: usize, width: usize, height: usize, color: [u8; 4]) {
        for j in 0..height {
            for i in 0..width {
                self.point(x + i, y + j, color);
            }
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn draw_sky_and_ground(&mut self) {
        let sky_color = [163, 240, 255, 255];  // Celeste claro (RGBA)
        let ground_color = [34, 139, 34, 255]; // Verde oscuro (RGBA)

        self.draw_rect(0, 0, self.width, self.height / 2, sky_color);
        self.draw_rect(0, self.height / 2, self.width, self.height / 2, ground_color);
    }

    // Método para renderizar texto en el framebuffer
    pub fn draw_text(&mut self, text: &str, x: usize, y: usize, scale: f32) {
        let font_data = include_bytes!("../assets/Typold-Book500.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error cargando la fuente");
        
        let scale = Scale::uniform(scale);
        let v_metrics = font.v_metrics(scale);
        let offset = rusttype::point(x as f32, y as f32 + v_metrics.ascent);

        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, v| {
                    let x = (gx as i32 + bounding_box.min.x) as usize;
                    let y = (gy as i32 + bounding_box.min.y) as usize;
                    if x < self.width && y < self.height {
                        let intensity = (v * 255.0) as u8;
                        self.point(x, y, [intensity, intensity, intensity, 255]);
                    }
                });
            }
        }
    }
}
