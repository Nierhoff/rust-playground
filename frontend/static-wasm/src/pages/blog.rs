use yew::prelude::{html, Html, function_component};

use crate::sections::{cta_content::CtaContent, latest_blogs::LatestBlog, testimonials::Testimonials, our_services::OurServices};

#[function_component]
pub fn Blog() -> Html {
    html! {
        <div class="home-page">
            <OurServices />
            <Testimonials />
            <CtaContent />
            <LatestBlog />
        </div>
    }
}
