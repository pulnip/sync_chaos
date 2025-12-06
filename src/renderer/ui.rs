use eframe::egui;
use glam::{Vec3, Quat};
use crate::simulation::{AizawaParams, Particle, step_rk4};
use super::{Camera, Grid, project};

/// Main application state
pub struct App {
    particles: Vec<Particle>,
    params: AizawaParams,
    camera: Camera,
    grid: Grid
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut camera = Camera::new()
            .with_position(Vec3::new(4.0, 3.0, 4.0));
        camera.look_at(Vec3::ZERO);

        Self {
            particles: Particle::spawn_batch(1000),
            params: AizawaParams::default(),
            camera: camera,
            grid: Grid::new(4.0, 0.5)
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update simulation
        let dt = 0.01;
        for particle in &mut self.particles {
            particle.position = step_rk4(particle.position, &self.params, dt);
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Synchronized Chaos");
            ui.label(format!("Particles: {}", self.particles.len()));

            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::drag(),  // 드래그 감지
            );

            // 드래그로 카메라 회전
            if response.dragged() {
                let delta = response.drag_delta();
                let rotation = Quat::from_rotation_y(-delta.x*0.01);

                self.camera.position = rotation * self.camera.position;
                self.camera.look_at(Vec3::ZERO);
            }

            let rect = response.rect;
            let center = rect.center();
            let aspect = rect.width() / rect.height();

            let vp = self.camera.view_projection(aspect);
            let half_size = (0.5 * rect.width(), 0.5 * rect.height());

            self.grid.draw(&painter, &vp, center, half_size);

            // Draw particles
            for particle in &self.particles {
                let pos = project(particle.position, &vp, center, half_size);

                // Depth-based coloring
                let depth = particle.position.x + particle.position.y + particle.position.z;
                let depth_normalized = ((depth + 3.0) / 6.0).clamp(0.0, 1.0);
                let color = egui::Color32::from_rgb(
                    (100.0 + 155.0 * depth_normalized) as u8,
                    (200.0 * (1.0 - depth_normalized * 0.5)) as u8,
                    255,
                );

                painter.circle_filled(pos, 2.0, color);
            }
        });

        ctx.request_repaint();
    }
}

/// Run the application
pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Synchronized Chaos 🦋"),
        ..Default::default()
    };

    eframe::run_native(
        "Synchronized Chaos",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
