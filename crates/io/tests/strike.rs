//! The app's import/export dispatcher must preserve PhotoStrike identity and editability.

mod common;

use common::{Features, gen_doc};
use photocraft_color::{ColorMode, SampleType};
use photocraft_io::{ExportOptions, export, import};

#[test]
fn native_dispatch_round_trips_a_layered_document() {
    let original = gen_doc(ColorMode::Rgb, SampleType::U16, Features::ALL);
    let encoded = export(&original, "portrait.pstrike", &ExportOptions::default()).unwrap();
    let reopened = import("portrait.pstrike", &encoded.bytes).unwrap();
    assert!(!reopened.source_read_only);
    assert!(!reopened.preview_only);
    assert_eq!(reopened.document, original);
    assert_eq!(photocraft_format::read_manifest(&encoded.bytes).unwrap().strike.unwrap().format, "photostrike");
}

#[test]
fn legacy_import_requires_a_new_save_path_and_a_renamed_bundle_is_rejected() {
    let original = gen_doc(ColorMode::Rgb, SampleType::U8, Features::PIXELS);
    let legacy = export(&original, "source.pcraft", &ExportOptions::default()).unwrap();
    let imported = import("source.pcraft", &legacy.bytes).unwrap();
    assert!(imported.source_read_only);
    assert_eq!(imported.document, original);
    assert!(import("renamed.pstrike", &legacy.bytes).is_err());
    assert!(import("broken.pstrike", b"not a project").is_err());
}
