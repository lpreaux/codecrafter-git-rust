use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use crate::objects::object_base::GitObject;
use anyhow::Result;
use flate2::Compression;
use flate2::write::ZlibEncoder;

pub struct GitObjectWriter;

impl GitObjectWriter {
    pub fn write_object<T: GitObject>(object: &T) -> Result<()> {
        let object_data = object.compute_object_data();

        let path = object.compute_file_path()?;

        // Si l'objet existe déjà, ne pas réécrire
        if path.exists() {
            return Err(anyhow::anyhow!("Object already exists with hash: {}", object.get_hash()));
        }

        // Créez les répertoires si nécessaire
        if let Some(parent_dir) = path.parent() {
            create_dir_all(parent_dir)?;
        }

        // Ouvrez un fichier en mode écriture dans .git/objects/ et utilisez la compression Zlib
        let writer = BufWriter::new(File::create(&path)?);
        let mut encoder = ZlibEncoder::new(writer, Compression::default());

        // Écrivez les données compressées dans le fichier
        encoder.write_all(&object_data)?;
        encoder.finish()?;

        Ok(())
    }
}