//! Native window icon from the same approved PhotoStrike PNG as the title bar.

pub fn window_icon() -> egui::IconData {
    photocraft_ui_egui::brand::icon()
}

#[cfg(test)]
mod tests {
    #[test]
    fn window_icon_preserves_the_approved_mark() {
        let icon = super::window_icon();
        let expected = photocraft_ui_egui::brand::icon();
        assert_eq!((icon.width, icon.height), (656, 656));
        assert_eq!(icon.rgba, expected.rgba);
        assert_eq!(icon.rgba.len(), 656 * 656 * 4);
        assert_eq!(icon.rgba[3], 0);
    }
}
