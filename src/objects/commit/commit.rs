use crate::objects::object_base::{GitObject, GitObjectBase};
use crate::objects::tree::tree;
use crate::objects::utils;
use anyhow::{anyhow, Result};

pub(crate) const HEADER_PREFIX: &str = "commit";
const PARENT_PREFIX: &str = "parent";
const AUTHOR_PREFIX: &str = "author";

#[derive(Debug)]
pub struct Commit {
    pub base: GitObjectBase,
    pub tree_hash: String,
    pub parent_hash: Option<String>,
    pub message: String,
    pub author: CommitAuthor,
}

#[derive(Debug)]
pub struct CommitAuthor {
    name: String,
    email: String,
}

impl Commit {
    pub(crate) fn new(tree_hash: &String, parent_hash: &Option<String>, message: &String, author: Option<CommitAuthor>) -> Result<Commit> {
        let author = author.unwrap_or_else(|| CommitAuthor {
            name: "unpseudo".to_string(),
            email: "unpseudo@mail.mail".to_string(),
        });

        // Créer un buffer pour stocker les données de l'objet commit
        let mut content = Vec::new();

        content.extend_from_slice(&tree::HEADER_PREFIX.as_bytes());
        content.push(b' ');
        content.extend_from_slice(tree_hash.as_bytes());
        content.push(b'\n');

        if let Some(parent_hash) = parent_hash {
            content.extend_from_slice(&PARENT_PREFIX.as_bytes());
            content.push(b' ');
            content.extend_from_slice(parent_hash.as_bytes());
            content.push(b'\n');
        }

        content.extend_from_slice(&AUTHOR_PREFIX.as_bytes());
        content.push(b' ');
        content.extend_from_slice(author.name.as_bytes());
        content.push(b' ');
        content.extend_from_slice(format!("<{}>", author.email).as_bytes());
        content.push(b'\n');

        content.push(b'\n');
        content.extend_from_slice(message.as_bytes());
        content.push(b'\n');

        // Taille du contenu de l'objet tree
        let size = content.len();

        // Créer l'en-tête du blob avec le préfixe "tree" et la taille
        let mut blob_data = Vec::new();
        blob_data.extend_from_slice(format!("tree {}\0", size).as_bytes());

        // Ajouter les données des entrées
        blob_data.extend_from_slice(&content);

        // Calculer le hash SHA-1 pour les données complètes de l'objet tree
        let hash = utils::compute_sha1_from_bytes(&blob_data);

        Ok(Commit {
            base: GitObjectBase { hash },
            tree_hash: tree_hash.to_owned(),
            parent_hash: parent_hash.to_owned(),
            message: message.to_owned(),
            author,
        })
    }
}

impl Commit {}

impl GitObject for Commit {
    fn get_hash(&self) -> &str {
        &self.base.hash
    }

    fn get_header_prefix(&self) -> &'static str {
        HEADER_PREFIX
    }

    fn compute_size(&self) -> usize {
        format!("{} ", tree::HEADER_PREFIX).len()       // Size of tree prefix + ' ' separator
            + 10                                        // Size of tree hash
            + 1                                         // Size of '\n'
            + if self.parent_hash.is_some() {
            format!("{} ", PARENT_PREFIX).len()     // Size of parent commit prefix + ' ' separator
                + 10                                // Size of parent commit hash
                + 1                                  // Size of '\n'
        } else { 0 }
            + format!("{} ", AUTHOR_PREFIX).len()       // Size of author prefix + ' ' separator
            + self.author.name.len()                    // Size of author name
            + format!(" <{}>", self.author.email).len() // Size of author email with separators
            + 1                                          // Size of '\n'
            + 1                                          // Size of '\n' for the blank line between commit infos and commit message
            + self.message.len()
    }

    fn compute_object_data(&self) -> Vec<u8> {
        let header_str = self.get_header();
        let header = header_str.as_bytes();

        let mut result = Vec::with_capacity(header.len() + self.compute_size());

        result.extend_from_slice(&header);

        result.extend_from_slice(&tree::HEADER_PREFIX.as_bytes());
        result.push(b' ');
        result.extend_from_slice(&self.tree_hash.as_bytes());
        result.push(b'\n');

        if let Some(parent_hash) = &self.parent_hash {
            result.extend_from_slice(&PARENT_PREFIX.as_bytes());
            result.push(b' ');
            result.extend_from_slice(parent_hash.as_bytes());
            result.push(b'\n');
        }

        result.extend_from_slice(&AUTHOR_PREFIX.as_bytes());
        result.push(b' ');
        result.extend_from_slice(&self.author.name.as_bytes());
        result.push(b' ');
        result.extend_from_slice(format!("<{}>", &self.author.email).as_bytes());
        result.push(b'\n');

        result.push(b'\n');
        result.extend_from_slice(self.message.as_bytes());
        result.push(b'\n');

        result
    }

    fn from_object_file(hash: &str, content: &[u8]) -> Result<Self>
    where
        Self: Sized,
    {
        let mut tree_hash = String::new();
        let mut parent_hash: Option<String> = None;
        let mut message = String::new();
        let mut author = CommitAuthor {
            name: String::new(),
            email: String::new(),
        };
        let mut idx = 0;

        while idx < content.len() {
            let line_end = content[idx..].iter()
                .position(|&c| c == b'\n')
                .ok_or_else(|| anyhow!("Invalid commit object: missing line end"))?;

            let line = std::str::from_utf8(&content[idx..idx + line_end])?;

            if line.is_empty() {
                // Passage aux données du message après la première ligne vide
                idx += line_end + 1;
                message = std::str::from_utf8(&content[idx..])?.to_owned();
                break;
            } else {
                let parts: Vec<&str> = line.split_whitespace().collect();
                match parts.as_slice() {
                    // Traitement des différentes parties d'un commit Git
                    [tree::HEADER_PREFIX, tree_hash_value] => tree_hash = tree_hash_value.to_string(),
                    [PARENT_PREFIX, parent_hash_value] => parent_hash = Some(parent_hash_value.to_string()),
                    [AUTHOR_PREFIX, author_name, author_email, ..] => {
                        author.name = author_name.to_string();
                        author.email = author_email.to_string();
                    }
                    _ => return Err(anyhow!("Unknown commit object field: {}", line)),
                }
                idx += line_end + 1; // Avancer à la ligne suivante
            }
        }

        Ok(Commit {
            base: GitObjectBase {
                hash: hash.to_string(),
            },
            tree_hash,
            parent_hash,
            message,
            author,
        })
    }
}