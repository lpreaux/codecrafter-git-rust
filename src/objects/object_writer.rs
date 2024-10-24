use crate::objects::object_base::GitObject;
use crate::objects::object_kind::GitObjectKind;
use crate::objects::tree::tree::Tree;
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};

pub struct GitObjectWriter;

impl GitObjectWriter {
    pub fn write_object(&self, object: &GitObjectKind) -> Result<()> {
        match object {
            GitObjectKind::Blob(blob) => self.write_individual_object(blob),
            GitObjectKind::Tree(tree) => self.write_tree(tree),
            GitObjectKind::Commit(commit) => self.write_individual_object(commit),
        }
    }

    fn write_individual_object<T: GitObject>(&self, object: &T) -> Result<()> {
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

    fn write_tree(&self, tree: &Tree) -> Result<()> {
        for entry in &tree.entries {
            if let Some(object) = &entry.object {
                match object {
                    GitObjectKind::Blob(blob) => self.write_individual_object(blob)?,
                    GitObjectKind::Tree(sub_tree) => self.write_tree(sub_tree)?,
                    GitObjectKind::Commit(commit) => self.write_individual_object(commit)?,
                }
            }
        }

        // Sauvegarde du Tree lui-même
        self.write_individual_object(tree)
    }
}
