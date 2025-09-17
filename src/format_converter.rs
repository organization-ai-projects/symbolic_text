// src/format_converter.rs
use bincode::{config::standard, decode_from_slice, encode_to_vec};
use ron::de::from_str;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Convertit un fichier .ron en .bin (bincode v2)
pub fn ron_to_bin(ron_path: &Path, bin_path: &Path) -> io::Result<()> {
    let ron_content = fs::read_to_string(ron_path)?;
    let table: HashMap<String, String> = from_str(&ron_content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("RON parse error: {}", e),
        )
    })?;
    let bin_data = encode_to_vec(&table, standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode error: {}", e)))?;
    let mut file = fs::File::create(bin_path)?;
    file.write_all(&bin_data)?;
    Ok(())
}

/// Convertit un fichier .bin (bincode v2) en HashMap<String, String>
pub fn load_bin(bin_path: &Path) -> io::Result<HashMap<String, String>> {
    let bin_data = fs::read(bin_path)?;
    let (table, _len): (HashMap<String, String>, usize) = decode_from_slice(&bin_data, standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode error: {}", e)))?;
    Ok(table)
}

/// Vérifie si le .bin est à jour par rapport au .ron, et le régénère si besoin
pub fn ensure_bin_up_to_date(ron_path: &Path, bin_path: &Path) -> io::Result<()> {
    let ron_meta = fs::metadata(ron_path)?;
    let bin_meta = fs::metadata(bin_path).ok();
    let ron_mtime = ron_meta.modified()?;
    let bin_mtime = bin_meta.and_then(|m| m.modified().ok());
    let needs_update = match bin_mtime {
        Some(bin_time) => ron_mtime > bin_time,
        None => true,
    };
    if needs_update {
        ron_to_bin(ron_path, bin_path)?;
    }
    Ok(())
}
