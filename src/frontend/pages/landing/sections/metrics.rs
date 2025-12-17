//! Section des métriques de la landing page.
//!
//! Affiche les statistiques et métriques impressionnantes de la plateforme.

use crate::frontend::design_system::{Card, CardBody, Heading, HeadingLevel, Text, TextTone};
use leptos::prelude::*;

/// Section des métriques de la landing page.
///
/// Affiche des statistiques clés (nombre de liens, CTR, performance)
/// pour démontrer la valeur de la plateforme.
#[component]
pub fn MetricsSection() -> impl IntoView {
    let metrics = [
        (
            "2,3M",
            "Liens suivis chaque mois",
            "Agrégés en temps réel depuis vos CMS, campagnes et automatisations.",
        ),
        (
            "38%",
            "CTR moyen constaté",
            "Optimisez vos campagnes grâce aux tests A/B et aux redirections intelligentes.",
        ),
        (
            "< 200ms",
            "Temps de résolution",
            "Infrastructure edge distribuée sur 120 PoP pour une expérience instantanée.",
        ),
    ];

    view! {
        <section class="grid gap-8">
            <div class="flex flex-col gap-3">
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Des performances qui se traduisent en conversions mesurables."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-3xl">
                    "Analysez chaque campagne avec des métriques granulaires et partagez des rapports exploitables avec vos équipes."
                </Text>
            </div>
            <div class="grid gap-6 md:grid-cols-3">
                {metrics
                    .into_iter()
                    .map(|(value, title, description)| {
                        view! {
                            <Card class="bg-surface/80">
                                <CardBody class="flex min-h-[200px] flex-col gap-4">
                                    <Heading level=HeadingLevel::H2 class="text-4xl font-semibold text-brand">
                                        {value}
                                    </Heading>
                                    <Heading level=HeadingLevel::H4 class="text-foreground">
                                        {title}
                                    </Heading>
                                    <Text tone=TextTone::Subtle>{description}</Text>
                                </CardBody>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
