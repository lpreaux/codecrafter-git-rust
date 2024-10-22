use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

const BLOB_HEADER_PREFIX: &str = "blob ";
const BLOB_CONTENT_SEPARATOR: &str = "\0";

const OBJECT_HASH_SIZE: usize = 40;

const GIT_OBJECTS_DIR: &str = ".git/objects";

/// Représente un objet Blob dans un dépôt Git.
///
/// # Champs
/// - `hash`: Identifiant unique de l'objet (hash).
/// - `file_path`: Chemin vers le fichier de l'objet sur le disque.
/// - `size`: Taille du contenu du blob en octets.
/// - `content`: Contenu du blob, représenté sous forme de chaîne de caractères.
#[derive(Debug)]
pub struct Blob {
    pub hash: String,
    _file_path: PathBuf,
    pub size: usize,
    pub content: String,
}


impl TryFrom<&String> for Blob {
    type Error = anyhow::Error;

    /// Convertit un identifiant d'objet (hash) en un Blob.
    ///
    /// # Paramètres
    /// - `object_identifier`: Identifiant de l'objet à convertir.
    ///
    /// # Renvoie
    /// - Un `Blob` contenant les informations extraites du fichier.
    ///
    /// # Erreurs
    /// - Renvoie une erreur si le format du fichier blob est invalide.
    fn try_from(object_identifier: &String) -> Result<Self> {
        let path = hash_to_object_path(object_identifier)?;

        // Utilisation de BufReader pour une lecture potentiellement plus efficace
        let file = File::open(&path).map_err(|_| anyhow!("Failed to open blob data file at {:?}", path))?;
        let reader = BufReader::new(file);

        // Lire le contenu du fichier avec GzDecoder
        let mut decoder = ZlibDecoder::new(reader);
        let mut blob_data = String::new();

        // Lire le contenu et gérer les erreurs
        decoder.read_to_string(&mut blob_data).map_err(|e| {
            anyhow!("Failed to read and decode the blob data from {:?}: {}", path, e)
        })?;


        // Vérifier le préfixe
        if blob_data.len() < BLOB_HEADER_PREFIX.len() || !blob_data.starts_with(BLOB_HEADER_PREFIX) {
            return Err(anyhow!("Invalid blob object format: expected prefix '{}', but it was not found", BLOB_HEADER_PREFIX));
        }

        // Supprimer le préfixe et vérifier le séparateur
        let stripped_blob_data = &blob_data[BLOB_HEADER_PREFIX.len()..];
        let mut parts = stripped_blob_data.splitn(2, BLOB_CONTENT_SEPARATOR);
        let size_header = parts.next().ok_or_else(|| anyhow!("Expected size header missing"))?;
        let blob_content = parts.next().ok_or_else(|| anyhow!("Expected blob content missing"))?;

        // Convertir la taille en usize
        let size: usize = size_header.parse().map_err(|_| anyhow!("Invalid size format in blob object"))?;

        // Retourner la structure Blob
        Ok(Blob {
            hash: String::from(object_identifier),
            _file_path: path,
            size,
            content: blob_content.to_string(),
        })
    }
}

impl TryFrom<&PathBuf> for Blob {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut file_data = Vec::new();
        reader.read_to_end(&mut file_data)?;

        let size = file_data.len();
        let content = String::from_utf8(file_data).map_err(|_| anyhow!("Invalid UTF-8 in object file"))?;

        let blob_data = format!("{}{}{}{}", BLOB_HEADER_PREFIX, size, BLOB_CONTENT_SEPARATOR, content);

        let mut hasher = Sha1::new();
        hasher.update(blob_data.as_bytes());
        let unencoded_sha = hasher.finalize();
        let hash = hex::encode(unencoded_sha);
        let path = hash_to_object_path(&hash)?;

        Ok(Blob {
            hash,
            _file_path: path,
            size,
            content,
        })
    }
}

/// Convertit un identifiant d'objet (hash) en un chemin d'accès à l'objet dans le répertoire Git.
///
/// # Paramètres
/// - `hash`: Identifiant de l'objet à convertir.
///
/// # Renvoie
/// - Un `PathBuf` pointant vers le fichier de l'objet.
///
/// # Erreurs
/// - Renvoie une erreur si la longueur du hash n'est pas valide.
fn hash_to_object_path(hash: &String) -> Result<PathBuf> {
    // Vérifier si la longueur de l'identifiant est correcte
    if hash.len() != OBJECT_HASH_SIZE {
        return Err(anyhow!("Invalid object identifier: expected {} characters, got {}", OBJECT_HASH_SIZE, hash.len()));
    }

    // Extraire le répertoire et le nom de fichier à partir du hash
    let (dir, file) = hash.split_at(2);
    Ok(PathBuf::from(GIT_OBJECTS_DIR).join(dir).join(file))
}

pub fn file_to_hash(path: &PathBuf, write_mode: &bool) -> Result<String> {
    let object: Blob = path.try_into()?;
    
    if *write_mode {
       write_object(&object)?;
    }
    
    Ok(object.hash)
}

fn write_object(object: &Blob) -> Result<()> {
    let writer = BufWriter::new(File::create(&object._file_path)?);
    let mut encoder = ZlibEncoder::new(writer, Compression::default());
    let blob_data = format!("{}{}{}{}", BLOB_HEADER_PREFIX, object.size, BLOB_CONTENT_SEPARATOR, object.content);
    encoder.write_all(blob_data.as_bytes())?;
    Ok(())
}
