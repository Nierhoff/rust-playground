use yew::prelude::{function_component, html, Html};

use crate::components::{footer::Footer, header::Header};
use crate::sections::{
    cta_content::CtaContent, featured_blogs::FeaturedBlogs, hero_home::HeroHome, intro::Intro,
    latest_blogs::LatestBlog, our_services::OurServices, testimonials::Testimonials,
};

#[function_component]
pub fn Home() -> Html {
    html! {
        <div id="page"
                                 class="site"
                                 data-current-lang="en">

            <Header />
            <HeroHome />
            <FeaturedBlogs />
            <Intro />
            <OurServices />
            <Testimonials />
            <CtaContent />
            <LatestBlog />
            <Footer />
        </div>
    }
}
