//! Système de design de l'application.
//!
//! Ce module fournit un ensemble cohérent de composants de base réutilisables
//! pour construire l'interface utilisateur. Tous les composants suivent
//! les mêmes conventions de style et d'accessibilité.
//!
//! ## Composants disponibles
//!
//! - [`Button`] : Boutons avec différentes variantes et tailles
//! - [`InputField`] : Champs de formulaire avec validation
//! - [`Card`] : Conteneurs de contenu avec header, body, footer
//! - [`Badge`] : Badges pour afficher des statuts ou labels
//! - [`Chip`] : Chips pour les tags et filtres
//! - [`Heading`] : Titres avec différents niveaux (H1-H6)
//! - [`Text`] : Texte avec différents tons et tailles

pub mod components;

pub use components::{
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, CardBody, CardFooter, CardHeader},
    chip::Chip,
    heading::{Heading, HeadingLevel},
    input::{FormControl, InputField},
    text::{Text, TextTone},
};
