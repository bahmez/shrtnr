//! Section workflow de la landing page.
//!
//! Présente le processus d'utilisation de la plateforme étape par étape.

use crate::frontend::design_system::{
    Badge, BadgeVariant, Card, CardBody, CardHeader, Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

/// Section workflow de la landing page.
///
/// Explique le processus d'utilisation de la plateforme en plusieurs étapes
/// pour montrer la simplicité d'utilisation.
#[component]
pub fn WorkflowSection() -> impl IntoView {
    struct Step<'a> {
        title: &'a str,
        description: &'a str,
        result: &'a str,
    }

    let steps = [
        Step {
            title: "Connectez vos sources",
            description:
                "Reliez vos outils existants (CRM, CMS, automation) ou importez vos campagnes CSV. Les liens sont enrichis automatiquement.",
            result: "Paramètres UTM alignés et gouvernance centralisée.",
        },
        Step {
            title: "Définissez vos règles",
            description:
                "Choisissez des redirections dynamiques, des expirations ou des variantes selon l'origine du trafic.",
            result: "Optimisation automatique selon le contexte de clic.",
        },
        Step {
            title: "Diffusez et mesurez",
            description:
                "Synchronisez vos campagnes dans Slack, HubSpot, Notion ou via l'API. Suivez les performances par canal.",
            result: "Rapports temps réel prêts pour votre comité marketing.",
        },
    ];

    view! {
        <section
            class="grid gap-8 rounded-3xl border border-border/60 bg-gradient-to-br from-surface to-surface-strong/70 p-10 shadow-inner shadow-black/10"
            id="workflow"
        >
            <div class="flex flex-col gap-3">
                <Badge variant=BadgeVariant::Subtle class="w-fit bg-brand/15 text-brand">
                    "Flux orchestrés"
                </Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Un flux simple, de la connexion à l'analyse."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-3xl">
                    "Chaque étape est conçue pour se brancher à votre stack et offrir la visibilité nécessaire aux décideurs."
                </Text>
            </div>
            <div class="grid gap-6 lg:grid-cols-3">
                {steps
                    .into_iter()
                    .enumerate()
                    .map(|(index, step)| {
                        view! {
                            <Card class="flex h-full flex-col bg-background/80">
                                <CardHeader class="flex flex-col gap-4">
                                    <span class="inline-flex h-10 w-10 items-center justify-center rounded-xl bg-brand/10 text-sm font-semibold text-brand">
                                        {format!("{:02}", index + 1)}
                                    </span>
                                    <Heading level=HeadingLevel::H3 class="text-lg">{step.title}</Heading>
                                </CardHeader>
                                <CardBody class="flex flex-1 flex-col gap-4">
                                    <Text tone=TextTone::Muted>{step.description}</Text>
                                    <div class="rounded-xl border border-dashed border-brand/30 bg-brand/5 px-4 py-3 text-sm text-brand">
                                        <span class="text-xs font-medium uppercase tracking-wide text-brand/80">
                                            "Résultat"
                                        </span>
                                        <p class="mt-1">{step.result}</p>
                                    </div>
                                </CardBody>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
