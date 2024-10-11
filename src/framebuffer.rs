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

    // Método para dibujar un rectángulo en el framebuffer
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

    // Nuevo método para dibujar cielo y suelo
    pub fn draw_sky_and_ground(&mut self) {
        let sky_color = [163,240,255,255];  // Celeste claro (RGBA)
        //let ground_color = [169, 169, 169, 255]; // Gris claro (RGBA)
        let ground_color = [34, 139, 34, 255]; // Verde oscuro (RGBA)

        // Dibujar el cielo en la mitad superior de la pantalla
        self.draw_rect(0, 0, self.width, self.height / 2, sky_color);

        // Dibujar el suelo en la mitad inferior de la pantalla
        self.draw_rect(0, self.height / 2, self.width, self.height / 2, ground_color);
    }
}
