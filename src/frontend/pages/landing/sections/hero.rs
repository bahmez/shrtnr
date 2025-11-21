use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Card, CardBody, CardHeader, Chip,
    Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="grid gap-12 lg:grid-cols-[1.1fr_0.9fr] lg:items-center" id="hero">
            <div class="flex flex-col gap-6">
                <Badge variant=BadgeVariant::Subtle class="w-fit">
                    "Version beta privée"
                </Badge>
                <Heading level=HeadingLevel::H1 class="text-balance">
                    "Le raccourcisseur d'URL pensé pour les équipes orientées data."
                </Heading>
                <Text tone=TextTone::Muted class="text-lg leading-relaxed">
                    "shrtnr centralise vos liens courts, automatise vos campagnes et transforme chaque clic en insight actionnable — sans complexité technique."
                </Text>
                <div class="flex flex-wrap items-center gap-3">
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>
                        "Demander un accès anticipé"
                    </Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Lg>
                        "Explorer la plateforme"
                    </Button>
                    <Chip class="bg-surface-strong/80 text-foreground/80 backdrop-blur">
                        "99,9% de disponibilité"
                    </Chip>
                </div>
                <div class="flex flex-wrap gap-3 text-sm text-foreground/80">
                    {["Suivi des performances en temps réel", "Automatisations multi-canal", "Sécurité d'entreprise"]
                        .into_iter()
                        .map(|item| {
                            view! {
                                <Chip class="bg-surface px-3 py-1 text-xs uppercase tracking-wide">
                                    {item}
                                </Chip>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
            <Card class="relative overflow-hidden border-brand/20 bg-gradient-to-br from-surface to-surface-strong/80 shadow-brand/10">
                <CardHeader class="pb-4">
                    <Heading level=HeadingLevel::H3 class="flex items-center gap-2 text-base">
                        <span class="inline-flex h-8 w-8 items-center justify-center rounded-xl bg-brand/15 text-brand">
                            "⚡"
                        </span>
                        "Aperçu en direct"
                    </Heading>
                    <Text tone=TextTone::Subtle class="text-sm">
                        "Orchestrez la génération de liens courts, le routage des campagnes et le suivi des conversions depuis un seul tableau de bord."
                    </Text>
                </CardHeader>
                <CardBody class="flex flex-col gap-4">
                    <div class="flex flex-col gap-3 rounded-xl border border-border/50 bg-background/80 p-4 shadow-inner shadow-black/5">
                        <span class="text-xs uppercase tracking-wide text-brand/80">"Campagne"</span>
                        <Heading level=HeadingLevel::H3 class="text-lg font-semibold">
                            "Lancement produit Q2"
                        </Heading>
                        <div class="grid gap-2 text-sm">
                            <div class="flex items-center justify-between rounded-lg bg-surface px-3 py-2 font-mono text-xs text-foreground/80">
                                <span>"longform.domain.com/produit/q2/campagne-paid?"</span>
                                <Chip class="bg-brand/10 text-xs text-brand">"Source"</Chip>
                            </div>
                            <div class="flex items-center justify-between rounded-lg bg-brand/10 px-3 py-2 font-mono text-xs text-brand">
                                <span>"shrtnr.dev/q2-launch"</span>
                                <Chip class="bg-brand text-brand-foreground text-[11px]">"+42% de CTR"</Chip>
                            </div>
                        </div>
                    </div>
                    <div class="grid gap-2 rounded-xl border border-dashed border-brand/30 bg-brand/5 p-4 text-sm text-brand">
                        <span class="text-xs font-medium uppercase tracking-wide">"Automatisation prête"</span>
                        <span>"Déploiement Slack, HubSpot et QR codes synchronisés."</span>
                    </div>
                </CardBody>
            </Card>
        </section>
    }
}
