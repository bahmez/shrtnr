//! Sections de la page d'accueil.
//!
//! Ce module contient toutes les sections composant la landing page :
//! - [`HeroSection`] : Section principale avec titre et CTA
//! - [`MetricsSection`] : Métriques et statistiques
//! - [`FeaturesSection`] : Présentation des fonctionnalités
//! - [`WorkflowSection`] : Workflow d'utilisation
//! - [`PricingSection`] : Tarifs et plans
//! - [`TestimonialsSection`] : Témoignages clients
//! - [`CtaSection`] : Call-to-action final

pub mod cta;
pub mod features;
pub mod hero;
pub mod metrics;
pub mod pricing;
pub mod testimonials;
pub mod workflow;

pub use cta::CtaSection;
pub use features::FeaturesSection;
pub use hero::HeroSection;
pub use metrics::MetricsSection;
pub use pricing::PricingSection;
pub use testimonials::TestimonialsSection;
pub use workflow::WorkflowSection;
