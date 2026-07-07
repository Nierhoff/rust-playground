use yew::prelude::*;

use crate::sections::{cta_content::CtaContent, latest_blogs::LatestBlog, testimonials::Testimonials, our_services::OurServices};

#[function_component]
pub fn Contact() -> Html {
    html! {
        <div class="home-page">
            <OurServices />
            <Testimonials />
            <CtaContent />
            <LatestBlog />
        </div>
    }
}
