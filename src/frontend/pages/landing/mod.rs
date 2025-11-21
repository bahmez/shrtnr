mod sections;

use crate::frontend::{
    components::{Footer, Navigation},
    design_system::Text,
};
use leptos::prelude::*;
use sections::{
    CtaSection, FeaturesSection, HeroSection, MetricsSection, PricingSection, TestimonialsSection,
    WorkflowSection,
};

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <div class="bg-background text-foreground" id="hero">
            <Navigation/>
            <main class="mx-auto flex w-full max-w-6xl flex-col gap-24 px-6 py-16 lg:px-12 lg:py-24">
                <HeroSection/>
                <MetricsSection/>
                <FeaturesSection/>
                <WorkflowSection/>
                <PricingSection/>
                <TestimonialsSection/>
                <CtaSection/>
            </main>
            <Footer/>
            <div class="bg-muted/10 py-6 text-center text-xs text-foreground/50">
                <Text>"shrtnr est en version beta privée — demandez un accès anticipé."</Text>
            </div>
        </div>
    }
}
