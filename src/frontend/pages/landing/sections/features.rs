//! Section des fonctionnalités de la landing page.
//!
//! Présente les principales fonctionnalités de la plateforme.

use crate::frontend::design_system::{
    Badge, BadgeVariant, Card, CardBody, CardHeader, Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

/// Section des fonctionnalités de la landing page.
///
/// Affiche les principales fonctionnalités de la plateforme
/// (pilotage, analytics, automatisation) avec leurs avantages.
#[component]
pub fn FeaturesSection() -> impl IntoView {
    struct Feature<'a> {
        badge: &'a str,
        title: &'a str,
        description: &'a str,
        items: &'a [&'a str],
    }

    let features = [
        Feature {
            badge: "Pilotage",
            title: "Gérez toutes vos marques depuis un seul espace.",
            description:
                "Déployez des espaces de travail segmentés, attribuez les droits adéquats et gardez une gouvernance claire sur vos campagnes.",
            items: &["Espaces collaboratifs", "Rôles & permissions", "Expirations programmées"],
        },
        Feature {
            badge: "Automatisation",
            title: "Raccourcissez à la volée, dans vos outils existants.",
            description:
                "Connectez shrtnr à votre CRM, CMS ou workflow no-code. Publiez des liens courts enrichis de paramètres sans quitter vos outils favoris.",
            items: &["Intégrations Slack, Notion, HubSpot", "Webhook & API REST", "Templates de campagne"],
        },
        Feature {
            badge: "Analytics",
            title: "Attribuez chaque clic à la bonne initiative.",
            description:
                "Croisez les performances de vos campagnes avec vos objectifs business et partagez des tableaux de bord prêts pour le comité de pilotage.",
            items: &["KPI temps réel", "Rapports exportables", "Alertes intelligentes"],
        },
    ];

    view! {
        <section class="grid gap-8" id="features">
            <div class="flex flex-col gap-3">
                <Badge variant=BadgeVariant::Subtle class="w-fit">"Plateforme complète"</Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Tout ce dont vous avez besoin pour construire des expériences de liens cohérentes."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-3xl">
                    "shrtnr s'adapte à votre stack marketing et fournit les garde-fous nécessaires aux équipes distribuées."
                </Text>
            </div>
            <div class="grid gap-6 lg:grid-cols-3">
                {features
                    .into_iter()
                    .map(|feature| {
                        view! {
                            <Card class="flex h-full flex-col bg-surface">
                                <CardHeader class="gap-3">
                                    <Badge variant=BadgeVariant::Outline class="w-fit">
                                        {feature.badge}
                                    </Badge>
                                    <Heading level=HeadingLevel::H3 class="text-lg text-foreground">
                                        {feature.title}
                                    </Heading>
                                </CardHeader>
                                <CardBody class="flex flex-1 flex-col gap-4 text-sm text-foreground/80">
                                    <Text tone=TextTone::Subtle>{feature.description}</Text>
                                    <ul class="grid gap-3 text-sm text-foreground">
                                        {feature
                                            .items
                                            .iter()
                                            .map(|item| {
                                                view! {
                                                    <li class="flex items-start gap-2">
                                                        <span class="mt-1 inline-flex h-2.5 w-2.5 flex-none rounded-full bg-brand"></span>
                                                        <span>{item.to_string()}</span>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </CardBody>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
