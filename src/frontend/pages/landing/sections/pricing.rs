//! Section tarifs de la landing page.
//!
//! Affiche les différents plans tarifaires disponibles.

use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardBody, CardFooter, CardHeader,
    Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

/// Section tarifs de la landing page.
///
/// Affiche les différents plans tarifaires (Growth, Scale, Enterprise)
/// avec leurs fonctionnalités et prix respectifs.
#[component]
pub fn PricingSection() -> impl IntoView {
    struct Plan<'a> {
        name: &'a str,
        price: &'a str,
        description: &'a str,
        features: &'a [&'a str],
        highlight: bool,
        cta: &'a str,
    }

    let plans = [
        Plan {
            name: "Growth",
            price: "49€",
            description: "Pour les équipes marketing qui veulent centraliser leurs liens et mesurer leurs résultats.",
            features: &[
                "5 espaces de travail",
                "Jusqu'à 50 000 redirections/mois",
                "Tags & paramètres dynamiques",
                "Exports CSV & webhook",
            ],
            highlight: false,
            cta: "Essai gratuit",
        },
        Plan {
            name: "Scale",
            price: "Sur devis",
            description: "Pensé pour les organisations distribuées avec des exigences de gouvernance avancées.",
            features: &[
                "Espaces illimités",
                "SLA 99,99% et support dédié",
                "Single Sign-On (SAML)",
                "Audit trail & conformité",
            ],
            highlight: true,
            cta: "Contacter l'équipe",
        },
    ];

    view! {
        <section class="grid gap-8" id="pricing">
            <div class="flex flex-col gap-3">
                <Badge variant=BadgeVariant::Subtle class="w-fit">"Tarification claire"</Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Choisissez le plan qui accompagne votre croissance."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-3xl">
                    "Commencez gratuitement, évoluez vers un plan personnalisable quand vous êtes prêt. Aucun paiement n'est requis pour tester la plateforme."
                </Text>
            </div>
            <div class="grid gap-6 lg:grid-cols-2">
                {plans
                    .into_iter()
                    .map(|plan| {
                        let badge = plan.highlight.then(|| {
                            view! { <Badge variant=BadgeVariant::Solid class="w-fit">"Populaire"</Badge> }
                        });

                        let card_classes = if plan.highlight {
                            "bg-gradient-to-br from-brand/10 via-surface to-surface-strong/80 border-brand/40"
                        } else {
                            "bg-surface"
                        };

                        let button_variant = if plan.highlight {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Outline
                        };

                        let price_caption = if plan.price == "Sur devis" {
                            "selon votre usage"
                        } else {
                            "par mois"
                        };

                        view! {
                            <Card class={format!("flex h-full flex-col {}", card_classes)}>
                                <CardHeader class="gap-2">
                                    {badge}
                                    <Heading level=HeadingLevel::H3 class="text-2xl">
                                        {plan.name}
                                    </Heading>
                                    <div class="flex items-baseline gap-2">
                                        <Heading level=HeadingLevel::H2 class="text-4xl font-semibold">
                                            {plan.price}
                                        </Heading>
                                        <Text tone=TextTone::Subtle class="text-sm">
                                            {price_caption}
                                        </Text>
                                    </div>
                                    <Text tone=TextTone::Muted>{plan.description}</Text>
                                </CardHeader>
                                <CardBody class="flex flex-1 flex-col gap-4 text-sm text-foreground/80">
                                    <ul class="grid gap-3">
                                        {plan
                                            .features
                                            .iter()
                                            .map(|feature| {
                                                view! {
                                                    <li class="flex items-start gap-3">
                                                        <span class="mt-1 inline-flex h-2.5 w-2.5 flex-none rounded-full bg-brand"></span>
                                                        <span>{feature.to_string()}</span>
                                                    </li>
                                                }
                                            })
                                            .collect_view()}
                                    </ul>
                                </CardBody>
                                <CardFooter class="justify-between border-t border-border/40">
                                    <Text tone=TextTone::Subtle class="text-xs">
                                        "Pas de carte bancaire requise pour démarrer."
                                    </Text>
                                    <Button variant=button_variant size=ButtonSize::Md>
                                        {plan.cta}
                                    </Button>
                                </CardFooter>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
