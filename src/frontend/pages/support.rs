//! Page de support.
//!
//! Permet aux utilisateurs de contacter l'équipe de support par email.

use crate::frontend::design_system::{
    Button, ButtonSize, ButtonVariant, Card, CardBody, Heading, HeadingLevel, Text, TextTone,
};
use crate::frontend::layouts::DashboardLayout;
use leptos::prelude::*;

/// Page de support de l'application.
///
/// Affiche les informations de contact et permet d'envoyer un email
/// directement à l'équipe de support.
#[component]
pub fn SupportPage() -> impl IntoView {
    view! {
        <DashboardLayout>
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-3 text-center">
                    <Heading level=HeadingLevel::H1 class="text-4xl">
                        "Support"
                    </Heading>
                    <Text tone=TextTone::Muted class="text-lg">
                        "Contactez notre équipe"
                    </Text>
                </div>

                <div class="grid gap-6">
                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col gap-6">
                                <div class="flex flex-col gap-2">
                                    <Heading level=HeadingLevel::H2 class="text-xl">
                                        "Besoin d'aide ?"
                                    </Heading>
                                    <Text tone=TextTone::Muted>
                                        "Pour toute question ou problème, contactez-nous directement par email."
                                    </Text>
                                </div>

                                <div class="flex flex-col gap-4 rounded-xl border border-border/60 bg-surface-strong/50 p-6">
                                    <div class="flex items-center gap-3">
                                        <div class="flex h-10 w-10 items-center justify-center rounded-lg bg-brand/10">
                                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-brand">
                                                <rect width="20" height="16" x="2" y="4" rx="2"></rect>
                                                <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"></path>
                                            </svg>
                                        </div>
                                        <div class="flex flex-col gap-1">
                                            <Text class="text-sm font-medium text-foreground/70">
                                                "Email de contact"
                                            </Text>
                                            <a
                                                href="mailto:dev@stepwise.agency"
                                                class="text-lg font-semibold text-brand hover:underline transition"
                                            >
                                                "dev@stepwise.agency"
                                            </a>
                                        </div>
                                    </div>
                                </div>

                                <div class="flex flex-col gap-3 rounded-xl border border-brand/20 bg-brand/5 p-4">
                                    <Text class="text-sm font-medium">
                                        "💡 Conseil"
                                    </Text>
                                    <Text tone=TextTone::Muted class="text-sm">
                                        "Pour un traitement plus rapide, veuillez inclure autant de détails que possible dans votre message : description du problème, captures d'écran si nécessaire, et étapes pour reproduire le problème."
                                    </Text>
                                </div>

                                <div class="flex gap-3">
                                    <Button
                                        variant=ButtonVariant::Primary
                                        size=ButtonSize::Md
                                        full_width=true
                                        on:click=move |_| {
                                            let _ = window().location().set_href("mailto:dev@stepwise.agency");
                                        }
                                    >
                                        "Envoyer un email"
                                    </Button>
                                    <Button
                                        variant=ButtonVariant::Outline
                                        size=ButtonSize::Md
                                        full_width=true
                                        on:click=move |_| {
                                            let _ = window().location().set_href("/aide");
                                        }
                                    >
                                        "Consulter l'aide"
                                    </Button>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </DashboardLayout>
    }
}
