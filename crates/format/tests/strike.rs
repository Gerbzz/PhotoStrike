//! Native PhotoStrike identity over the retained editable tile bundle.

mod common;
use common::*;
use photocraft_color::{ColorMode, SampleType};
use photocraft_format::{PcraftWriter, STRIKE_EXTENSION, SaveOptions, load_strike_from_bytes, read_manifest, save_strike_to_bytes, save_to_bytes};

fn document() -> photocraft_doc::Document {
    rich_doc(ColorMode::Rgb, SampleType::U16)
}

#[test]
fn strike_round_trip_keeps_the_complete_document_and_legacy_stays_importable() {
    let original = document();
    let bytes = save_strike_to_bytes(&original, &SaveOptions::default()).unwrap();
    let manifest = read_manifest(&bytes).unwrap();
    let identity = manifest.strike.as_ref().unwrap();
    assert_eq!((identity.format.as_str(), identity.schema_version), ("photostrike", 1));
    assert_eq!(load_strike_from_bytes(&bytes).unwrap(), original);

    let legacy = save_to_bytes(&original, &SaveOptions::default()).unwrap();
    assert!(read_manifest(&legacy).unwrap().strike.is_none());
    assert!(load_strike_from_bytes(&legacy).is_err(), "a renamed .pcraft is not a .pstrike");
    assert_eq!(photocraft_format::load_from_bytes(&legacy).unwrap(), original);
}

#[test]
fn malformed_and_ambiguous_strike_archives_are_rejected() {
    use photocraft_format::zip::{ZipReader, ZipWriter};

    let valid = save_strike_to_bytes(&document(), &SaveOptions::default()).unwrap();
    assert!(load_strike_from_bytes(&valid[..valid.len() / 2]).is_err());
    let manifest = ZipReader::new(&valid).unwrap().read_by_name("manifest.json", 1 << 20).unwrap();

    for extra in ["../outside", "tiles/../../outside.zst", "C:/outside"] {
        let mut writer = ZipWriter::new();
        writer.add("manifest.json", &manifest).unwrap();
        writer.add(extra, b"unexpected").unwrap();
        let err = load_strike_from_bytes(&writer.finish().unwrap()).unwrap_err().to_string();
        assert!(err.contains("unsafe"), "{extra}: {err}");
    }

    let mut writer = ZipWriter::new();
    writer.add("manifest.json", &manifest).unwrap();
    writer.add("manifest.json", &manifest).unwrap();
    assert!(load_strike_from_bytes(&writer.finish().unwrap()).unwrap_err().to_string().contains("duplicate"));

    let mut unknown = read_manifest(&valid).unwrap();
    unknown.strike.as_mut().unwrap().schema_version = 99;
    let mut writer = ZipWriter::new();
    writer.add("manifest.json", &serde_json::to_vec(&unknown).unwrap()).unwrap();
    assert!(load_strike_from_bytes(&writer.finish().unwrap()).unwrap_err().to_string().contains("schema 99"));
}

#[test]
fn overwrite_is_atomic_and_an_invalid_revision_preserves_the_last_good_project() {
    let dir = temp_dir("strike-overwrite");
    let path = dir.join(format!("portrait.{STRIKE_EXTENSION}"));
    let mut original = document();
    let mut writer = PcraftWriter::new_strike();
    writer.save_path(&original, &path, &SaveOptions::default()).unwrap();
    let first = std::fs::read(&path).unwrap();

    original.layers[0].name = "Edited and saved again".into();
    writer.save_path(&original, &path, &SaveOptions::default()).unwrap();
    let second = std::fs::read(&path).unwrap();
    assert_ne!(second, first);
    assert_eq!(load_strike_from_bytes(&second).unwrap(), original);

    original.resolution_dpi = f32::NAN;
    assert!(writer.save_path(&original, &path, &SaveOptions::default()).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), second, "failed save never replaces the project");
    std::fs::remove_dir_all(dir).unwrap();
}
