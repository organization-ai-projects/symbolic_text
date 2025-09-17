// src/format_converter.rs
use bincode::{config::standard, decode_from_slice, encode_to_vec};
use ron::de::from_str;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// Convertit un fichier .ron en .bin (bincode v2, générique)
pub fn ron_to_bin<T>(ron_path: &Path, bin_path: &Path) -> io::Result<()>
where
    T: DeserializeOwned + serde::Serialize + bincode::Encode,
{
    let ron_content = fs::read_to_string(ron_path)?;
    let data: T = from_str(&ron_content).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("RON parse error: {}", e),
        )
    })?;
    let bin_data = encode_to_vec(&data, standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode error: {}", e)))?;
    let mut file = fs::File::create(bin_path)?;
    file.write_all(&bin_data)?;
    Ok(())
}

/// Convertit un fichier .bin (bincode v2) en type générique
pub fn load_bin<T>(bin_path: &Path) -> io::Result<T>
where
    T: serde::de::DeserializeOwned + bincode::Decode<()>,
{
    let bin_data = fs::read(bin_path)?;
    let (data, _len): (T, usize) = decode_from_slice(&bin_data, standard())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Bincode error: {}", e)))?;
    Ok(data)
}

/// Vérifie si le .bin est à jour par rapport au .ron, et le régénère si besoin
/// Retourne Ok(true) si conversion effectuée, Ok(false) sinon
pub fn ensure_bin_up_to_date<F>(ron_path: &Path, bin_path: &Path, convert: F) -> io::Result<bool>
where
    F: Fn(&Path, &Path) -> io::Result<()>,
{
    let ron_meta = fs::metadata(ron_path)?;
    let bin_meta = fs::metadata(bin_path).ok();
    let ron_mtime = ron_meta.modified()?;
    let bin_mtime = bin_meta.and_then(|m| m.modified().ok());
    let needs_update = match bin_mtime {
        Some(bin_time) => ron_mtime > bin_time,
        None => true,
    };
    if needs_update {
        convert(ron_path, bin_path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}
