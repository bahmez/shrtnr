use crate::frontend::design_system::{
    Badge, BadgeVariant, Card, CardBody, Heading, HeadingLevel, Text, TextTone,
};
use crate::frontend::layouts::DashboardLayout;
use leptos::prelude::*;

#[component]
pub fn StatusPage() -> impl IntoView {
    view! {
        <DashboardLayout>
            <div class="flex flex-col gap-4 -mt-4">
                <div class="flex flex-col gap-2 text-center">
                    <Heading level=HeadingLevel::H1 class="text-4xl">
                        "Status"
                    </Heading>
                    <Text tone=TextTone::Muted class="text-lg">
                        "Statut des services Shrtnr"
                    </Text>
                </div>

                <div class="grid gap-4">
                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col gap-4">
                                <div class="flex items-center justify-between">
                                    <div class="flex flex-col gap-2">
                                        <Heading level=HeadingLevel::H2 class="text-xl">
                                            "État global"
                                        </Heading>
                                        <Text tone=TextTone::Muted class="text-sm">
                                            "Dernière mise à jour : il y a quelques secondes"
                                        </Text>
                                    </div>
                                    <Badge variant=BadgeVariant::Solid class="bg-success text-success-foreground">
                                        "✓ Opérationnel"
                                    </Badge>
                                </div>

                                <div class="h-px bg-border/60"></div>

                                <div class="flex flex-col gap-4">
                                    <Text class="font-semibold">
                                        "Services"
                                    </Text>

                                    <div class="grid gap-3">
                                        <ServiceStatus
                                            name="API"
                                            status="operational"
                                            description="L'API répond normalement"
                                        />
                                        <ServiceStatus
                                            name="Interface web"
                                            status="operational"
                                            description="L'interface fonctionne normalement"
                                        />
                                        <ServiceStatus
                                            name="Raccourcissement de liens"
                                            status="operational"
                                            description="La création de liens courts fonctionne"
                                        />
                                        <ServiceStatus
                                            name="Redirections"
                                            status="operational"
                                            description="Les redirections sont opérationnelles"
                                        />
                                        <ServiceStatus
                                            name="Base de données"
                                            status="operational"
                                            description="Connexion stable"
                                        />
                                    </div>
                                </div>

                                <div class="rounded-xl border border-success/20 bg-success/5 p-4">
                                    <div class="flex items-start gap-3">
                                        <div class="mt-0.5 flex h-5 w-5 items-center justify-center rounded-full bg-success/20">
                                            <div class="h-2 w-2 rounded-full bg-success"></div>
                                        </div>
                                        <div class="flex flex-col gap-1">
                                            <Text class="text-sm font-medium text-success">
                                                "Tous les services fonctionnent normalement"
                                            </Text>
                                            <Text tone=TextTone::Muted class="text-sm">
                                                "Aucun incident n'est actuellement signalé."
                                            </Text>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </DashboardLayout>
    }
}

#[component]
fn ServiceStatus(
    name: &'static str,
    status: &'static str,
    description: &'static str,
) -> impl IntoView {
    let (icon, color_class) = match status {
        "operational" => ("✓", "text-success"),
        "degraded" => ("⚠", "text-warning"),
        "outage" => ("✗", "text-danger"),
        _ => ("•", "text-foreground/40"),
    };

    view! {
        <div class="flex items-center justify-between rounded-lg border border-border/60 bg-surface-strong/30 p-4 transition hover:bg-surface-strong/50">
            <div class="flex flex-col gap-1">
                <Text class="font-medium">
                    {name}
                </Text>
                <Text tone=TextTone::Muted class="text-sm">
                    {description}
                </Text>
            </div>
            <div class=format!("flex h-8 w-8 items-center justify-center rounded-lg font-semibold {}", color_class)>
                {icon}
            </div>
        </div>
    }
}
