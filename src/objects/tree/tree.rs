use crate::objects::object_base::{GitObject, GitObjectBase};
use crate::objects::objects;
use anyhow::{anyhow, Result};

pub(crate) const HEADER_PREFIX: &str = "tree";

pub struct Tree {
    pub base: GitObjectBase,
    pub entries: Vec<TreeEntry>,  // Spécifique aux arbres
}

pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub hash: String,
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
            entry.mode.len()          // Taille du mode (en ASCII)
                + 1                       // Espace séparateur entre le mode et le nom
                + entry.name.len()        // Taille du nom
                + 1                       // Séparateur NUL '\0'
                + 20                      // Taille du hash en binaire (SHA-1 est 20 octets)
        }).sum()
    }

    fn compute_object_data(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.get_header().len() + 1 + self.compute_size());

        result.extend_from_slice(&self.get_header());

        result.push(objects::OBJECT_CONTENT_SEPARATOR);

        // Ajouter chaque entrée du tree
        for entry in &self.entries {
            // Ajouter le mode (comme "100644" ou "040000")
            result.extend_from_slice(entry.mode.as_bytes());

            // Ajouter un espace séparateur
            result.push(b' ');

            // Ajouter le nom du fichier ou du répertoire
            result.extend_from_slice(entry.name.as_bytes());

            // Ajouter le séparateur NUL '\0'
            result.push(objects::OBJECT_CONTENT_SEPARATOR);

            // Ajouter le hash (20 octets binaires, pas en hexadécimal)
            result.extend_from_slice(&hex::decode(&entry.hash).expect("Invalid hash format"));
        }

        result
    }

    fn from_data(hash: &str, content: &[u8]) -> Result<Tree> {
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
                .position(|&c| c == objects::OBJECT_CONTENT_SEPARATOR)
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