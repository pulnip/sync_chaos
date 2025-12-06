use eframe::egui::{Color32, Painter, Pos2, Stroke};
use glam::{Mat4, Vec3};

use super::project;

pub struct Grid {
    pub size: f32,
    pub step: f32
}

enum Axis{
    X,
    Y,
    Z
}

impl Grid {
    pub fn new(size: f32, step: f32) -> Self {
        Self { size, step }
    }

    fn draw_plane(&self, painter: &Painter, vp: &Mat4, center: Pos2, half_size: (f32, f32), fixed_axis: Axis, grid_color: Color32, border_color: Color32) {
        let make_point = |a: f32, b: f32| -> Vec3 {
            match fixed_axis {
                Axis::X => Vec3::new(0.0, a, b),
                Axis::Y => Vec3::new(a, 0.0, b),
                _ => Vec3::new(a, b, 0.0)
            }
        };
        let stroke = Stroke::new(1.0, grid_color);

        let mut v = self.step;
        while v < self.size {
            let p1 = project(make_point(0.0, v), vp, center, half_size);
            let p2 = project(make_point(self.size, v), vp, center, half_size);
            painter.line_segment([p1, p2], stroke);

            let p1 = project(make_point(v, 0.0), vp, center, half_size);
            let p2 = project(make_point(v, self.size), vp, center, half_size);
            painter.line_segment([p1, p2], stroke);

            v += self.step;
        }

        // grid border
        let p2 = project(make_point(self.size, 0.0), vp, center, half_size);
        let p3 = project(make_point(self.size, self.size), vp, center, half_size);
        let p4 = project(make_point(0.0, self.size), vp, center, half_size);
        let stroke = Stroke::new(2.0, border_color);
        painter.line_segment([p2, p3], stroke);
        painter.line_segment([p3, p4], stroke);
    }

    fn draw_axes(&self, painter: &Painter, vp: &Mat4, center: Pos2,
  half_size: (f32, f32)) {
        let origin = project(Vec3::ZERO, vp, center, half_size);
        let x_end = project(Vec3::new(self.size, 0.0, 0.0), vp, center,half_size);
        let y_end = project(Vec3::new(0.0, self.size, 0.0), vp, center,half_size);
        let z_end = project(Vec3::new(0.0, 0.0, self.size), vp, center, half_size);

        painter.line_segment([origin, x_end], Stroke::new(2.0, Color32::RED));
        painter.line_segment([origin, y_end], Stroke::new(2.0, Color32::GREEN));
        painter.line_segment([origin, z_end], Stroke::new(2.0, Color32::BLUE));
    }

    pub fn draw(&self, painter: &Painter, vp: &Mat4, center: Pos2, half_size: (f32, f32)) {
        let grid_color = Color32::from_rgba_unmultiplied(128, 128, 128, 40);

        self.draw_plane(painter, vp, center, half_size, Axis::X, grid_color, Color32::RED);
        self.draw_plane(painter, vp, center, half_size, Axis::Y, grid_color, Color32::GREEN);
        self.draw_plane(painter, vp, center, half_size, Axis::Z, grid_color, Color32::BLUE);
        self.draw_axes(painter, vp, center, half_size);
    }
}