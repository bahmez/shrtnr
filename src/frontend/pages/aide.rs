//! Page d'aide et FAQ.
//!
//! Affiche les questions fréquentes et les réponses pour aider les utilisateurs.

use crate::frontend::design_system::{
    Card, CardBody, Heading, HeadingLevel, Text, TextTone,
};
use crate::frontend::layouts::DashboardLayout;
use leptos::prelude::*;

/// Page d'aide de l'application.
///
/// Affiche une liste de questions fréquentes avec des réponses dépliables.
#[component]
pub fn AidePage() -> impl IntoView {
    view! {
        <DashboardLayout>
            <div class="flex flex-col gap-8">
                <div class="flex flex-col gap-3 text-center">
                    <Heading level=HeadingLevel::H1 class="text-4xl">
                        "Aide"
                    </Heading>
                    <Text tone=TextTone::Muted class="text-lg">
                        "Trouvez des réponses à vos questions"
                    </Text>
                </div>

                <div class="grid gap-6">
                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col gap-6">
                                <Heading level=HeadingLevel::H2 class="text-xl">
                                    "Questions fréquentes"
                                </Heading>

                                <div class="grid gap-4">
                                    <FaqItem
                                        question="Comment créer un workspace ?"
                                        answer="Pour créer un workspace, connectez-vous à votre compte et cliquez sur le bouton 'Nouveau workspace' dans le tableau de bord. Donnez un nom à votre workspace et validez. Vous pourrez ensuite inviter des membres et commencer à raccourcir des liens."
                                    />
                                    <FaqItem
                                        question="Comment inviter des membres ?"
                                        answer="Dans les paramètres de votre workspace, accédez à la section 'Membres'. Cliquez sur le bouton 'Inviter un utilisateur', saisissez l'adresse email du membre à inviter, choisissez son rôle (Membre ou Administrateur), puis validez. Le membre recevra un accès au workspace."
                                    />
                                    <FaqItem
                                        question="Comment raccourcir un lien ?"
                                        answer="Une fois dans votre workspace, cliquez sur 'Créer un lien'. Collez l'URL longue que vous souhaitez raccourcir, personnalisez éventuellement le code court, puis cliquez sur 'Créer'. Votre lien court sera immédiatement disponible et prêt à être partagé."
                                    />
                                    <FaqItem
                                        question="Comment suivre les statistiques de mes liens ?"
                                        answer="Chaque lien dispose d'une page de statistiques détaillée. Cliquez sur un lien dans votre liste pour voir le nombre de clics, les sources de trafic, les appareils utilisés, et d'autres métriques utiles pour analyser les performances de vos liens."
                                    />
                                </div>
                            </div>
                        </CardBody>
                    </Card>

                    <Card>
                        <CardBody class="pt-6">
                            <div class="flex flex-col gap-4">
                                <Heading level=HeadingLevel::H3 class="text-lg">
                                    "Vous ne trouvez pas votre réponse ?"
                                </Heading>
                                <Text tone=TextTone::Muted>
                                    "Notre équipe de support est là pour vous aider. N'hésitez pas à nous contacter."
                                </Text>
                                <a
                                    href="/support"
                                    class="inline-flex items-center gap-2 text-brand hover:underline font-medium transition"
                                >
                                    "Contacter le support"
                                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M5 12h14"></path>
                                        <path d="m12 5 7 7-7 7"></path>
                                    </svg>
                                </a>
                            </div>
                        </CardBody>
                    </Card>
                </div>
            </div>
        </DashboardLayout>
    }
}

/// Composant pour un item de FAQ (question/réponse).
///
/// Affiche une question avec une réponse dépliable au clic.
///
/// # Arguments
///
/// * `question` - La question à afficher
/// * `answer` - La réponse à afficher quand l'item est déplié
#[component]
fn FaqItem(
    question: &'static str,
    answer: &'static str,
) -> impl IntoView {
    let expanded = RwSignal::new(false);

    view! {
        <div class="mt-4 rounded-lg border border-border/60 bg-surface-strong/30 transition hover:bg-surface-strong/50">
            <button
                class="flex w-full items-center justify-between p-4 text-left transition"
                on:click=move |_| expanded.update(|e| *e = !*e)
            >
                <Text class="font-medium pr-4">
                    {question}
                </Text>
                <div class="flex-shrink-0 text-foreground/60 transition-transform" class:rotate-180=move || expanded.get()>
                    <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="m6 9 6 6 6-6"></path>
                    </svg>
                </div>
            </button>
            {move || {
                if expanded.get() {
                    Some(view! {
                        <div class="border-t border-border/60 px-4 pb-4 pt-4">
                            <Text tone=TextTone::Muted class="text-sm leading-relaxed">
                                {answer}
                            </Text>
                        </div>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
