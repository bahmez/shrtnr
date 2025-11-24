use crate::frontend::design_system::{Text, TextTone};
use leptos::prelude::*;

#[component]
pub fn DashboardFooter() -> impl IntoView {
    view! {
        <footer class="border-t border-border/60 bg-background/90">
            <div class="mx-auto flex w-full max-w-6xl flex-col gap-4 px-4 py-6 text-sm text-foreground/60 sm:flex-row sm:items-center sm:justify-between sm:px-6 lg:px-12">
                <Text tone=TextTone::Subtle class="text-xs sm:text-sm">
                    "shrtnr dashboard · Surveillez vos liens, vos audiences et vos insights."
                </Text>
                <div class="flex items-center gap-4 text-xs sm:text-sm">
                    <a href="/support" class="transition hover:text-foreground">
                        "Support"
                    </a>
                    <a href="/status" class="transition hover:text-foreground">
                        "Status"
                    </a>
                    <a href="/aide" class="transition hover:text-foreground">
                        "Aide"
                    </a>
                </div>
            </div>
        </footer>
    }
}
