use eframe::egui::{self, Color32, Layout, RichText, Stroke};
use std::sync::Arc;
use tokio::runtime::Runtime;

use crate::config::i18n::{self, Language};
use crate::core::connection::ConnectionStatus;
use crate::core::state::SharedState;
use crate::gui::theme;
use crate::gui::views;
use crate::tor::bootstrap::TorBootstrap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Settings,
    Logs,
    Tools,
    About,
}

impl Screen {
    fn icon(&self) -> &'static str {
        match self {
            Screen::Home => egui_phosphor::regular::HOUSE,
            Screen::Settings => egui_phosphor::regular::GEAR_SIX,
            Screen::Logs => egui_phosphor::regular::LIST_BULLETS,
            Screen::Tools => egui_phosphor::regular::WRENCH,
            Screen::About => egui_phosphor::regular::INFO,
        }
    }

    fn i18n_key(&self) -> i18n::Key {
        match self {
            Screen::Home => i18n::Key::NavDashboard,
            Screen::Settings => i18n::Key::NavSettings,
            Screen::Logs => i18n::Key::NavLogs,
            Screen::Tools => i18n::Key::Tools,
            Screen::About => i18n::Key::NavAbout,
        }
    }
}

pub struct OnionymousApp {
    state: SharedState,
    runtime: Arc<Runtime>,
    screen: Screen,
    bootstrap: Arc<tokio::sync::Mutex<Option<Arc<TorBootstrap>>>>,
    logo: Option<egui::TextureHandle>,
    pending_start_minimized: bool,
    audio: Arc<crate::utils::audio::AudioEngine>,
    last_status_kind: StatusKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusKind {
    Disconnected,
    Connecting,
    Connected,
    Failed,
    Disconnecting,
}

impl StatusKind {
    fn from(s: &ConnectionStatus) -> Self {
        match s {
            ConnectionStatus::Disconnected => Self::Disconnected,
            ConnectionStatus::Connecting { .. } => Self::Connecting,
            ConnectionStatus::Connected => Self::Connected,
            ConnectionStatus::Failed(_) => Self::Failed,
            ConnectionStatus::Disconnecting => Self::Disconnecting,
        }
    }
}

impl OnionymousApp {
    pub fn new(
        state: SharedState,
        runtime: Arc<Runtime>,
        cc: &eframe::CreationContext<'_>,
        start_minimized: bool,
    ) -> Self {
        let logo = crate::gui::logo::load_logo(&cc.egui_ctx);
        if logo.is_none() {
            log::info!("assets/logo.ico not found — using fallback logo glyph");
        }
        let app = Self {
            state: state.clone(),
            runtime: runtime.clone(),
            screen: Screen::Home,
            bootstrap: Arc::new(tokio::sync::Mutex::new(None)),
            logo,
            pending_start_minimized: start_minimized,
            audio: Arc::new(crate::utils::audio::AudioEngine::new()),
            last_status_kind: StatusKind::Disconnected,
        };

        let desired_autostart = state.settings_snapshot().start_with_system;
        runtime.spawn(async move {
            if let Err(e) = crate::network::autostart::set_autostart(desired_autostart) {
                log::warn!("Reconciling autostart on launch failed: {e}");
            }
        });

        app
    }

    fn spawn_connect(&self) {
        let state = self.state.clone();
        let bootstrap_holder = self.bootstrap.clone();
        self.runtime.spawn(async move {
            let mut holder = bootstrap_holder.lock().await;
            let bootstrap = if let Some(b) = holder.clone() {
                b
            } else {
                match TorBootstrap::new(state.clone()) {
                    Ok(b) => {
                        let arc = Arc::new(b);
                        *holder = Some(arc.clone());
                        arc
                    }
                    Err(e) => {
                        log::error!("Failed to init TorBootstrap: {e}");
                        state.set_status(ConnectionStatus::Failed(format!(
                            "Could not initialise Tor: {e}"
                        )));
                        return;
                    }
                }
            };
            drop(holder);

            if let Err(e) = bootstrap.connect().await {
                log::error!("Connect failed: {e}");
                state.set_status(ConnectionStatus::Failed(format!("{e}")));
            }
        });
    }

    fn spawn_disconnect(&self) {
        let state = self.state.clone();
        let holder = self.bootstrap.clone();
        self.runtime.spawn(async move {
            let guard = holder.lock().await;
            if let Some(bootstrap) = guard.clone() {
                drop(guard);
                if let Err(e) = bootstrap.disconnect().await {
                    log::error!("Disconnect failed: {e}");
                    state.set_status(ConnectionStatus::Failed(format!("{e}")));
                }
            }
        });
    }

    fn spawn_new_identity(&self) {
        let holder = self.bootstrap.clone();
        self.runtime.spawn(async move {
            let guard = holder.lock().await;
            if let Some(bootstrap) = guard.clone() {
                drop(guard);
                if let Err(e) = bootstrap.new_identity().await {
                    log::error!("NEWNYM failed: {e}");
                }
            }
        });
    }

    fn spawn_refresh_ip(&self) {
        let state = self.state.clone();
        self.runtime.spawn(async move {
            let socks_port = if state.current_status().is_connected() {
                Some(state.settings_snapshot().socks_port)
            } else {
                None
            };
            match crate::network::ip_lookup::fetch_public_ip(socks_port).await {
                Ok(ip) => {
                    *state.current_ip.write() = Some(ip);
                }
                Err(e) => {
                    log::warn!("IP lookup failed: {e}");
                }
            }
        });
    }

    fn persist_settings(&self) {
        let settings = self.state.settings_snapshot();
        self.runtime.spawn(async move {
            if let Err(e) = settings.save() {
                log::error!("Persisting settings failed: {e}");
            }
        });
    }

    fn spawn_set_autostart(&self, enabled: bool) {
        self.runtime.spawn(async move {
            if let Err(e) = crate::network::autostart::set_autostart(enabled) {
                log::error!("Toggling autostart failed: {e}");
            }
        });
    }

    fn spawn_tor_update(&self) {
        use std::sync::atomic::Ordering;
        if self.state.update_in_progress.swap(true, Ordering::Relaxed) {
            return;
        }
        self.state.update_log.write().clear();
        let estado = self.state.clone();
        self.runtime.spawn(async move {
            let reportero = std::sync::Arc::new(
                crate::network::tor_updater::Reportero::nuevo(estado.clone()),
            );
            let resultado = crate::network::tor_updater::descargar_e_instalar(
                estado.clone(),
                reportero.clone(),
            )
            .await;
            if let Err(e) = resultado {
                reportero.error(format!("Actualización abortada: {:#}", e));
                estado.update_last_result.store(1, Ordering::Relaxed);
            } else {
                estado.update_last_result.store(2, Ordering::Relaxed);
            }
            estado.update_in_progress.store(false, Ordering::Relaxed);
        });
    }

    fn spawn_exit(&self) {
        let bootstrap = self.bootstrap.clone();
        let state = self.state.clone();
        self.runtime.spawn(async move {
            if matches!(
                state.current_status(),
                ConnectionStatus::Connected | ConnectionStatus::Connecting { .. }
            ) {
                let mut slot = bootstrap.lock().await;
                if let Some(b) = slot.take() {
                    let _ = b.disconnect().await;
                }
            }
            std::process::exit(0);
        });
    }


    fn draw_sidebar(&mut self, ctx: &egui::Context) {
        let lang = self.state.settings_snapshot().language;
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .exact_width(230.0)
            .frame(
                egui::Frame::none()
                    .fill(theme::SIDEBAR)
                    .inner_margin(egui::Margin {
                        left: 18.0,
                        right: 18.0,
                        top: 20.0,
                        bottom: 18.0,
                    }),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if let Some(logo) = &self.logo {
                        ui.add(egui::Image::new(logo).fit_to_exact_size(egui::vec2(40.0, 40.0)));
                    } else {
                        let (rect, _) =
                            ui.allocate_exact_size([40.0, 40.0].into(), egui::Sense::hover());
                        ui.painter().rect_filled(rect, 10.0, theme::BRAND);
                        ui.painter().text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "O",
                            egui::FontId::proportional(22.0),
                            Color32::WHITE,
                        );
                    }
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Onionymous").size(16.0).strong());
                        ui.label(
                            RichText::new("Anonymous routing")
                                .color(theme::TEXT_MUTED)
                                .size(11.0),
                        );
                    });
                });
                ui.add_space(24.0);

                ui.label(
                    RichText::new("NAVIGATION")
                        .color(theme::TEXT_MUTED)
                        .size(10.5)
                        .strong(),
                );
                ui.add_space(6.0);

                for screen in [Screen::Home, Screen::Settings, Screen::Logs, Screen::Tools, Screen::About] {
                    let label = i18n::t(lang, screen.i18n_key());
                    if self.nav_item(ui, screen, label).clicked() {
                        if self.screen != screen {
                            self.audio.play(crate::utils::audio::Sfx::Click);
                        }
                        self.screen = screen;
                    }
                }

                ui.with_layout(Layout::bottom_up(egui::Align::Min), |ui| {
                    ui.add_space(4.0);
                    let status = self.state.current_status();
                    let (dot, label_key) = match &status {
                        ConnectionStatus::Disconnected => (theme::TEXT_DIM, i18n::Key::StatusDisconnected),
                        ConnectionStatus::Connecting { .. } => (theme::WARN, i18n::Key::StatusConnecting),
                        ConnectionStatus::Connected => (theme::OK, i18n::Key::StatusConnected),
                        ConnectionStatus::Failed(_) => (theme::DANGER, i18n::Key::StatusFailed),
                        ConnectionStatus::Disconnecting => (theme::WARN, i18n::Key::StatusDisconnecting),
                    };
                    egui::Frame::none()
                        .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 10))
                        .rounding(10.0)
                        .inner_margin(egui::Margin::same(10.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                crate::gui::widgets::status_dot(ui, dot);
                                ui.label(
                                    RichText::new(i18n::t(lang, label_key))
                                        .size(13.0)
                                        .strong(),
                                );
                            });
                        });
                });
            });
    }

    fn nav_item(&self, ui: &mut egui::Ui, target: Screen, label: &str) -> egui::Response {
        let is_active = self.screen == target;
        let desired_size = egui::vec2(ui.available_width(), 42.0);
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        let painter = ui.painter();
        if is_active {
            painter.rect_filled(
                rect,
                10.0,
                Color32::from_rgba_unmultiplied(138, 92, 255, 80),
            );
            let bar_rect = egui::Rect::from_min_size(
                rect.left_top() + egui::vec2(-4.0, 8.0),
                egui::vec2(3.0, rect.height() - 16.0),
            );
            painter.rect_filled(bar_rect, 1.5, theme::ACCENT_STRONG);
        } else if response.hovered() {
            painter.rect_filled(
                rect,
                10.0,
                Color32::from_rgba_unmultiplied(255, 255, 255, 16),
            );
        }

        let icon_color = if is_active { Color32::WHITE } else { theme::TEXT_DIM };
        let text_color = if is_active { Color32::WHITE } else { theme::TEXT };

        let icon_pos = rect.left_center() + egui::vec2(16.0, 0.0);
        painter.text(
            icon_pos,
            egui::Align2::LEFT_CENTER,
            target.icon(),
            egui::FontId::proportional(18.0),
            icon_color,
        );
        let text_pos = rect.left_center() + egui::vec2(44.0, 0.0);
        painter.text(
            text_pos,
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(14.5),
            text_color,
        );

        ui.add_space(4.0);
        response
    }

    fn draw_topbar(&mut self, ctx: &egui::Context) {
        let lang = self.state.settings_snapshot().language;
        egui::TopBottomPanel::top("topbar")
            .exact_height(60.0)
            .frame(
                egui::Frame::none()
                    .fill(Color32::TRANSPARENT)
                    .inner_margin(egui::Margin::symmetric(28.0, 14.0)),
            )
            .show(ctx, |ui| {
                let bar_rect = ui.max_rect().expand2(egui::vec2(28.0, 14.0));
                theme::paint_topbar_gradient(ui, bar_rect);

                ui.horizontal(|ui| {
                    // Title = translated name of current screen.
                    ui.label(
                        RichText::new(i18n::t(lang, self.screen.i18n_key()))
                            .size(22.0)
                            .strong(),
                    );

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        self.draw_language_button(ui, lang);
                    });
                });
            });
    }

    fn draw_language_button(&self, ui: &mut egui::Ui, current: Language) {
        let id = ui.make_persistent_id("lang-picker");
        let btn_text = format!("{}  {}", egui_phosphor::regular::GLOBE, current.code());
        let button = egui::Button::new(RichText::new(btn_text).size(13.5).color(theme::TEXT))
            .fill(Color32::from_rgba_unmultiplied(255, 255, 255, 18))
            .stroke(Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(255, 255, 255, 36),
            ))
            .rounding(8.0)
            .min_size(egui::vec2(72.0, 32.0));
        let response = ui.add(button);
        if response.clicked() {
            ui.memory_mut(|m| m.toggle_popup(id));
        }
        egui::popup::popup_below_widget(
            ui,
            id,
            &response,
            egui::PopupCloseBehavior::CloseOnClick,
            |ui| {
                ui.set_min_width(140.0);
                for &lang in Language::all() {
                    let selected = lang == current;
                    let label = match lang {
                        Language::English => "🇬🇧  English",
                        Language::Spanish => "🇪🇸  Español",
                    };
                    if ui.selectable_label(selected, label).clicked() {
                        self.state.update_settings(|s| s.language = lang);
                    }
                }
            },
        );
    }
}

impl eframe::App for OnionymousApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(500));

        if self.pending_start_minimized {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            self.pending_start_minimized = false;
        }

        if ctx.input(|i| i.viewport().close_requested())
            && self.state.settings_snapshot().minimize_to_tray
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }

        let current = StatusKind::from(&self.state.current_status());
        if current != self.last_status_kind {
            use crate::utils::audio::Sfx;
            use StatusKind::*;
            match (self.last_status_kind, current) {
                (_, Connected) => self.audio.play(Sfx::Connect),
                (Connected, Disconnected) | (Disconnecting, Disconnected) => {
                    self.audio.play(Sfx::Disconnect)
                }
                (_, Failed) => self.audio.play(Sfx::Error),
                _ => {}
            }
            self.last_status_kind = current;
        }

        {
            use std::sync::atomic::Ordering;
            let flag = self.state.update_last_result.swap(0, Ordering::Relaxed);
            match flag {
                1 => self.audio.play(crate::utils::audio::Sfx::Error),
                2 => self.audio.play(crate::utils::audio::Sfx::Connect),
                _ => {}
            }
        }

        self.draw_sidebar(ctx);
        self.draw_topbar(ctx);

        let lang = self.state.settings_snapshot().language;

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::TRANSPARENT)
                    .inner_margin(egui::Margin::symmetric(28.0, 18.0)),
            )
            .show(ctx, |ui| {
                let bg_rect = ui.max_rect().expand(32.0);
                theme::paint_gradient_background(ui, bg_rect);

                match self.screen {
                    Screen::Home => {
                        let actions = views::home::show(ui, &self.state, lang);
                        if actions.connect {
                            self.audio.play(crate::utils::audio::Sfx::Click);
                            self.spawn_connect();
                        }
                        if actions.disconnect {
                            self.audio.play(crate::utils::audio::Sfx::Click);
                            self.spawn_disconnect();
                        }
                        if actions.new_identity {
                            self.audio.play(crate::utils::audio::Sfx::Tick);
                            self.spawn_new_identity();
                        }
                        if actions.refresh_ip {
                            self.audio.play(crate::utils::audio::Sfx::Tick);
                            self.spawn_refresh_ip();
                        }
                        if actions.settings_changed {
                            self.persist_settings();
                        }
                        if actions.exit {
                            self.audio.play(crate::utils::audio::Sfx::Click);
                            self.spawn_exit();
                        }
                    }
                    Screen::Settings => {
                        let actions = views::settings::show(ui, &self.state, lang);
                        if actions.settings_changed {
                            self.persist_settings();
                        }
                        if let Some(enabled) = actions.autostart_changed {
                            self.spawn_set_autostart(enabled);
                        }
                        for _ in 0..actions.interaction_ticks.min(3) {
                            self.audio.play(crate::utils::audio::Sfx::Tick);
                        }
                    }
                    Screen::Logs => views::logs::show(ui, &self.state, lang),
                    Screen::Tools => {
                        let acciones = views::tools::mostrar(ui, &self.state, lang);
                        if acciones.iniciar_actualizacion {
                            self.audio.play(crate::utils::audio::Sfx::Click);
                            self.spawn_tor_update();
                        }
                    }
                    Screen::About => views::about::show(ui, lang),
                }
            });
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
