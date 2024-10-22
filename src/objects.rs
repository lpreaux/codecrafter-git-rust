use anyhow::{anyhow, Result};
use flate2::read::ZlibDecoder;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
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
/// - `_file_path`: Chemin vers le fichier de l'objet sur le disque (champ privé).
/// - `size`: Taille du contenu du blob en octets.
/// - `content`: Contenu du blob, représenté sous forme de chaîne de caractères.
#[derive(Debug)]
pub struct Blob {
    pub hash: String,
    _file_path: PathBuf,
    pub size: usize,
    pub content: String,
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
        write_object(&object)?;
    }

    // Retourne le hash du blob
    Ok(object.hash)
}

/// Écrit un objet Git sur le disque avec compression Zlib.
///
/// # Paramètres
/// - `object`: L'objet Blob à écrire.
///
/// # Renvoie
/// - `Result` indiquant le succès ou l'échec de l'opération.
fn write_object(object: &Blob) -> Result<()> {
    // S'assure que le répertoire parent du fichier blob existe
    if let Some(parent_dir) = object._file_path.parent() {
        create_dir_all(parent_dir)?;
    }

    // Crée un fichier pour écrire l'objet
    let writer = BufWriter::new(File::create(&object._file_path)?);

    // Encode les données du blob en utilisant Zlib
    let mut encoder = ZlibEncoder::new(writer, Compression::default());

    // Génère les données formatées du blob
    let blob_data = get_blob_data(&object.size, &object.content);

    // Écrit les données compressées dans le fichier
    encoder.write_all(blob_data.as_bytes())?;

    Ok(())
}


impl TryFrom<&str> for Blob {
    type Error = anyhow::Error;

    /// Convertit un identifiant de hash Git en un objet Blob.
    ///
    /// # Paramètres
    /// - `hash`: Le hash de l'objet à convertir.
    ///
    /// # Renvoie
    /// - Un objet `Blob` si la conversion réussit, sinon une erreur.
    ///
    /// # Erreurs
    /// - Retourne une erreur si le fichier blob ne peut pas être lu ou s'il est mal formaté.
    fn try_from(hash: &str) -> Result<Self> {
        // Génère le chemin d'accès au fichier blob correspondant au hash
        let path = hash_to_object_path(hash)?;

        // Ouvre le fichier blob
        let file = File::open(&path).map_err(|_| anyhow!("Failed to open blob data file at {:?}", path))?;
        let reader = BufReader::new(file);

        // Décompresse et lit les données Zlib du fichier
        let mut decoder = ZlibDecoder::new(reader);
        let mut blob_data = String::new();

        // Lit le contenu du fichier dans blob_data
        decoder.read_to_string(&mut blob_data).map_err(|e| {
            anyhow!("Failed to read and decode the blob data from {:?}: {}", path, e)
        })?;

        // Vérifie que le préfixe "blob " est présent
        if blob_data.len() < BLOB_HEADER_PREFIX.len() || !blob_data.starts_with(BLOB_HEADER_PREFIX) {
            return Err(anyhow!("Invalid blob object format: expected prefix '{}', but it was not found", BLOB_HEADER_PREFIX));
        }

        // Sépare les données en en-tête (taille) et contenu
        let stripped_blob_data = &blob_data[BLOB_HEADER_PREFIX.len()..];
        let mut parts = stripped_blob_data.splitn(2, BLOB_CONTENT_SEPARATOR);
        let size_header = parts.next().ok_or_else(|| anyhow!("Expected size header missing"))?;
        let blob_content = parts.next().ok_or_else(|| anyhow!("Expected blob content missing"))?;

        // Parse la taille du blob
        let size: usize = parse_blob_size(size_header)?;

        // Retourne un objet Blob avec les informations lues
        Ok(Blob {
            hash: String::from(hash),
            _file_path: path,
            size,
            content: blob_content.to_string(),
        })
    }
}

impl TryFrom<&PathBuf> for Blob {
    type Error = anyhow::Error;

    /// Crée un objet Blob à partir d'un chemin de fichier.
    ///
    /// # Paramètres
    /// - `path`: Le chemin du fichier à convertir en blob.
    ///
    /// # Renvoie
    /// - Un objet `Blob` avec les données extraites du fichier.
    fn try_from(path: &PathBuf) -> Result<Self> {
        // Ouvre le fichier spécifié
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut file_data = Vec::new();

        // Lit le contenu du fichier
        reader.read_to_end(&mut file_data)?;

        // Calcule la taille des données et les convertit en chaîne UTF-8
        let size = file_data.len();
        let content = String::from_utf8(file_data).map_err(|e| anyhow!("Invalid UTF-8 in object file: {}", e))?;

        // Crée la chaîne de données du blob
        let blob_data = get_blob_data(&size, &content);

        // Calcule le hash SHA-1 des données du blob
        let hash = compute_sha1(&blob_data);
        let path = hash_to_object_path(&hash)?;

        // Retourne un objet Blob
        Ok(Blob {
            hash,
            _file_path: path,
            size,
            content,
        })
    }
}

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
fn hash_to_object_path(hash: &str) -> Result<PathBuf> {
    if hash.len() != OBJECT_HASH_SIZE {
        return Err(anyhow!("Invalid object identifier: expected {} characters, got {}", OBJECT_HASH_SIZE, hash.len()));
    }

    // Sépare le hash en deux parties : le répertoire (les deux premiers caractères) et le fichier
    let (dir, file) = hash.split_at(2);
    Ok(PathBuf::from(GIT_OBJECTS_DIR).join(dir).join(file))
}

/// Parse une chaîne de taille de blob en un entier `usize`.
///
/// # Paramètres
/// - `size_str`: Chaîne représentant la taille.
///
/// # Renvoie
/// - La taille du blob en tant qu'entier.
///
/// # Erreurs
/// - Retourne une erreur si la chaîne ne peut pas être convertie en `usize`.
fn parse_blob_size(size_str: &str) -> Result<usize> {
    size_str.parse().map_err(|_| anyhow!("Invalid size format in blob object"))
}

/// Génère la chaîne de données blob avec un en-tête Git blob.
///
/// # Paramètres
/// - `size`: Taille du contenu.
/// - `content`: Le contenu des données du blob.
///
/// # Renvoie
/// - Une chaîne formatée qui inclut l'en-tête blob et le contenu séparé par un caractère nul.
fn get_blob_data(size: &usize, content: &str) -> String {
    format!("{}{}{}{}", BLOB_HEADER_PREFIX, size, BLOB_CONTENT_SEPARATOR, content)
}

/// Calcule le hash SHA-1 pour les données spécifiées.
///
/// # Paramètres
/// - `data`: Les données à hacher.
///
/// # Renvoie
/// - Un hash SHA-1 sous forme de chaîne hexadécimale.
fn compute_sha1(data: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data.as_bytes());
    hex::encode(hasher.finalize())
}
