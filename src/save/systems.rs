use std::fs;

use crate::{
    config::save::SAVE_VERSION, save::data::{SaveData, SaveFile, SaveMetadata, compute_checksum, meta_file_path, save_file_path},
};

pub fn load_save_data(slot: usize) -> Result<SaveData, String> {
    let config = bincode_next::config::standard();

    let path = save_file_path(slot);
    let file_bytes = fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
    let (save_file, _): (SaveFile, usize) =
        bincode_next::serde::decode_from_slice(&file_bytes, config)
            .map_err(|e| format!("Deserialize error: {}", e))?;

    let computed = compute_checksum(&save_file.data);
    if computed != save_file.checksum {
        return Err("Save file corrupted or tampered with!".into());
    }

    let (save_data, _): (SaveData, usize) =
        bincode_next::serde::decode_from_slice(&save_file.data, config)
            .map_err(|e| format!("Data deserialize error: {}", e))?;

    if save_data.version != SAVE_VERSION {
        return Err(format!(
            "Incompatible save version: {} (expected: {})",
            save_data.version, SAVE_VERSION
        ));
    }

    Ok(save_data)
}

pub fn load_slot_metadata(slot: usize) -> Option<SaveMetadata> {
    let config = bincode_next::config::standard();
    let path = meta_file_path(slot);
    let bytes = fs::read(&path).ok()?;
    bincode_next::serde::decode_from_slice(&bytes, config).ok()?.0
}
