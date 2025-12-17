//! Module d'authentification et de gestion des utilisateurs.
//!
//! Ce module fournit :
//! - Les handlers API pour l'inscription, connexion, et gestion de profil
//! - La génération et validation des tokens JWT
//! - Le hashage et la vérification des mots de passe avec bcrypt
//!
//! ## Architecture
//!
//! Le module est organisé en plusieurs sous-modules :
//! - [`handlers`] : Handlers Axum pour les endpoints API
//! - [`jwt`] : Gestion des tokens JWT (création, validation, refresh)
//! - [`password`] : Hashage et vérification des mots de passe
//!
//! ## Sécurité
//!
//! - Les mots de passe sont hashés avec bcrypt (cost factor: 12)
//! - Les tokens JWT utilisent HS256 avec un secret configurable
//! - Les tokens d'accès expirent après 15 minutes
//! - Les tokens de refresh expirent après 7 jours

pub mod handlers;
pub mod jwt;
pub mod password;

pub use jwt::{create_token_pair, verify_access_token, verify_refresh_token, Claims, TokenPair};
pub use password::{hash_password, verify_password};
