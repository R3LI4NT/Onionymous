pub mod app;
pub mod theme;
pub mod widgets;
pub mod views;
pub mod logo;

#[cfg(windows)]
pub mod blur;

#[cfg(not(windows))]
pub mod blur {
    pub fn enable_blur_for_window(_cc: &eframe::CreationContext<'_>) {}
}
