use eframe::egui::{ColorImage, Context, TextureHandle, TextureOptions};

pub fn load_logo(ctx: &Context) -> Option<TextureHandle> {
    let bytes = crate::resources::LOGO_ICO;
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (w, h) = image.dimensions();
    let pixels =
        ColorImage::from_rgba_unmultiplied([w as usize, h as usize], image.as_raw());
    Some(ctx.load_texture("onionymous-logo", pixels, TextureOptions::LINEAR))
}
