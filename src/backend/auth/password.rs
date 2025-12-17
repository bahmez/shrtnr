//! Fonctions de hashage et vérification des mots de passe.
//!
//! Ce module utilise bcrypt pour sécuriser les mots de passe des utilisateurs.
//! Les mots de passe sont hashés avant stockage et vérifiés lors de la connexion.

use bcrypt::{hash, verify, DEFAULT_COST};

/// Hash un mot de passe en clair avec bcrypt.
///
/// Utilise le coût par défaut de bcrypt (12 rounds) pour équilibrer
/// sécurité et performance.
///
/// # Arguments
///
/// * `password` - Le mot de passe en clair à hasher
///
/// # Returns
///
/// * `Ok(String)` - Le hash bcrypt du mot de passe
/// * `Err(BcryptError)` - En cas d'erreur lors du hashage
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::backend::auth::hash_password;
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let password = "mySecurePassword123";
/// let hash = hash_password(password)?;
/// println!("Hashed password: {}", hash);
/// # Ok(())
/// # }
/// ```
pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

/// Vérifie qu'un mot de passe en clair correspond à un hash bcrypt.
///
/// Compare le mot de passe fourni avec le hash stocké en base de données.
///
/// # Arguments
///
/// * `password` - Le mot de passe en clair à vérifier
/// * `hash` - Le hash bcrypt stocké en base de données
///
/// # Returns
///
/// * `Ok(true)` - Si le mot de passe correspond au hash
/// * `Ok(false)` - Si le mot de passe ne correspond pas
/// * `Err(BcryptError)` - En cas d'erreur lors de la vérification
///
/// # Exemple
///
/// ```rust,no_run
/// use shrtnr::backend::auth::{hash_password, verify_password};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let password = "mySecurePassword123";
/// let hash = hash_password(password)?;
///
/// // Vérification réussie
/// assert!(verify_password(password, &hash)?);
///
/// // Vérification échouée
/// assert!(!verify_password("wrongPassword", &hash)?);
/// # Ok(())
/// # }
/// ```
pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
