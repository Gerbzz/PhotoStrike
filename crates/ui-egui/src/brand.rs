//! Interim PhotoStrike placeholder mark drawn in Rust, without donor-brand image assets.
//! Replace it with the approved suite mark before any release packaging.

use egui::{Color32, Context, Id, Rect, TextureHandle, TextureOptions, Ui, pos2};

const SIZE: usize = 128;

/// Temporary rounded charcoal tile, lens ring, and warm shutter dot.
pub fn icon_rgba() -> Vec<u8> {
    let mut pixels = Vec::with_capacity(SIZE * SIZE * 4);
    for row in 0..SIZE {
        for col in 0..SIZE {
            let x = (col as f32 + 0.5) / SIZE as f32;
            let y = (row as f32 + 0.5) / SIZE as f32;
            let cx = (x - 0.5).abs() - 0.31;
            let cy = (y - 0.5).abs() - 0.31;
            let outside = cx.max(0.0).hypot(cy.max(0.0)) - 0.18;
            let alpha = ((-outside * SIZE as f32).clamp(0.0, 1.0) * 255.0) as u8;
            let lens = (x - 0.50).hypot(y - 0.50);
            let dot = (x - 0.70).hypot(y - 0.30);
            let color = if dot < 0.055 {
                [255, 178, 109]
            } else if (0.235..0.285).contains(&lens) {
                [110, 226, 210]
            } else if lens < 0.185 {
                [36, 75, 89]
            } else {
                [25, 31, 45]
            };
            pixels.extend_from_slice(&[color[0], color[1], color[2], alpha]);
        }
    }
    pixels
}

fn decode() -> egui::ColorImage {
    egui::ColorImage::from_rgba_unmultiplied([SIZE, SIZE], &icon_rgba())
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
    fn the_icon_decodes_to_a_rounded_128_px_tile() {
        let img = decode();
        assert_eq!(img.size, [128, 128]);
        assert_eq!(img.pixels[0].a(), 0);
        assert_eq!(img.pixels[64 * 128 + 64].a(), 255);
    }

    #[test]
    fn the_title_bar_paints_the_icon_from_one_cached_texture() {
        let ctx = Context::default();
        crate::PhotocraftApp::setup_context(&ctx, crate::theme::ThemeKind::ALL[0]);
        let mut app = crate::PhotocraftApp::new(photocraft_engine::Session::new(), crate::Services::default());
        app.custom_titlebar = true;
        let mut frame = || {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| crate::panels::title_bar(&mut app, ui));
            let uploads = out.textures_delta.set.values().flat_map(|d| d.iter()).filter(|d| d.image.size() == [128, 128]).count();
            out.textures_delta.clear();
            uploads
        };
        assert_eq!(frame(), 1, "uploaded on first use");
        assert_eq!(frame(), 0, "then reused");
        assert!(mark_rect(&ctx).is_some());
    }
}
