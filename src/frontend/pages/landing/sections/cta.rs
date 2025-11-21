use crate::frontend::design_system::{
    Badge, BadgeVariant, Button, ButtonSize, ButtonVariant, Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

#[component]
pub fn CtaSection() -> impl IntoView {
    view! {
        <section
            class="relative overflow-hidden rounded-3xl border border-brand/30 bg-gradient-to-br from-brand to-brand-muted px-8 py-12 text-brand-foreground shadow-xl"
        >
            <div class="pointer-events-none absolute inset-0 bg-[radial-gradient(circle_at_top,_rgba(255,255,255,0.25)_0,_rgba(255,255,255,0)_60%)]"></div>
            <div class="relative flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between">
                <div class="flex flex-col gap-3">
                    <Badge variant=BadgeVariant::Outline class="w-fit border-white/60 text-brand-foreground">
                        "Rejoindre la beta"
                    </Badge>
                    <Heading level=HeadingLevel::H2 class="text-balance text-3xl text-brand-foreground">
                        "Prêt à délivrer des expériences de liens cohérentes ?"
                    </Heading>
                    <Text tone=TextTone::Subtle class="max-w-2xl text-brand-foreground/80">
                        "Passez de campagnes dispersées à une gouvernance unifiée. Demandez un accès anticipé et bénéficiez d'un accompagnement personnalisé de notre équipe produit."
                    </Text>
                </div>
                <div class="flex flex-col gap-3 sm:flex-row">
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Lg class="bg-white/10 text-brand-foreground hover:bg-white/20">
                        "Demander une démo"
                    </Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg class="bg-brand-foreground text-brand hover:bg-white/90 hover:text-brand">
                        "Rejoindre la liste d'attente"
                    </Button>
                </div>
            </div>
        </section>
    }
}
