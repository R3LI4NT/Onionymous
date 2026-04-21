use eframe::egui::{self, Layout, RichText, Ui};

use crate::config::countries::COUNTRIES;
use crate::config::i18n::{self, Key, Language};
use crate::core::routing::BridgeType;
use crate::core::state::SharedState;
use crate::gui::theme;
use crate::gui::widgets::{card_with_icon, field_label, toggle};

pub struct SettingsActions {
    pub settings_changed: bool,
    pub autostart_changed: Option<bool>,
    pub interaction_ticks: u32,
}

impl SettingsActions {
    fn new() -> Self {
        Self {
            settings_changed: false,
            autostart_changed: None,
            interaction_ticks: 0,
        }
    }
}

pub fn show(ui: &mut Ui, state: &SharedState, lang: Language) -> SettingsActions {
    let mut actions = SettingsActions::new();

    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_with_icon(ui, egui_phosphor::regular::SLIDERS, i18n::t(lang, Key::AppBehaviour), |ui| {
            let mut settings = state.settings.write();

            let old_autostart = settings.start_with_system;
            if toggle_row(ui, i18n::t(lang, Key::StartWithSystem), &mut settings.start_with_system) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
                if settings.start_with_system != old_autostart {
                    actions.autostart_changed = Some(settings.start_with_system);
                }
            }

            if toggle_row(ui, i18n::t(lang, Key::StartMinimized), &mut settings.start_minimized) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
            if toggle_row(ui, i18n::t(lang, Key::MinimizeToTray), &mut settings.minimize_to_tray) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
        });

        ui.add_space(14.0);

        card_with_icon(ui, egui_phosphor::regular::BRIDGE, i18n::t(lang, Key::BridgesTitle), |ui| {
            let mut settings = state.settings.write();
            if toggle_row(ui, i18n::t(lang, Key::BridgesEnable), &mut settings.bridge.enabled) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
            ui.add_space(4.0);
            field_label(ui, i18n::t(lang, Key::BridgesTransport));
            let mut changed = false;
            egui::ComboBox::from_id_source("bridge-combo")
                .width(ui.available_width())
                .selected_text(settings.bridge.bridge_type.label())
                .show_ui(ui, |ui| {
                    for &bt in BridgeType::all() {
                        if ui.selectable_value(&mut settings.bridge.bridge_type, bt, bt.label()).changed() {
                            changed = true;
                        }
                    }
                });
            if changed { actions.settings_changed = true; actions.interaction_ticks += 1; }

            if settings.bridge.bridge_type == BridgeType::Custom {
                ui.add_space(8.0);
                field_label(ui, i18n::t(lang, Key::BridgesCustom));
                let mut joined = settings.bridge.custom_bridges.join("\n");
                let response = ui.add(
                    egui::TextEdit::multiline(&mut joined)
                        .desired_rows(4)
                        .desired_width(ui.available_width())
                        .font(egui::TextStyle::Monospace),
                );
                if response.changed() {
                    settings.bridge.custom_bridges =
                        joined.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                    actions.settings_changed = true; actions.interaction_ticks += 1;
                }
            }
        });

        ui.add_space(14.0);

        card_with_icon(ui, egui_phosphor::regular::SHIELD_CHECK, i18n::t(lang, Key::Security), |ui| {
            let mut settings = state.settings.write();
            if toggle_row(ui, i18n::t(lang, Key::KillSwitch), &mut settings.kill_switch) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
        });

        ui.add_space(14.0);

        card_with_icon(ui, egui_phosphor::regular::PLUGS, i18n::t(lang, Key::Ports), |ui| {
            let mut settings = state.settings.write();
            if port_field(ui, i18n::t(lang, Key::SocksPort), &mut settings.socks_port) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
            if port_field(ui, i18n::t(lang, Key::ControlPort), &mut settings.control_port) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
            if port_field(ui, i18n::t(lang, Key::DnsPort), &mut settings.dns_port) {
                actions.settings_changed = true; actions.interaction_ticks += 1;
            }
        });

        ui.add_space(14.0);

        card_with_icon(ui, egui_phosphor::regular::PROHIBIT, i18n::t(lang, Key::ExcludedCountries), |ui| {
            ui.label(
                RichText::new(i18n::t(lang, Key::ExcludedDesc))
                    .color(theme::TEXT_DIM)
                    .size(12.5),
            );
            ui.add_space(6.0);

            let mut settings = state.settings.write();
            egui::Grid::new("excluded-grid").num_columns(3).spacing([14.0, 6.0]).show(ui, |ui| {
                for (idx, country) in COUNTRIES.iter().enumerate() {
                    let mut ticked = settings.excluded_countries.iter().any(|c| c == country.code);
                    if ui.checkbox(&mut ticked, country.display()).changed() {
                        if ticked {
                            if !settings.excluded_countries.iter().any(|c| c == country.code) {
                                settings.excluded_countries.push(country.code.to_string());
                            }
                        } else {
                            settings.excluded_countries.retain(|c| c != country.code);
                        }
                        actions.settings_changed = true; actions.interaction_ticks += 1;
                    }
                    if (idx + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }
            });
        });

        ui.add_space(20.0);
    });

    actions
}

fn toggle_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    let mut flipped = false;
    ui.horizontal(|ui| {
        ui.label(label);
        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
            if toggle(ui, value) {
                flipped = true;
            }
        });
    });
    flipped
}

fn port_field(ui: &mut Ui, label: &str, value: &mut u16) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.label(label);
        ui.with_layout(Layout::right_to_left(egui::Align::Min), |ui| {
            let mut s = value.to_string();
            let response = ui.add(egui::TextEdit::singleline(&mut s).desired_width(80.0));
            if response.changed() {
                if let Ok(p) = s.parse::<u16>() {
                    if p > 0 && p != *value {
                        *value = p;
                        changed = true;
                    }
                }
            }
        });
    });
    changed
}
