use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[cfg(not(target_arch = "wasm32"))]
use std::process;

use eframe::egui::{self, Shape};
// use egui::Vec2;
use egui::text::LayoutJob;
use egui::{Color32, Pos2, Stroke, Ui};
use num2words::Num2Words;
use serde::{Deserialize, Serialize};

const MAX_PIECES: usize = 1000;

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    n: usize,
    lines_enabled: bool,
    scale_factor: f32,
    // central_panel_res: Vec2,
    color_scheme: ColorScheme,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ColorScheme {
    Rainbow,
    Tableau,
    Blues,
    Grays,
}

impl ColorScheme {
    fn get_slice_color(&self, i: usize, n: usize) -> Color32 {
        slice_color(i, n, self)
    }
}

fn slice_color(i: usize, n: usize, color_scheme: &ColorScheme) -> Color32 {
    match color_scheme {
        // -- Scheme 1 - pick a color by cycling hues
        ColorScheme::Rainbow => {
            let hue = (i as f32) / (n as f32);
            egui::epaint::Hsva::new(hue, 0.9, 0.9, 1.0).into()
        }
        // -- Scheme 2 - solid colors
        ColorScheme::Tableau => {
            let palette = [
                // Color32::from_rgb(31, 119, 180),  // blue
                // Color32::from_rgb(255, 127, 14),  // orange
                // Color32::from_rgb(44, 160, 44),   // green
                // Color32::from_rgb(214, 39, 40),   // red
                // Color32::from_rgb(148, 103, 189), // purple
                // Color32::from_rgb(140, 86, 75),   // brown
                Color32::from_hex("#5778a4").unwrap_or_default(), // blue
                Color32::from_hex("#e49444").unwrap_or_default(), // orange
                Color32::from_hex("#d1615d").unwrap_or_default(), // red
                Color32::from_hex("#85b6b2").unwrap_or_default(), // teal
                Color32::from_hex("#6a9f58").unwrap_or_default(), // green
                Color32::from_hex("#e7ca60").unwrap_or_default(), // yellow
                Color32::from_hex("#a87c9f").unwrap_or_default(), // purple
                Color32::from_hex("#f1a2a9").unwrap_or_default(), // pink
                Color32::from_hex("#967662").unwrap_or_default(), // brown
                Color32::from_hex("#b8b0ac").unwrap_or_default(), // grey
            ];
            if n > 1 && i == n - 1 && (i + 1) % palette.len() == 1 {
                // Prevent adjacent slices from being colored the same
                palette[(i + 1) % palette.len()]
            } else {
                palette[i % palette.len()]
            }
        }
        // -- Scheme 3 - shades of blue
        ColorScheme::Blues => {
            let intensity = 0.3 + 0.7 * (i as f32 / (n as f32 - 1.0).max(1.0));
            Color32::from_rgb(
                (intensity * 0.2 * 255.0) as u8,
                (intensity * 0.6 * 255.0) as u8,
                (intensity * 0.9 * 255.0) as u8,
            )
        }
        // -- Scheme 4 - shades of gray depending on index
        ColorScheme::Grays => {
            let intensity = 200 - (i as u8 * 20 % 150);
            Color32::from_rgb(intensity, intensity, intensity)
        }
    }
}

fn color_text(text: &str, color_scheme: &ColorScheme) -> LayoutJob {
    let mut job = LayoutJob::default();
    for (i, ch) in text.chars().enumerate() {
        let color = slice_color(i, text.len(), color_scheme);
        job.append(
            &ch.to_string(),
            0.0,
            egui::TextFormat {
                color,
                ..Default::default()
            },
        );
    }

    job
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

    fn header_add_number_selector(&mut self, ui: &mut Ui) {
        ui.label("How many slices?");
        ui.add(egui::Slider::new(&mut self.n, 1..=MAX_PIECES).logarithmic(true));
    }

    fn header_add_misc_buttons(&mut self, ui: &mut Ui) {
        if ui.button("➖").clicked() && self.n > 1 {
            self.n -= 1;
        };
        if ui.button("➕").clicked() && self.n < MAX_PIECES {
            self.n += 1;
        };
        if ui.button("Toggle Lines").clicked() {
            self.lines_enabled = !self.lines_enabled;
        };
        #[cfg(not(target_arch = "wasm32"))]
        if ui.button("Quit").clicked() {
            process::exit(0);
        };
    }
    fn header_add_color_selector(&mut self, ui: &mut Ui) {
        ui.label("Colors:");
        ui.selectable_value(
            &mut self.color_scheme,
            ColorScheme::Rainbow,
            color_text("Pinwheel", &ColorScheme::Rainbow),
        );
        ui.selectable_value(
            &mut self.color_scheme,
            ColorScheme::Tableau,
            color_text("Tableau", &ColorScheme::Tableau),
        );
        ui.selectable_value(
            &mut self.color_scheme,
            ColorScheme::Blues,
            color_text("Blues", &ColorScheme::Blues),
        );
        ui.selectable_value(
            &mut self.color_scheme,
            ColorScheme::Grays,
            color_text("Grays", &ColorScheme::Grays),
        );
    }

    fn add_header(&mut self, ui: &mut Ui, width: f32, height: f32) {
        ui.group(|ui| {
            if width >= height {
                ui.horizontal_wrapped(|ui| {
                    self.header_add_number_selector(ui);
                    ui.separator();
                    self.header_add_misc_buttons(ui);
                    ui.separator();
                    self.header_add_color_selector(ui);
                });
            } else {
                ui.vertical(|ui| {
                    ui.horizontal_wrapped(|ui| {
                        self.header_add_number_selector(ui);
                    });
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        self.header_add_misc_buttons(ui);
                    });
                    ui.separator();
                    ui.horizontal_wrapped(|ui| {
                        self.header_add_color_selector(ui);
                    });
                });
            }
        });
    }

    fn add_circle(&mut self, ui: &Ui) {
        let rect = ui.available_rect_before_wrap();
        let side = rect.width().min(rect.height()) * 1.05;
        let square_rect = egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(side));
        let painter = ui.painter_at(square_rect);
        let center = square_rect.center();
        let radius = side * 0.42;

        // radial lines with filled "slices"
        for i in 0..self.n {
            // ---- filled polygons ----
            let color = self.color_scheme.get_slice_color(i, self.n);
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
            let label_distance = radius + 15.0; // push labels outside the circle
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
        let pos = rect.lerp_inside(egui::Vec2::new(0.09, 0.09));
        painter.text(
            pos,
            egui::Align2::LEFT_CENTER,
            get_ordinal_text(self.n),
            egui::FontId::proportional(16.0),
            Color32::from_rgb(64, 224, 208),
        );

        // dimensions
        // let painter = ui.painter_at(rect);
        // let width = rect.width().floor();
        // let height = rect.height().floor();
        // painter.text(
        //     Pos2::new(width * 0.08, height * 0.97),
        //     egui::Align2::CENTER_CENTER,
        //     format!("Debug info\nSize: {width} x {height}"),
        //     egui::FontId::proportional(14.0),
        //     Color32::from_rgb(50, 50, 50),
        // );
        // self.central_panel_res = Vec2::new(rect.width(), rect.height());

        // center dot
        // painter.circle_filled(center, 2.0, Color32::from_rgb(220, 100, 100));
    }

    // fn add_footer(&self, ui: &mut Ui) {
    //     ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
    //         // Show window resolution so I can figure out good default scaling parameters
    //         let available = self.central_panel_res;
    //         let scale = ui.pixels_per_point();
    //         ui.label(format!(
    //             "Size: {} x {}",
    //             (available.x * scale) as u32,
    //             (available.y * scale) as u32
    //         ));
    //     });
    // }
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            n: 1,
            lines_enabled: false,
            scale_factor: 2.0,
            // central_panel_res: Vec2::new(0.0, 0.0),
            color_scheme: ColorScheme::Tableau,
        }
    }
}

impl eframe::App for TemplateApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let screen_rect = ctx.screen_rect();
        let width = screen_rect.width();
        let height = screen_rect.height();
        self.scale_factor = if height > width { 3.0 } else { 2.0 };
        ctx.set_pixels_per_point(self.scale_factor);

        egui::TopBottomPanel::top("top_panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                self.add_header(ui, width, height);
            });

        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     self.add_footer(ui);
        // });

        // Add circle panel last since its dimensions are calculated based on remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            self.add_circle(ui);
        });
    }
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
