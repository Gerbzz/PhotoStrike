//! Runtime window icon from the interim PhotoStrike placeholder shared with the title bar.

pub fn window_icon() -> egui::IconData {
    egui::IconData { rgba: photocraft_ui_egui::brand::icon_rgba(), width: 128, height: 128 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn window_icon_uses_original_mark() {
        let icon = super::window_icon();
        assert_eq!((icon.width, icon.height), (128, 128));
        assert_eq!(icon.rgba.len(), 128 * 128 * 4);
        assert_eq!(icon.rgba[3], 0);
    }
}
