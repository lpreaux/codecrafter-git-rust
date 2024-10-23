use crate::objects::blob::blob::Blob;
use crate::objects::object_writer::GitObjectWriter;
use anyhow::Result;
use std::path::Path;
use crate::objects::object_kind::GitObjectKind;
use crate::objects::objet_reader::GitObjectReader;

pub(crate) const OBJECT_CONTENT_SEPARATOR: u8 = 0;
pub(crate) const OBJECT_HASH_SIZE: usize = 40;
pub(crate) const GIT_OBJECTS_DIR: &str = ".git/objects";

pub fn read_object(hash: &str) -> Result<GitObjectKind> {
    let reader = GitObjectReader;
    reader.read_object(hash)
}

/// Calcule et retourne le hash d'un fichier et, si nécessaire, écrit l'objet Git sur le disque.
///
/// # Paramètres
/// - `path`: Le chemin du fichier.
/// - `write_mode`: Indique si l'objet doit être écrit sur le disque.
///
/// # Renvoie
/// - Le hash SHA-1 de l'objet Git.
pub fn file_to_hash(path: &Path, write_mode: bool) -> Result<String> {
    // Crée un objet Blob à partir du chemin du fichier
    let object: Blob = path.try_into()?;

    // Si `write_mode` est activé, écrit l'objet sur le disque
    if write_mode {
        GitObjectWriter::write_object(&object)?;
    }

    // Retourne le hash du blob
    Ok(object.base.hash)
}





