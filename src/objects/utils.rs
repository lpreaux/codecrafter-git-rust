use std::path::PathBuf;
use anyhow::anyhow;
use sha1::{Digest, Sha1};
use crate::objects::object_manager;

/// Convertit un hash Git en un chemin vers le fichier objet dans `.git/objects`.
///
/// # Paramètres
/// - `hash`: L'identifiant SHA-1 de l'objet.
///
/// # Renvoie
/// - Un `PathBuf` pointant vers l'emplacement du fichier blob.
///
/// # Erreurs
/// - Retourne une erreur si le hash est de longueur incorrecte.
pub(crate) fn hash_to_object_path(hash: &str) -> anyhow::Result<PathBuf> {
    if hash.len() != object_manager::OBJECT_HASH_SIZE {
        return Err(anyhow!("Invalid object identifier: expected {} characters, got {}", object_manager::OBJECT_HASH_SIZE, hash.len()));
    }

    // Sépare le hash en deux parties : le répertoire (les deux premiers caractères) et le fichier
    let (dir, file) = hash.split_at(2);
    Ok(PathBuf::from(object_manager::GIT_OBJECTS_DIR).join(dir).join(file))
}

/// Calcule le hash SHA-1 pour les données spécifiées.
///
/// # Paramètres
/// - `data`: Les données à hacher.
///
/// # Renvoie
/// - Un hash SHA-1 sous forme de chaîne hexadécimale.
pub(crate) fn compute_sha1(data: &str) -> String {
    compute_sha1_from_bytes(data.as_bytes())
}

pub(crate) fn compute_sha1_from_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
