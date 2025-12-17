//! Composants réutilisables au niveau application.
//!
//! Ce module contient les composants partagés utilisés dans plusieurs pages :
//! - [`Navigation`] : Barre de navigation principale
//! - [`Footer`] : Pied de page de l'application

pub mod footer;
pub mod navigation;

pub use footer::Footer;
pub use navigation::Navigation;
