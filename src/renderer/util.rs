use eframe::egui::Pos2;
use glam::{Vec3, Vec4, Mat4};

#[inline]
pub fn project(p: Vec3, vp: &Mat4, center: Pos2, half_size: (f32, f32)) -> Pos2 {
    let clip = *vp * Vec4::new(p.x, p.y, p.z, 1.0);
    let ndc = clip.truncate() / clip.w;
    Pos2 {
        x: center.x + half_size.0 * ndc.x,
        y: center.y - half_size.1 * ndc.y,
    }
}