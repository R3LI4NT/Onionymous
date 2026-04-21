#![allow(clippy::needless_return)]

use eframe::egui::{self, Color32, Layout, RichText, Stroke, Ui, Vec2};

use crate::config::i18n::{self, Key, Language};
use crate::core::state::{LogLevel, SharedState};
use crate::gui::theme;
use crate::gui::widgets::card_with_icon;

pub struct AccionesHerramientas {
    pub iniciar_actualizacion: bool,
}

impl AccionesHerramientas {
    fn nueva() -> Self {
        Self {
            iniciar_actualizacion: false,
        }
    }
}

pub fn mostrar(ui: &mut Ui, estado: &SharedState, lang: Language) -> AccionesHerramientas {
    let mut acciones = AccionesHerramientas::nueva();

    let en_progreso = estado
        .update_in_progress
        .load(std::sync::atomic::Ordering::Relaxed);

    card_with_icon(
        ui,
        egui_phosphor::regular::WRENCH,
        i18n::t(lang, Key::TorRuntime),
        |ui| {
            let snapshot = estado.settings_snapshot();

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{}:", i18n::t(lang, Key::CurrentVersion)))
                        .color(theme::TEXT_DIM)
                        .size(13.0),
                );
                ui.add_space(4.0);

                let version_mostrada = if let Some(v) = &snapshot.version_tor_instalada {
                    v.clone()
                } else {
                    let id = ui.make_persistent_id("version-tor-cache");
                    let cacheada: Option<String> =
                        ui.memory_mut(|m| m.data.get_temp::<String>(id));
                    match cacheada {
                        Some(v) if !v.is_empty() => v,
                        Some(_) => i18n::t(lang, Key::EmbeddedBundle).to_string(),
                        None => {
                            let detectada =
                                crate::network::tor_updater::detectar_version_instalada();
                            let a_guardar = detectada.clone().unwrap_or_default();
                            ui.memory_mut(|m| m.data.insert_temp(id, a_guardar));
                            detectada.unwrap_or_else(|| {
                                i18n::t(lang, Key::EmbeddedBundle).to_string()
                            })
                        }
                    }
                };

                ui.label(
                    RichText::new(version_mostrada)
                        .color(Color32::from_rgb(255, 214, 77))
                        .strong()
                        .size(13.5)
                        .monospace(),
                );
            });

            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label(
                    RichText::new(format!("{}:", i18n::t(lang, Key::LastChecked)))
                        .color(theme::TEXT_DIM)
                        .size(13.0),
                );
                ui.add_space(4.0);
                let texto_fecha = match &snapshot.ultima_actualizacion_tor {
                    Some(fecha) => formatear_fecha_humana(fecha, lang),
                    None => i18n::t(lang, Key::NeverUpdated).to_string(),
                };
                ui.label(RichText::new(texto_fecha).color(theme::TEXT).size(13.0));
            });

            ui.add_space(14.0);

            let etiqueta_boton = if en_progreso {
                i18n::t(lang, Key::Updating).to_string()
            } else {
                format!(
                    "{}  {}",
                    egui_phosphor::regular::DOWNLOAD_SIMPLE,
                    i18n::t(lang, Key::UpdateTor)
                )
            };

            let respuesta = ui.add_enabled(
                !en_progreso,
                egui::Button::new(
                    RichText::new(etiqueta_boton)
                        .color(Color32::WHITE)
                        .strong()
                        .size(14.0),
                )
                .fill(if en_progreso {
                    Color32::from_rgba_unmultiplied(138, 92, 255, 80)
                } else {
                    theme::ACCENT
                })
                .stroke(Stroke::new(1.0, theme::ACCENT_STRONG))
                .rounding(10.0)
                .min_size(Vec2::new(200.0, 38.0)),
            );

            if respuesta.clicked() && !en_progreso {
                acciones.iniciar_actualizacion = true;
            }

            ui.add_space(4.0);
            ui.label(
                RichText::new(i18n::t(lang, Key::UpdateTorHint))
                    .color(theme::TEXT_DIM)
                    .size(11.5),
            );
        },
    );

    ui.add_space(14.0);

    card_with_icon(
        ui,
        egui_phosphor::regular::TERMINAL,
        i18n::t(lang, Key::UpdateOutput),
        |ui| {
            dibujar_terminal_actualizacion(ui, estado);
        },
    );

    acciones
}

fn dibujar_terminal_actualizacion(ui: &mut Ui, estado: &SharedState) {
    let altura = 280.0;
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), altura),
        egui::Sense::hover(),
    );

    let painter = ui.painter();
    painter.rect_filled(
        rect,
        8.0,
        Color32::from_rgba_unmultiplied(0, 0, 0, 180),
    );
    painter.rect_stroke(
        rect,
        8.0,
        Stroke::new(1.0, Color32::from_rgba_unmultiplied(255, 255, 255, 22)),
    );

    let entradas = estado.update_log.read().clone();
    if entradas.is_empty() {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "— sin actividad —",
            egui::FontId::monospace(12.0),
            theme::TEXT_MUTED,
        );
        return;
    }

    let area_scroll = egui::Rect::from_min_size(
        rect.min + Vec2::new(10.0, 10.0),
        rect.size() - Vec2::new(20.0, 20.0),
    );
    let mut child = ui.child_ui(area_scroll, Layout::top_down(egui::Align::Min), None);

    egui::ScrollArea::vertical()
        .auto_shrink([false; 2])
        .stick_to_bottom(true)
        .show(&mut child, |ui| {
            for entrada in entradas.iter() {
                let hora = entrada.timestamp.format("%H:%M:%S").to_string();
                let color = match entrada.level {
                    LogLevel::Error => Color32::from_rgb(240, 96, 110),
                    LogLevel::Warn => Color32::from_rgb(255, 191, 0),
                    LogLevel::Notice => Color32::from_rgb(92, 224, 148),
                    LogLevel::Info => Color32::from_rgb(200, 200, 220),
                    LogLevel::Debug => theme::TEXT_DIM,
                };
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(&hora)
                            .monospace()
                            .size(11.5)
                            .color(theme::TEXT_MUTED),
                    );
                    ui.label(
                        RichText::new(&entrada.message)
                            .monospace()
                            .size(11.5)
                            .color(color),
                    );
                });
            }
        });

    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
}

fn formatear_fecha_humana(iso: &str, lang: Language) -> String {
    use chrono::{DateTime, Local};
    let parsed = DateTime::parse_from_rfc3339(iso);
    match parsed {
        Ok(fecha) => {
            let local: DateTime<Local> = fecha.with_timezone(&Local);
            let ahora = Local::now();
            let delta = ahora.signed_duration_since(local);
            let dias = delta.num_days();
            if dias == 0 {
                match lang {
                    Language::Spanish => format!("hoy ({})", local.format("%H:%M")),
                    Language::English => format!("today ({})", local.format("%H:%M")),
                }
            } else if dias < 0 {
                local.format("%Y-%m-%d %H:%M").to_string()
            } else {
                match lang {
                    Language::Spanish => format!("hace {} días ({})", dias, local.format("%Y-%m-%d")),
                    Language::English => format!("{} days ago ({})", dias, local.format("%Y-%m-%d")),
                }
            }
        }
        Err(_) => iso.to_string(),
    }
}
