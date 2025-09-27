use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[cfg(not(target_arch = "wasm32"))]
use std::process;

use eframe::egui::{self, Shape};
use egui::{Color32, Pos2, Stroke, Ui};
use num2words::Num2Words;

const MAX_PIECES: usize = 1000;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    n: usize,
    lines_enabled: bool,
    scale_factor: f32,
    scale_factor_str: String,
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn draw_circle(&self, ui: &Ui) {
        let rect = ui.available_rect_before_wrap();
        let side = rect.width().min(rect.height());
        let square_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(side));
        let painter = ui.painter_at(square_rect);
        let center = square_rect.center();
        let radius = side * 0.40;

        // radial lines with filled "slices"
        for i in 0..self.n {
            // ---- filled polygons ----
            let color = slice_color(i, self.n);
            if self.n == 1 {
                painter.circle_filled(center, radius, color);
            } else {
                let start_angle = (i as f32) * (TAU / (self.n as f32)) + FRAC_PI_2;
                let end_angle = ((i + 1) as f32) * (TAU / (self.n as f32)) + FRAC_PI_2;

                // build polygon: center + arc points
                let mut points = vec![center];

                // number of segments along the arc (for smoother curves)
                let steps = 32.max((radius as usize) / 10);
                for j in 0..=steps {
                    let t = j as f32 / steps as f32;
                    let angle = start_angle + t * (end_angle - start_angle);
                    let x = center.x + radius * angle.cos();
                    let y = center.y + radius * angle.sin();
                    points.push(Pos2::new(x, y));
                }

                // draw filled slice
                painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
            }

            // ---- lines ----
            if self.lines_enabled {
                let angle = (i as f32) * (TAU / (self.n as f32)) + FRAC_PI_2;
                let dx = radius * angle.cos();
                let dy = radius * angle.sin();
                let end = Pos2::new(center.x + dx, center.y + dy);
                if self.n > 1 {
                    painter.line_segment(
                        [center, end],
                        Stroke::new(1.6, Color32::from_rgb(200, 200, 200)),
                    );
                }
            }

            // ---- labels ----
            let label_distance = radius + 20.0; // push labels outside the circle
            let label_angle =
                -(i as f32) * (TAU / (self.n as f32)) + FRAC_PI_2 + (PI / (self.n as f32));
            let lx = center.x + label_distance * label_angle.cos();
            let ly = center.y + label_distance * label_angle.sin();
            let label_pos = Pos2::new(lx, ly);
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                (i + 1).to_string(), // label text
                egui::FontId::proportional(16.0),
                Color32::YELLOW,
            );
        }

        // outline
        if self.lines_enabled {
            painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::WHITE));
        }

        // ordinal text
        let painter = ui.painter_at(rect);
        let x = rect.width() * 0.1;
        let y = 100.0;
        painter.text(
            Pos2::new(x, y),
            egui::Align2::CENTER_CENTER,
            get_ordinal_text(self.n),
            egui::FontId::proportional(16.0),
            Color32::from_rgb(64, 224, 208),
        );

        // center dot
        // painter.circle_filled(center, 2.0, Color32::from_rgb(220, 100, 100));
    }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            n: 1,
            lines_enabled: false,
            scale_factor: 1.5,
            scale_factor_str: "1.5".to_string(),
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(self.scale_factor);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(format!("Choose a number between 1 and {MAX_PIECES}:"));
                    ui.add(egui::Slider::new(&mut self.n, 1..=1000).logarithmic(true));

                    ui.separator();

                    if ui.button("➕").clicked() && self.n < MAX_PIECES {
                        self.n += 1;
                    };
                    if ui.button("➖").clicked() && self.n > 1 {
                        self.n -= 1;
                    };

                    ui.separator();

                    if ui.button("Toggle Lines").clicked() {
                        self.lines_enabled = !self.lines_enabled;
                    };

                    #[cfg(not(target_arch = "wasm32"))]
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Quit").clicked() {
                            process::exit(0);
                        };
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_sized(
                            [40.0, 20.0],
                            egui::TextEdit::singleline(&mut self.scale_factor_str),
                        );
                        if let Ok(scale) = self.scale_factor_str.parse::<f32>() {
                            if (0.5..=4.0).contains(&scale) {
                                self.scale_factor = scale;
                            }
                        }
                        ui.label("Scaling:");
                    });
                });
                ui.add_space(4.0);
            });

            self.draw_circle(ui);
        });
    }
}

fn slice_color(i: usize, n: usize) -> egui::Color32 {
    // -- Scheme 1 - pick a color by cycling hues
    // let hue = (i as f32) / (n as f32);
    // egui::epaint::Hsva::new(hue, 0.9, 0.9, 1.0).into()

    // -- Scheme 2
    let palette = [
        egui::Color32::from_rgb(31, 119, 180),  // blue
        egui::Color32::from_rgb(255, 127, 14),  // orange
        egui::Color32::from_rgb(44, 160, 44),   // green
        egui::Color32::from_rgb(214, 39, 40),   // red
        egui::Color32::from_rgb(148, 103, 189), // purple
        egui::Color32::from_rgb(140, 86, 75),   // brown
    ];
    if n > 1 && i == n - 1 && (i + 1) % palette.len() == 1 {
        // Prevent adjacent slices from being colored the same
        palette[(i + 1) % palette.len()]
    } else {
        palette[i % palette.len()]
    }

    // -- Scheme 3 - shades of blue
    // let intensity = 0.3 + 0.7 * (i as f32 / (n as f32 - 1.0).max(1.0));
    // egui::Color32::from_rgb(
    //     (intensity * 0.2 * 255.0) as u8,
    //     (intensity * 0.6 * 255.0) as u8,
    //     (intensity * 0.9 * 255.0) as u8,
    // )

    // -- Scheme 4 - shades of gray depending on index
    // let intensity = 200 - (i as u8 * 20 % 150);
    // egui::Color32::from_rgb(intensity, intensity, intensity)
}

fn get_ordinal_text(n: usize) -> String {
    match n {
        1 => "Whole".into(),
        2 => "Half".into(),
        _ => capitalize_string(
            &Num2Words::new(n as u32)
                .ordinal()
                .to_words()
                .unwrap_or_else(|_| panic!("Num2Words failed for value {n}")),
        ),
    }
}

fn capitalize_string(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_uppercase().collect::<String>();
            result.push_str(chars.as_str());
            result
        }
    }
}
