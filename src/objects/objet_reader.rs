use crate::objects::blob::blob::Blob;
use crate::objects::object_base::GitObject;
use crate::objects::object_kind::GitObjectKind;
use crate::objects::tree::tree::Tree;
use crate::objects::utils;
use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{BufReader, Read};

pub struct GitObjectReader;

impl GitObjectReader {
    pub fn read_object(&self, hash: &str) -> Result<GitObjectKind> {
        // Génère le chemin d'accès au fichier blob correspondant au hash
        let path = utils::hash_to_object_path(hash)?;

        // Ouvre le fichier de l'objet Git
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Décompresse les données Zlib
        let mut decoder = ZlibDecoder::new(reader);
        let mut object_data = Vec::new();
        decoder.read_to_end(&mut object_data)?;

        // La première partie des données est l'en-tête de l'objet : "<type> <size>\0"
        let (header, content) = self.split_header_and_content(&object_data)?;

        // Identifier le type de l'objet à partir de l'en-tête
        let object_kind = self.identify_object_type(&header)?;

        // Reconstruire l'objet en fonction de son type
        match object_kind.as_str() {
            "blob" => {
                // Crée un objet Blob à partir des données
                let blob = Blob::from_object_file(hash, content)?;
                Ok(GitObjectKind::Blob(blob))
            },
            "tree" => {
                // Crée un objet Tree à partir des données
                let tree = Tree::from_object_file(hash, content)?;
                Ok(GitObjectKind::Tree(tree))
            },
            _ => Err(anyhow!("Unsupported object type: {}", object_kind)),
        }
    }

    /// Sépare l'en-tête et le contenu des données d'un objet Git.
    fn split_header_and_content<'a>(&self, object_data: &'a [u8]) -> Result<(&'a str, &'a [u8])> {
        // Trouver l'index du premier '\0' pour séparer l'en-tête du contenu
        let null_byte_pos = object_data.iter().position(|&b| b == b'\0')
            .ok_or_else(|| anyhow!("Invalid object: missing header"))?;

        // Séparer l'en-tête du contenu
        let header = std::str::from_utf8(&object_data[..null_byte_pos])
            .map_err(|_| anyhow!("Invalid UTF-8 in object header"))?;
        let content = &object_data[null_byte_pos + 1..];

        Ok((header, content))
    }

    /// Identifie le type de l'objet Git à partir de l'en-tête.
    fn identify_object_type(&self, header: &str) -> Result<String> {
        // L'en-tête a la forme "<type> <size>"
        let mut parts = header.split_whitespace();
        let object_type = parts.next().ok_or_else(|| anyhow!("Invalid object header: missing type"))?;
        Ok(object_type.to_string())
    }
}