//! The approved PhotoStrike butterfly shared by the title bar and native window.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};

/// The unchanged approved suite artwork; its exact bytes/provenance live beside the PNG.
const PNG: &[u8] = include_bytes!("../../../assets/branding/PhotoStrike.png");

/// One decoded source for the native window and the title-bar texture. Derive dimensions from
/// the asset rather than silently assuming the old placeholder's 128-pixel size.
pub fn icon() -> egui::IconData {
    let image = photocraft_codecs::decode(PNG).expect("the embedded approved PhotoStrike PNG must decode");
    egui::IconData { rgba: image.to_rgba8(), width: image.width(), height: image.height() }
}

fn decode() -> egui::ColorImage {
    let icon = icon();
    egui::ColorImage::from_rgba_unmultiplied([icon.width as usize, icon.height as usize], &icon.rgba)
}

fn texture(ctx: &Context) -> TextureHandle {
    let id = Id::new("photostrike-brand-mark");
    if let Some(tex) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return tex;
    }
    let options = TextureOptions { mipmap_mode: Some(egui::TextureFilter::Linear), ..TextureOptions::LINEAR };
    let tex = ctx.load_texture("photostrike-brand-mark", decode(), options);
    ctx.data_mut(|d| d.insert_temp(id, tex.clone()));
    tex
}

fn rect_id() -> Id {
    Id::new("photostrike-brand-mark-rect")
}

pub fn paint_mark(ui: &Ui, r: Rect) {
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    ui.painter().image(texture(ui.ctx()).id(), r, uv, Color32::WHITE);
    ui.ctx().data_mut(|d| d.insert_temp(rect_id(), r));
}

#[cfg(test)]
pub fn mark_rect(ctx: &Context) -> Option<Rect> {
    ctx.data(|d| d.get_temp(rect_id()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_icon_preserves_the_approved_png_pixels_and_dimensions() {
        let expected = photocraft_codecs::decode(PNG).unwrap();
        let icon = icon();
        assert_eq!((icon.width, icon.height), (512, 512));
        assert_eq!(icon.rgba, expected.to_rgba8());
        let img = decode();
        assert_eq!(img.size, [512, 512]);
        assert_eq!(img.pixels[0].a(), 0);
        // The approved 512px handoff center is RGBA (7, 5, 6, 254), not fully opaque.
        assert_eq!(img.pixels[256 * 512 + 256].a(), 254);
    }

    #[test]
    fn the_title_bar_paints_the_icon_from_one_cached_texture() {
        let ctx = Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
            let uploads = out.textures_delta.set.values().flat_map(|d| d.iter()).filter(|d| d.image.size() == [512, 512]).count();
            out.textures_delta.clear();
            uploads
        };
        assert_eq!(frame(), 1, "uploaded on first use");
        assert_eq!(frame(), 0, "then reused");
        assert!(mark_rect(&ctx).is_some());
    }
}
