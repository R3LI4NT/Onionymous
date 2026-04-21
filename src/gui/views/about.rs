use eframe::egui::{self, Layout, RichText, Ui};

use crate::config::i18n::{self, Key, Language};
use crate::gui::theme;
use crate::gui::widgets::card_with_icon;

const DEV_HANDLE: &str = "#R3LI4NT";
const DEV_URL: &str = "https://github.com/R3LI4NT/Onionymous";

pub fn show(ui: &mut Ui, lang: Language) {
    egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, |ui| {
        card_with_icon(
            ui,
            egui_phosphor::regular::PACKAGE,
            "Onionymous",
            |ui| {
                ui.label(
                    RichText::new(format!(
                        "{} {}",
                        i18n::t(lang, Key::Version),
                        env!("CARGO_PKG_VERSION")
                    ))
                    .color(theme::TEXT_DIM)
                    .size(12.5),
                );
                ui.add_space(8.0);
                ui.label(RichText::new(i18n::t(lang, Key::AboutBlurb)).color(theme::TEXT));
            },
        );

        ui.add_space(14.0);

        card_with_icon(
            ui,
            egui_phosphor::regular::HEART,
            i18n::t(lang, Key::Credits),
            |ui| {
                ui.label("• The Tor Project — the underlying anonymity network.");
                ui.label("• egui / eframe — immediate-mode Rust GUI framework.");
                ui.label("• Phosphor Icons — icon set used throughout the UI.");
            },
        );

        ui.add_space(14.0);

        card_with_icon(
            ui,
            egui_phosphor::regular::WARNING_CIRCLE,
            i18n::t(lang, Key::Disclaimer),
            |ui| {
                ui.label(
                    RichText::new(i18n::t(lang, Key::DisclaimerText))
                        .color(theme::TEXT_DIM),
                );
            },
        );

        let remaining = ui.available_size_before_wrap().y;
        if remaining > 60.0 {
            ui.add_space(remaining - 40.0);
        } else {
            ui.add_space(20.0);
        }
        ui.with_layout(Layout::right_to_left(egui::Align::BOTTOM), |ui| {
            draw_developer_credit(ui);
        });
    });
}

fn draw_developer_credit(ui: &mut Ui) {
    let text = RichText::new(DEV_HANDLE)
        .color(theme::ACCENT_STRONG)
        .size(12.5)
        .strong()
        .underline();
    let response = ui.add(
        egui::Label::new(text).sense(egui::Sense::click()),
    );
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if response.clicked() {
        ui.ctx().open_url(egui::OpenUrl {
            url: DEV_URL.to_string(),
            new_tab: true,
        });
    }
    response.on_hover_text(DEV_URL);
    ui.add_space(2.0);
}