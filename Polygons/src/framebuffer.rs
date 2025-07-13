use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, background_color);
        Self {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE,
        }
    }    
    

    pub fn clear(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.background_color);
            }
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            Image::draw_pixel(&mut self.color_buffer, x as i32, y as i32, self.current_color);
        }
    }


    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        Image::export_image(&self.color_buffer, file_path);
    }

    pub fn draw_polygon(&mut self, vertices: &[Vector2]) {
        use crate::line::line;

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = vertices[(i + 1) % vertices.len()];
            line(self, start, end);
        }
    }

    pub fn fill_polygon(&mut self, vertices: &[Vector2]) {
        let mut y_min = vertices[0].y;
        let mut y_max = vertices[0].y;

        for v in vertices {
            if v.y < y_min { y_min = v.y; }
            if v.y > y_max { y_max = v.y; }
        }

        let y_min = y_min.ceil() as i32;
        let y_max = y_max.floor() as i32;

        for y in y_min..=y_max {
            let mut intersections = Vec::new();

            for i in 0..vertices.len() {
                let v1 = vertices[i];
                let v2 = vertices[(i + 1) % vertices.len()];

                if (v1.y <= y as f32 && v2.y > y as f32) || (v2.y <= y as f32 && v1.y > y as f32) {
                    let x = v1.x + (y as f32 - v1.y) * (v2.x - v1.x) / (v2.y - v1.y);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for pair in intersections.chunks(2) {
                if pair.len() == 2 {
                    let x_start = pair[0].ceil() as i32;
                    let x_end = pair[1].floor() as i32;

                    for x in x_start..=x_end {
                        self.set_pixel(x as u32, y as u32);
                    }
                }
            }
        }
    }
}
