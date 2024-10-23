use crate::objects::blob::blob;
use crate::objects::blob::blob::Blob;
use crate::objects::object_base::GitObjectBase;
use crate::objects::{objects, utils};
use anyhow::anyhow;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

impl TryFrom<&Path> for Blob {
    type Error = anyhow::Error;

    /// Crée un objet Blob à partir d'un chemin de fichier.
    ///
    /// # Paramètres
    /// - `path`: Le chemin du fichier à convertir en blob.
    ///
    /// # Renvoie
    /// - Un objet `Blob` avec les données extraites du fichier.
    fn try_from(path: &Path) -> anyhow::Result<Self> {
        // Ouvre le fichier spécifié
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut file_data = Vec::new();

        // Lit le contenu du fichier
        reader.read_to_end(&mut file_data)?;

        let size = file_data.len();
        let content = String::from_utf8(file_data).map_err(|e| anyhow!("Invalid UTF-8 in object file: {}", e))?;
        let blob_data = format!("{} {}{}{}", blob::HEADER_PREFIX, size, "\0", content);
        let hash = utils::compute_sha1(&blob_data);
        Ok(Blob {
            base: GitObjectBase {
                hash,
            },
            content,
        })
    }
}