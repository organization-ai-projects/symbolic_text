use std::fs;
use std::path::Path;
use symbolic_text::format_converter::{ensure_bin_up_to_date, ron_to_bin};
use symbolic_text::lemmatizer::lemma_entries::LemmaEntries;

fn main() {
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        eprintln!("Le dossier 'data' est introuvable à la racine du projet.");
        std::process::exit(1);
    }
    println!("Traitement automatique du dossier data et de ses sous-dossiers...");
    visit_ron_files(data_dir);
}

fn visit_ron_files(dir: &Path) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_ron_files(&path);
            } else if path.extension().is_some_and(|ext| ext == "ron") {
                // Génère le .bin dans le dossier bin/ à côté du dossier ron/
                let bin_path = if let (Some(parent), Some(grandparent)) =
                    (path.parent(), path.parent().and_then(|p| p.parent()))
                {
                    let bin_dir = grandparent.join("bin");
                    if !bin_dir.exists() {
                        let _ = fs::create_dir_all(&bin_dir);
                    }
                    let file_name = path.file_stem().unwrap_or_default();
                    bin_dir.join(file_name).with_extension("bin")
                } else {
                    // fallback : place le .bin à côté du .ron
                    let file_name = path.file_stem().unwrap_or_default();
                    path.with_file_name(file_name).with_extension("bin")
                };
                match ensure_bin_up_to_date(&path, &bin_path, |r, b| {
                    ron_to_bin::<LemmaEntries>(r, b)
                }) {
                    Ok(true) => println!("Conversion réussie: {:?} -> {:?}", path, bin_path),
                    Ok(false) => println!("Aucun changement: {:?} -> {:?}", path, bin_path),
                    Err(e) => {
                        eprintln!("Erreur de conversion: {:?} -> {:?} : {}", path, bin_path, e)
                    }
                }
            }
        }
    }
}
