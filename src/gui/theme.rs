use eframe::egui::{self, Color32, FontFamily, FontId, Rounding, Stroke, Vec2};


pub const BG_DEEP: Color32 = Color32::from_rgb(18, 10, 38);
pub const BG_MID: Color32 = Color32::from_rgb(42, 22, 78);

pub const PANEL: Color32 = Color32::from_rgba_premultiplied(30, 22, 58, 200);
pub const PANEL_HOVER: Color32 = Color32::from_rgba_premultiplied(48, 36, 86, 220);
pub const SIDEBAR: Color32 = Color32::from_rgba_premultiplied(22, 14, 46, 220);

pub const ACCENT: Color32 = Color32::from_rgb(138, 92, 255);
pub const ACCENT_STRONG: Color32 = Color32::from_rgb(168, 130, 255);
pub const BRAND: Color32 = Color32::from_rgb(138, 92, 255);

pub const OK: Color32 = Color32::from_rgb(72, 220, 140);
pub const WARN: Color32 = Color32::from_rgb(243, 176, 80);
pub const DANGER: Color32 = Color32::from_rgb(240, 96, 110);

pub const TEXT: Color32 = Color32::from_rgb(240, 236, 255);
pub const TEXT_DIM: Color32 = Color32::from_rgb(176, 170, 210);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(130, 122, 170);

pub fn apply_theme(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.override_text_color = Some(TEXT);
    visuals.panel_fill = Color32::TRANSPARENT; 
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = BG_DEEP;
    visuals.faint_bg_color = PANEL;
    visuals.hyperlink_color = ACCENT_STRONG;

    let stroke_subtle = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 20));
    visuals.widgets.noninteractive.bg_fill = Color32::TRANSPARENT;
    visuals.widgets.noninteractive.weak_bg_fill = Color32::TRANSPARENT;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_DIM);
    visuals.widgets.noninteractive.bg_stroke = stroke_subtle;

    visuals.widgets.inactive.bg_fill = PANEL;
    visuals.widgets.inactive.weak_bg_fill = PANEL;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.inactive.bg_stroke = stroke_subtle;
    visuals.widgets.inactive.rounding = Rounding::same(10.0);

    visuals.widgets.hovered.bg_fill = PANEL_HOVER;
    visuals.widgets.hovered.weak_bg_fill = PANEL_HOVER;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 40));
    visuals.widgets.hovered.rounding = Rounding::same(10.0);

    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.weak_bg_fill = ACCENT;
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_STRONG);
    visuals.widgets.active.rounding = Rounding::same(10.0);

    visuals.selection.bg_fill = ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, ACCENT_STRONG);

    visuals.window_rounding = Rounding::same(14.0);
    visuals.menu_rounding = Rounding::same(10.0);
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: Vec2::new(0.0, 6.0),
        blur: 24.0,
        spread: 0.0,
        color: Color32::from_black_alpha(160),
    };

    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(16.0);

    let mut text_styles = style.text_styles.clone();
    text_styles.insert(
        egui::TextStyle::Heading,
        FontId::new(26.0, FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Body,
        FontId::new(14.5, FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Button,
        FontId::new(14.0, FontFamily::Proportional),
    );
    text_styles.insert(
        egui::TextStyle::Small,
        FontId::new(12.0, FontFamily::Proportional),
    );
    style.text_styles = text_styles;

    ctx.set_style(style);
}

pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}

pub fn paint_gradient_background(ui: &egui::Ui, rect: egui::Rect) {
    paint_vertical_gradient(ui, rect, BG_DEEP, BG_MID);
}

pub fn paint_topbar_gradient(ui: &egui::Ui, rect: egui::Rect) {
    const TOPBAR_TOP: Color32 = Color32::from_rgb(12, 6, 28);
    const TOPBAR_BOT: Color32 = Color32::from_rgb(22, 12, 52);
    paint_vertical_gradient(ui, rect, TOPBAR_TOP, TOPBAR_BOT);

    let painter = ui.painter();
    let line_y = rect.bottom() - 0.5;
    painter.line_segment(
        [
            egui::pos2(rect.left(), line_y),
            egui::pos2(rect.right(), line_y),
        ],
        egui::Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(168, 130, 255, 40),
        ),
    );
}

fn paint_vertical_gradient(ui: &egui::Ui, rect: egui::Rect, top: Color32, bot: Color32) {
    use egui::epaint::{Mesh, Vertex};
    use egui::{pos2, Pos2};

    let painter = ui.painter();
    let mut mesh = Mesh::default();
    let v = |p: Pos2, c: Color32| Vertex {
        pos: p,
        uv: egui::epaint::WHITE_UV,
        color: c,
    };
    mesh.vertices.push(v(pos2(rect.left(), rect.top()), top));
    mesh.vertices.push(v(pos2(rect.right(), rect.top()), top));
    mesh.vertices.push(v(pos2(rect.right(), rect.bottom()), bot));
    mesh.vertices.push(v(pos2(rect.left(), rect.bottom()), bot));
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    painter.add(egui::Shape::mesh(mesh));
}
