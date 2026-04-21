use eframe::egui::{self, Color32, Response, RichText, Sense, Stroke, Ui, Vec2};
use super::theme;

pub fn card(ui: &mut Ui, title: &str, body: impl FnOnce(&mut Ui)) {
    egui::Frame::none()
        .fill(theme::PANEL)
        .rounding(16.0)
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 24),
        ))
        .inner_margin(egui::Margin::same(20.0))
        .shadow(egui::epaint::Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 20.0,
            spread: 0.0,
            color: Color32::from_black_alpha(60),
        })
        .show(ui, |ui| {
            if !title.is_empty() {
                ui.label(RichText::new(title).strong().size(16.0));
                ui.add_space(12.0);
            }
            body(ui);
        });
}

pub fn card_with_icon(ui: &mut Ui, icon: &str, title: &str, body: impl FnOnce(&mut Ui)) {
    egui::Frame::none()
        .fill(theme::PANEL)
        .rounding(16.0)
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 24),
        ))
        .inner_margin(egui::Margin::same(20.0))
        .shadow(egui::epaint::Shadow {
            offset: Vec2::new(0.0, 4.0),
            blur: 20.0,
            spread: 0.0,
            color: Color32::from_black_alpha(60),
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).color(theme::ACCENT_STRONG).size(18.0));
                ui.label(RichText::new(title).strong().size(16.0));
            });
            ui.add_space(12.0);
            body(ui);
        });
}

pub fn status_dot(ui: &mut Ui, color: Color32) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(10.0, 10.0), Sense::hover());
    ui.painter().circle_filled(rect.center(), 5.0, color);
}

pub fn primary_button(ui: &mut Ui, label: &str) -> Response {
    let button = egui::Button::new(RichText::new(label).color(Color32::WHITE).strong().size(14.0))
        .fill(theme::ACCENT)
        .stroke(Stroke::new(1.0, theme::ACCENT_STRONG))
        .rounding(10.0)
        .min_size(Vec2::new(130.0, 38.0));
    ui.add(button)
}

pub fn secondary_button(ui: &mut Ui, label: &str) -> Response {
    let button = egui::Button::new(RichText::new(label).color(theme::TEXT).size(14.0))
        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 10))
        .stroke(Stroke::new(1.0, theme::ACCENT))
        .rounding(10.0)
        .min_size(Vec2::new(130.0, 38.0));
    ui.add(button)
}

pub fn danger_button(ui: &mut Ui, label: &str) -> Response {
    let fill = Color32::from_rgba_unmultiplied(240, 96, 110, 40);
    let button = egui::Button::new(RichText::new(label).color(theme::DANGER).strong().size(14.0))
        .fill(fill)
        .stroke(Stroke::new(1.0, theme::DANGER))
        .rounding(10.0)
        .min_size(Vec2::new(130.0, 38.0));
    ui.add(button)
}

pub fn field_label(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).color(theme::TEXT_DIM).size(13.0));
}

pub fn toggle(ui: &mut Ui, on: &mut bool) -> bool {
    let desired_size = Vec2::new(42.0, 22.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
    }
    let visuals = ui.ctx().animate_bool(response.id, *on);
    let off_bg = Color32::from_rgba_unmultiplied(70, 64, 110, 220);
    let on_bg = theme::ACCENT;
    let bg = lerp_color(off_bg, on_bg, visuals);
    let painter = ui.painter();
    painter.rect(
        rect,
        rect.height() / 2.0,
        bg,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 30)),
    );
    
    let knob_r = (rect.height() - 6.0) / 2.0;
    let knob_x = rect.left() + 3.0 + knob_r + visuals * (rect.width() - rect.height());
    let knob_center = egui::pos2(knob_x, rect.center().y);
    painter.circle_filled(knob_center, knob_r, Color32::WHITE);
    response.changed()
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    let lerp = |x: u8, y: u8| -> u8 {
        (x as f32 + (y as f32 - x as f32) * t).clamp(0.0, 255.0) as u8
    };
    Color32::from_rgba_unmultiplied(
        lerp(a.r(), b.r()),
        lerp(a.g(), b.g()),
        lerp(a.b(), b.b()),
        lerp(a.a(), b.a()),
    )
}
