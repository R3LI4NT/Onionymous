use eframe::egui::{self, Color32, RichText, Ui};

use crate::config::i18n::{self, Key, Language};
use crate::core::state::{LogLevel, LogSource, SharedState};
use crate::gui::theme;
use crate::gui::widgets::card_with_icon;

pub fn show(ui: &mut Ui, state: &SharedState, lang: Language) {
    card_with_icon(
        ui,
        egui_phosphor::regular::LIST_MAGNIFYING_GLASS,
        i18n::t(lang, Key::LatestEvents),
        |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button(i18n::t(lang, Key::Clear)).clicked() {
                        state.logs.write().clear();
                    }
                });
            });
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .max_height(560.0)
                .show(ui, |ui| {
                    let logs = state.logs.read();
                    if logs.is_empty() {
                        ui.label(
                            RichText::new(i18n::t(lang, Key::NoLogs))
                                .color(theme::TEXT_DIM),
                        );
                        return;
                    }
                    for entry in logs.iter() {
                        let level_color = match entry.level {
                            LogLevel::Debug => theme::TEXT_MUTED,
                            LogLevel::Info => theme::TEXT_DIM,
                            LogLevel::Notice => theme::ACCENT_STRONG,
                            LogLevel::Warn => theme::WARN,
                            LogLevel::Error => theme::DANGER,
                        };
                        let source_badge = match entry.source {
                            LogSource::App => "APP",
                            LogSource::Tor => "TOR",
                            LogSource::Dns => "DNS",
                        };
                        let ts = entry.timestamp.format("%H:%M:%S").to_string();
                        ui.horizontal_wrapped(|ui| {
                            ui.label(
                                RichText::new(format!("{}  ", ts))
                                    .color(theme::TEXT_MUTED)
                                    .monospace(),
                            );
                            ui.label(
                                RichText::new(format!("[{}]", source_badge))
                                    .color(Color32::from_rgb(168, 130, 255))
                                    .monospace(),
                            );
                            ui.label(
                                RichText::new(format!(" {}", entry.message))
                                    .color(level_color)
                                    .monospace(),
                            );
                        });
                    }
                });
        },
    );
}
