use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use crate::objects::object_base::{GitObject, GitObjectBase};
use crate::objects::object_kind::GitObjectKind;
use crate::objects::{object_manager, utils};
use anyhow::{anyhow, Result};

pub(crate) const HEADER_PREFIX: &str = "tree";

#[derive(Debug)]
pub struct Tree {
    pub base: GitObjectBase,
    pub entries: Vec<TreeEntry>,  // Spécifique aux arbres
}
#[derive(Debug)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub hash: String,
    pub object: Option<GitObjectKind>,
}

impl Tree {
    pub(crate) fn new(mut entries: Vec<TreeEntry>) -> Result<Self> {
        // Trier les entrées par ordre alphabétique du nom
        entries.sort_by_key(|entry| entry.name.clone());

        // Créer un buffer pour stocker les données de l'objet tree
        let mut entries_data = Vec::new();

        for entry in entries.iter() {
            // Ajouter le mode, suivi d'un espace
            entries_data.extend_from_slice(entry.mode.as_bytes());
            entries_data.push(b' ');

            // Ajouter le nom du fichier, suivi d'un byte nul
            entries_data.extend_from_slice(entry.name.as_bytes());
            entries_data.push(b'\0');

            // Ajouter le hash décodé (sous forme de bytes)
            let hash_bytes = hex::decode(&entry.hash).expect("Invalid hash format");
            entries_data.extend_from_slice(&hash_bytes);
        }

        // Taille du contenu de l'objet tree
        let size = entries_data.len();

        // Créer l'en-tête du blob avec le préfixe "tree" et la taille
        let mut blob_data = Vec::new();
        blob_data.extend_from_slice(format!("tree {}\0", size).as_bytes());

        // Ajouter les données des entrées
        blob_data.extend_from_slice(&entries_data);

        // Calculer le hash SHA-1 pour les données complètes de l'objet tree
        let hash = utils::compute_sha1_from_bytes(&blob_data);

        // Retourner l'objet Tree avec son hash
        Ok(Tree {
            base: GitObjectBase {
                hash,
            },
            entries,
        })
    }

}

impl GitObject for Tree {
    fn get_hash(&self) -> &str {
        &self.base.hash
    }

    fn get_header_prefix(&self) -> &'static str {
        HEADER_PREFIX
    }

    fn compute_size(&self) -> usize {
        self.entries.iter().map(|entry| {
            entry.mode.as_bytes().len()          // Taille du mode (en ASCII)
                + 1                       // Espace séparateur entre le mode et le nom
                + entry.name.as_bytes().len()        // Taille du nom
                + 1                       // Séparateur NUL '\0'
                + 20                      // Taille du hash en binaire (SHA-1 est 20 octets)
        }).sum::<usize>()
    }

    fn compute_object_data(&self) -> Vec<u8> {
        let header_str = self.get_header();
        let header = header_str.as_bytes();
        let mut path = PathBuf::new();
        path.push("tmp");
        path.push(&self.base.hash);
        if let Some(parent_dir) = path.parent() {
            create_dir_all(parent_dir).unwrap();
        }
        let mut writer = BufWriter::new(File::create(path).unwrap());
        writer.write(self.compute_size().to_string().as_bytes()).expect("TODO: panic message");
        let mut result = Vec::with_capacity(header.len() + self.compute_size());

        result.extend_from_slice(&header);

        // Ajouter chaque entrée du tree
        self.entries.iter().for_each(|entry| {
            // Ajouter le mode (comme "100644" ou "040000")
            result.extend_from_slice(&entry.mode.as_bytes());
            // Ajouter un espace séparateur
            result.push(b' ');

            // Ajouter le nom du fichier ou du répertoire
            result.extend_from_slice(entry.name.as_bytes());

            // Ajouter le séparateur NUL '\0'
            result.push(0);

            // Ajouter le hash (20 octets binaires, pas en hexadécimal)
            result.extend_from_slice(&hex::decode(&entry.hash).expect("Invalid hash format"));
        });

        result
    }

    fn from_object_file(hash: &str, content: &[u8]) -> Result<Tree> {
        let mut entries = Vec::new();
        let mut idx = 0;

        while idx < content.len() {
            // Étape 1 : Lire le mode (jusqu'à trouver un espace ' ')
            let mode_end = content[idx..].iter()
                .position(|&c| c == b' ')
                .ok_or_else(|| anyhow!("Invalid tree object: missing space after mode"))?;
            let mode = std::str::from_utf8(&content[idx..idx + mode_end])?.to_string();
            idx += mode_end + 1; // Passer l'espace

            // Étape 2 : Lire le nom (jusqu'à trouver un séparateur NUL '\0')
            let name_end = content[idx..].iter()
                .position(|&c| c == object_manager::OBJECT_CONTENT_SEPARATOR)
                .ok_or_else(|| anyhow!("Invalid tree object: missing NUL after name"))?;
            let name = std::str::from_utf8(&content[idx..idx + name_end])?.to_string();
            idx += name_end+1; // Passer le séparateur NUL

            // Étape 3 : Lire le hash (20 octets binaires)
            if idx + 20 > content.len() {
                return Err(anyhow!("Invalid tree object: insufficient bytes for hash"));
            }
            let hash = hex::encode(&content[idx..idx + 20]);
            idx += 20; // Avancer de 20 octets

            // Ajouter l'entrée au tableau d'entrées
            entries.push(TreeEntry {
                mode,
                name,
                hash,
                object: None,
            });
        }

        Ok(Tree {
            base: GitObjectBase {
                hash: hash.to_string(),
            },
            entries,
        })
    }
}