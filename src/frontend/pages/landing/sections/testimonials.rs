use crate::frontend::design_system::{
    Badge, BadgeVariant, Card, CardBody, CardHeader, Chip, Heading, HeadingLevel, Text, TextTone,
};
use leptos::prelude::*;

#[component]
pub fn TestimonialsSection() -> impl IntoView {
    struct Testimonial<'a> {
        quote: &'a str,
        author: &'a str,
        role: &'a str,
        metric: &'a str,
    }

    let testimonials = [
        Testimonial {
            quote: "Nous avons centralisé plus de 12 équipes autour des mêmes conventions de liens. Les rapports sont prêts pour notre comité marketing en un clic.",
            author: "Julie Martin",
            role: "Head of Growth · NovaMarket",
            metric: "+28% de taux de conversion sur nos campagnes médias",
        },
        Testimonial {
            quote: "shrtnr s'est branché à notre CRM en moins d'une journée. Les équipes locales adaptent leurs liens sans casser notre gouvernance globale.",
            author: "Rachid Benali",
            role: "Marketing Ops · Swyft Logistics",
            metric: "-45% d'erreurs de tracking en 6 semaines",
        },
    ];

    view! {
        <section class="grid gap-8" id="testimonials">
            <div class="flex flex-col gap-3">
                <Badge variant=BadgeVariant::Subtle class="w-fit">"Ils utilisent shrtnr"</Badge>
                <Heading level=HeadingLevel::H2 class="text-balance">
                    "Des équipes marketing qui orchestrent des milliers de campagnes."
                </Heading>
                <Text tone=TextTone::Muted class="max-w-3xl">
                    "Des scale-ups aux grandes entreprises, shrtnr s'intègre à des stacks existantes pour simplifier la collaboration et renforcer la cohérence."
                </Text>
            </div>
            <div class="grid gap-6 lg:grid-cols-2">
                {testimonials
                    .into_iter()
                    .map(|testimonial| {
                        view! {
                            <Card class="flex h-full flex-col bg-surface">
                                <CardHeader class="gap-4">
                                    <Chip class="w-fit bg-brand/10 text-brand">
                                        {testimonial.metric}
                                    </Chip>
                                    <Text class="text-lg leading-relaxed text-foreground">
                                        {"“"}{testimonial.quote}{"”"}
                                    </Text>
                                </CardHeader>
                                <CardBody class="flex flex-1 flex-col justify-end gap-1 text-sm text-foreground/80">
                                    <Heading level=HeadingLevel::H4 class="text-base font-semibold text-foreground">
                                        {testimonial.author}
                                    </Heading>
                                    <Text tone=TextTone::Subtle>{testimonial.role}</Text>
                                </CardBody>
                            </Card>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
}
