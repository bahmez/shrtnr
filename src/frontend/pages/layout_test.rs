use crate::frontend::{
    design_system::{
        Badge, BadgeVariant, Card, CardBody, CardHeader, Heading, HeadingLevel, Text, TextTone,
    },
    layouts::DashboardLayout,
};
use leptos::prelude::*;

#[component]
pub fn LayoutTestPage() -> impl IntoView {
    let highlights = [
        (
            "Liens actifs",
            "128",
            "Liens courts opérationnels sur vos campagnes.",
        ),
        (
            "CTR moyen",
            "34%",
            "Performance consolidée sur les 30 derniers jours.",
        ),
        (
            "Sources connectées",
            "6",
            "CMS, CRM et automatisations branchés.",
        ),
    ];

    view! {
        <DashboardLayout>
            <section class="grid gap-6">
                <div class="flex flex-col gap-2">
                    <Heading level=HeadingLevel::H1 class="text-balance">
                        "Bienvenue sur votre espace de contrôle."
                    </Heading>
                    <Text tone=TextTone::Muted class="max-w-2xl">
                        "Cette page temporaire illustre le layout du dashboard : navigation, dropdown profil, modal de paramètres et footer sont disponibles pour intégration."
                    </Text>
                </div>

                <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
                    {highlights
                        .into_iter()
                        .map(|(title, value, description)| {
                            view! {
                                <Card class="bg-background/70">
                                    <CardHeader class="pb-4">
                                        <Badge variant=BadgeVariant::Subtle class="w-fit">
                                            {title}
                                        </Badge>
                                    </CardHeader>
                                    <CardBody class="flex flex-col gap-2">
                                        <Heading level=HeadingLevel::H2 class="text-3xl font-semibold">
                                            {value}
                                        </Heading>
                                        <Text tone=TextTone::Subtle>
                                            {description}
                                        </Text>
                                    </CardBody>
                                </Card>
                            }
                        })
                        .collect_view()}
                </div>

                <Card class="bg-surface">
                    <CardHeader class="gap-2">
                        <Heading level=HeadingLevel::H3>
                            "Prochaines étapes"
                        </Heading>
                        <Text tone=TextTone::Subtle>
                            "Modifiez ce layout pour brancher vos graphiques, tables de liens et automations."
                        </Text>
                    </CardHeader>
                    <CardBody class="grid gap-3 text-sm text-foreground/80">
                        <Text>
                            "Le sélecteur de projets est mocké et prêt à être connecté à une API."
                        </Text>
                        <Text>
                            "Les liens \"Links\", \"Analytics\" et \"Settings\" sont configurés pour être remplacés par vos routes internes."
                        </Text>
                        <Text>
                            "Le modal de paramétrage fonctionne et peut être branché sur vos services utilisateurs."
                        </Text>
                    </CardBody>
                </Card>
            </section>
        </DashboardLayout>
    }
}
