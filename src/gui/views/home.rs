use eframe::egui::{self, Color32, Layout, RichText, Stroke, Ui, Vec2};

use crate::config::countries::{self, COUNTRIES};
use crate::config::i18n::{self, Key, Language};
use crate::core::connection::ConnectionStatus;
use crate::core::state::{SharedState, TrafficStats};
use crate::gui::theme;
use crate::gui::widgets::{card_with_icon, field_label, toggle};
use crate::utils::format::{format_bps, format_bytes};

pub struct HomeActions {
    pub connect: bool,
    pub disconnect: bool,
    pub refresh_ip: bool,
    pub new_identity: bool,
    pub settings_changed: bool,
    pub exit: bool,
}

impl HomeActions {
    fn new() -> Self {
        Self {
            connect: false,
            disconnect: false,
            refresh_ip: false,
            new_identity: false,
            settings_changed: false,
            exit: false,
        }
    }
}

pub fn show(ui: &mut Ui, state: &SharedState, lang: Language) -> HomeActions {
    let mut actions = HomeActions::new();
    let status = state.current_status();
    let traffic = state.traffic.read().clone();
    let current_ip = state.current_ip.read().clone();

    draw_hero(ui, &status, current_ip.as_deref(), lang, &mut actions);

    ui.add_space(16.0);

    ui.horizontal_top(|ui| {
        let total_w = ui.available_width();
        let left_w = (total_w * 0.40).max(320.0);
        let right_w = total_w - left_w - 16.0;

        ui.allocate_ui_with_layout(
            [left_w, 0.0].into(),
            Layout::top_down(egui::Align::Min),
            |ui| {
                draw_circuit_card(ui, state, lang, &mut actions);
            },
        );

        ui.add_space(16.0);

        ui.allocate_ui_with_layout(
            [right_w, 0.0].into(),
            Layout::top_down(egui::Align::Min),
            |ui| {
                draw_bandwidth_card(ui, state, &traffic, lang);
            },
        );
    });

    ui.add_space(16.0);
    ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
        let etiqueta = format!(
            "{}  {}",
            egui_phosphor::regular::SIGN_OUT,
            i18n::t(lang, Key::Exit)
        );
        let boton = egui::Button::new(
            RichText::new(etiqueta)
                .size(13.0)
                .color(Color32::from_rgb(240, 96, 110))
                .strong(),
        )
        .fill(Color32::from_rgba_unmultiplied(240, 96, 110, 36))
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(240, 96, 110, 120),
        ))
        .rounding(8.0)
        .min_size([120.0, 34.0].into());
        if ui.add(boton).clicked() {
            actions.exit = true;
        }
    });

    actions
}

fn draw_hero(
    ui: &mut Ui,
    status: &ConnectionStatus,
    current_ip: Option<&str>,
    lang: Language,
    actions: &mut HomeActions,
) {
    let (accent, label_key) = match status {
        ConnectionStatus::Disconnected => (theme::TEXT_DIM, Key::StatusDisconnected),
        ConnectionStatus::Connecting { .. } => (theme::WARN, Key::StatusConnecting),
        ConnectionStatus::Connected => (theme::OK, Key::StatusConnected),
        ConnectionStatus::Failed(_) => (theme::DANGER, Key::StatusFailed),
        ConnectionStatus::Disconnecting => (theme::WARN, Key::StatusDisconnecting),
    };

    egui::Frame::none()
        .fill(Color32::from_rgba_unmultiplied(30, 22, 58, 200))
        .rounding(18.0)
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 28),
        ))
        .inner_margin(egui::Margin::same(28.0))
        .shadow(egui::epaint::Shadow {
            offset: Vec2::new(0.0, 6.0),
            blur: 28.0,
            spread: 0.0,
            color: Color32::from_black_alpha(70),
        })
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                // The orb is the primary action target. Clicking it
                // toggles Connect/Disconnect based on current state.
                let orb_response = draw_status_orb(ui, status, accent);
                if orb_response.clicked() {
                    match status {
                        ConnectionStatus::Disconnected | ConnectionStatus::Failed(_) => {
                            actions.connect = true;
                        }
                        ConnectionStatus::Connected => {
                            actions.disconnect = true;
                        }
                        // Transitional states swallow the click so the
                        // user can't fire two requests back-to-back.
                        _ => {}
                    }
                }
                let tooltip = match status {
                    ConnectionStatus::Disconnected | ConnectionStatus::Failed(_) => {
                        i18n::t(lang, Key::Connect)
                    }
                    ConnectionStatus::Connected => i18n::t(lang, Key::Disconnect),
                    _ => i18n::t(lang, Key::Working),
                };
                orb_response.on_hover_text(tooltip);

                ui.add_space(14.0);

                ui.label(
                    RichText::new(i18n::t(lang, label_key))
                        .size(24.0)
                        .strong()
                        .color(theme::TEXT),
                );
                ui.add_space(4.0);

                // Amber IP — stands out against the violet card.
                // Dimmed during transitions so a stale value isn't read
                // as the current one.
                let ip_color = match status {
                    ConnectionStatus::Connecting { .. }
                    | ConnectionStatus::Disconnecting => theme::TEXT_DIM,
                    _ => Color32::from_rgb(255, 214, 77),
                };
                ui.label(
                    RichText::new(current_ip.unwrap_or("—"))
                        .size(20.0)
                        .color(ip_color)
                        .strong()
                        .monospace(),
                );
                ui.add_space(20.0);

                // Secondary actions only — the orb handles connect/disconnect.
                draw_hero_actions(ui, lang, actions);
            });
        });
}

fn draw_status_orb(
    ui: &mut Ui,
    status: &ConnectionStatus,
    accent: Color32,
) -> egui::Response {
    let size = 140.0;
    let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
    let painter = ui.painter();
    let center = rect.center();
    let outer_r = size / 2.0 - 6.0;
    let track_r = outer_r - 10.0;
    let inner_r = track_r - 10.0;

    let interactive = matches!(
        status,
        ConnectionStatus::Disconnected
            | ConnectionStatus::Failed(_)
            | ConnectionStatus::Connected
    );
    if response.hovered() && interactive {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let hover_boost = if response.hovered() && interactive { 0.4 } else { 0.0 };
    let glow_intensity = match status {
        ConnectionStatus::Connected => {
            let t = ui.ctx().input(|i| i.time) as f32;
            0.6 + 0.4 * (t * 2.2).sin()
        }
        ConnectionStatus::Connecting { .. } => 0.45,
        _ => 0.25,
    } + hover_boost;
    let glow_alpha = (glow_intensity * 70.0).clamp(0.0, 255.0) as u8;
    painter.circle_filled(
        center,
        outer_r + 6.0,
        Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), glow_alpha),
    );

    painter.circle(
        center,
        track_r,
        Color32::TRANSPARENT,
        Stroke::new(4.0, Color32::from_rgba_unmultiplied(255, 255, 255, 25)),
    );

    let fraction: f32 = match status {
        ConnectionStatus::Connected => 1.0,
        ConnectionStatus::Connecting { progress, .. } => (*progress as f32 / 100.0).clamp(0.0, 1.0),
        ConnectionStatus::Failed(_) => 1.0,
        _ => 0.0,
    };
    if fraction > 0.0 {
        draw_arc(&painter, center, track_r, fraction, accent, 4.0);
    }

    let inner_color = match status {
        ConnectionStatus::Connected => accent,
        ConnectionStatus::Failed(_) => accent,
        _ => Color32::from_rgba_unmultiplied(255, 255, 255, 180),
    };
    let glyph = match status {
        ConnectionStatus::Connected => egui_phosphor::regular::SHIELD_CHECK,
        ConnectionStatus::Failed(_) => egui_phosphor::regular::WARNING,
        ConnectionStatus::Connecting { .. } | ConnectionStatus::Disconnecting => {
            egui_phosphor::regular::SPINNER
        }
        ConnectionStatus::Disconnected => egui_phosphor::regular::POWER,
    };
    painter.text(
        center,
        egui::Align2::CENTER_CENTER,
        glyph,
        egui::FontId::proportional(inner_r * 1.1),
        inner_color,
    );

    if matches!(
        status,
        ConnectionStatus::Connected | ConnectionStatus::Connecting { .. }
    ) {
        ui.ctx().request_repaint_after(std::time::Duration::from_millis(16));
    }

    response
}

fn draw_arc(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    fraction: f32,
    color: Color32,
    thickness: f32,
) {
    const SEGMENTS: usize = 96;
    let total_steps = ((SEGMENTS as f32) * fraction).round() as usize;
    if total_steps < 1 {
        return;
    }
    let stroke = Stroke::new(thickness, color);
    let two_pi = std::f32::consts::TAU;
    let start = -std::f32::consts::FRAC_PI_2;
    let mut prev = egui::pos2(
        center.x + radius * start.cos(),
        center.y + radius * start.sin(),
    );
    for i in 1..=total_steps {
        let a = start + two_pi * (i as f32 / SEGMENTS as f32);
        let next = egui::pos2(center.x + radius * a.cos(), center.y + radius * a.sin());
        painter.line_segment([prev, next], stroke);
        prev = next;
    }
}

fn draw_hero_actions(
    ui: &mut Ui,
    lang: Language,
    actions: &mut HomeActions,
) {
    ui.horizontal(|ui| {
        let row_width = 280.0;
        ui.add_space((ui.available_width() - row_width).max(0.0) / 2.0);

        if pill_button(ui, egui_phosphor::regular::USER_SWITCH, i18n::t(lang, Key::NewIdentity))
            .clicked()
        {
            actions.new_identity = true;
        }
        if pill_button(ui, egui_phosphor::regular::ARROWS_CLOCKWISE, i18n::t(lang, Key::RefreshIp))
            .clicked()
        {
            actions.refresh_ip = true;
        }
    });
}

fn pill_button(ui: &mut Ui, icon: &str, label: &str) -> egui::Response {
    ui.add(
        egui::Button::new(
            RichText::new(format!("{}  {}", icon, label))
                .size(13.0)
                .color(theme::TEXT),
        )
        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 18))
        .stroke(Stroke::new(
            1.0,
            Color32::from_rgba_unmultiplied(255, 255, 255, 40),
        ))
        .rounding(8.0)
        .min_size([130.0, 36.0].into()),
    )
}

fn draw_circuit_card(
    ui: &mut Ui,
    state: &SharedState,
    lang: Language,
    actions: &mut HomeActions,
) {
    card_with_icon(
        ui,
        egui_phosphor::regular::CIRCUITRY,
        i18n::t(lang, Key::Routing),
        |ui| {
            let mut settings = state.settings.write();

            ui.horizontal(|ui| {
                ui.label(RichText::new(i18n::t(lang, Key::SmartConnect)).strong());
                ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
                    if toggle(ui, &mut settings.smart_connect) {
                        actions.settings_changed = true;
                    }
                });
            });
            ui.add_space(12.0);

            // Exit country combo — Mode combo removed since v1 only
            // ships Proxy mode. No point showing a dropdown with one
            // option.
            field_label(ui, i18n::t(lang, Key::ExitLocation));
            let current_label = settings
                .exit_country
                .as_ref()
                .and_then(|c| countries::find_by_code(c))
                .map(|c| c.display())
                .unwrap_or_else(|| i18n::t(lang, Key::Automatic).to_string());

            let mut country_changed = false;
            egui::ComboBox::from_id_source("exit-combo")
                .width(ui.available_width())
                .selected_text(current_label)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(
                            settings.exit_country.is_none(),
                            i18n::t(lang, Key::Automatic),
                        )
                        .clicked()
                    {
                        settings.exit_country = None;
                        country_changed = true;
                    }
                    ui.separator();
                    for country in COUNTRIES {
                        let selected = settings
                            .exit_country
                            .as_deref()
                            .map(|c| c == country.code)
                            .unwrap_or(false);
                        if ui.selectable_label(selected, country.display()).clicked() {
                            settings.exit_country = Some(country.code.to_string());
                            country_changed = true;
                        }
                    }
                });
            if country_changed {
                actions.settings_changed = true;
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            // Quick-read summary: bridges on/off. BridgeType no longer
            // has a None variant — the boolean `enabled` is the only
            // gate.
            let bridges_on = settings.bridge.enabled;
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(match lang {
                        Language::Spanish => "Puentes:",
                        Language::English => "Bridges:",
                    })
                    .color(theme::TEXT_DIM)
                    .size(12.5),
                );
                ui.label(
                    RichText::new(if bridges_on { "on" } else { "off" })
                        .color(if bridges_on { theme::OK } else { theme::TEXT_MUTED })
                        .size(12.5)
                        .strong(),
                );
            });
        },
    );
}

fn draw_bandwidth_card(
    ui: &mut Ui,
    state: &SharedState,
    traffic: &TrafficStats,
    lang: Language,
) {
    card_with_icon(
        ui,
        egui_phosphor::regular::CHART_LINE,
        i18n::t(lang, Key::TorNetwork),
        |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{}  {}",
                        egui_phosphor::regular::ARROW_DOWN,
                        format_bps(traffic.down_bps)
                    ))
                    .size(14.0)
                    .color(theme::ACCENT_STRONG)
                    .strong(),
                );
                ui.add_space(20.0);
                ui.label(
                    RichText::new(format!(
                        "{}  {}",
                        egui_phosphor::regular::ARROW_UP,
                        format_bps(traffic.up_bps)
                    ))
                    .size(14.0)
                    .color(theme::WARN)
                    .strong(),
                );
            });
            ui.add_space(10.0);

            draw_sparkline(ui, state, ui.available_width(), 90.0);

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{}:", i18n::t(lang, Key::SessionTotal)))
                        .color(theme::TEXT_DIM)
                        .size(12.5),
                );
                ui.label(
                    RichText::new(format!(
                        "{} {}   {} {}",
                        egui_phosphor::regular::ARROW_DOWN,
                        format_bytes(traffic.bytes_read),
                        egui_phosphor::regular::ARROW_UP,
                        format_bytes(traffic.bytes_written)
                    ))
                    .color(theme::TEXT)
                    .size(12.5)
                    .strong(),
                );
            });
        },
    );
}

fn draw_sparkline(ui: &mut Ui, state: &SharedState, width: f32, height: f32) {
    const MAX_SAMPLES: usize = 60;
    let id = ui.make_persistent_id("home-bw-sparkline");
    let traffic = state.traffic.read().clone();

    let mut samples: Vec<(f32, f32)> = ui
        .memory_mut(|m| m.data.get_temp::<Vec<(f32, f32)>>(id))
        .unwrap_or_default();

    let last_push: f64 = ui
        .memory_mut(|m| m.data.get_temp::<f64>(id.with("t")))
        .unwrap_or(0.0);
    let now = ui.ctx().input(|i| i.time);
    if now - last_push > 0.5 {
        samples.push((traffic.down_bps as f32, traffic.up_bps as f32));
        if samples.len() > MAX_SAMPLES {
            let overflow = samples.len() - MAX_SAMPLES;
            samples.drain(0..overflow);
        }
        ui.memory_mut(|m| m.data.insert_temp(id.with("t"), now));
    }
    ui.memory_mut(|m| m.data.insert_temp(id, samples.clone()));

    let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(
        rect,
        6.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 50),
    );

    if samples.len() < 2 {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            match state.current_status().is_connected() {
                true => "warming up…",
                false => "no data",
            },
            egui::FontId::proportional(11.0),
            theme::TEXT_MUTED,
        );
        return;
    }

    let peak = samples
        .iter()
        .map(|(d, u)| d.max(*u))
        .fold(1.0_f32, f32::max);

    let x_step = rect.width() / (MAX_SAMPLES as f32 - 1.0);
    let x_offset_samples = MAX_SAMPLES.saturating_sub(samples.len());

    let sample_point = |i: usize, v: f32| {
        let x = rect.left() + (i + x_offset_samples) as f32 * x_step;
        let y = rect.bottom() - (v / peak).clamp(0.0, 1.0) * rect.height() * 0.9 - 4.0;
        egui::pos2(x, y)
    };

    let down_pts: Vec<_> = samples.iter().enumerate().map(|(i, (d, _))| sample_point(i, *d)).collect();
    painter.add(egui::Shape::line(down_pts.clone(), Stroke::new(2.0, theme::ACCENT_STRONG)));
    if let (Some(first), Some(last)) = (down_pts.first(), down_pts.last()) {
        let mut area = down_pts.clone();
        area.push(egui::pos2(last.x, rect.bottom()));
        area.push(egui::pos2(first.x, rect.bottom()));
        painter.add(egui::Shape::convex_polygon(
            area,
            Color32::from_rgba_unmultiplied(138, 92, 255, 28),
            Stroke::NONE,
        ));
    }

    let up_pts: Vec<_> = samples.iter().enumerate().map(|(i, (_, u))| sample_point(i, *u)).collect();
    painter.add(egui::Shape::line(up_pts, Stroke::new(2.0, theme::WARN)));

    ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
}