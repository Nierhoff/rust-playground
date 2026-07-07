use yew::prelude::{function_component, html, Html};

#[function_component]
pub fn HeroHome() -> Html {
    html! {
        <div class="hero hero-home"
                style="--bgImage:url(https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2024/03/29f692d4-bc57-4e52-8b41-1b409bc0998c.jpg?media=1735832194);">
            <div class="hero-home__content">
                <p class="hero-home__subtitle">{"We are 44EAST"}</p>
                <h2 class="hero-home__title">{"We create change and build bridges between "}<span>{"business"}</span>{" and digital transformation"}</h2>
                <a href="/services"
                    class="btn btn-primary btn-primary-lg">{"View All Services"}</a>
            </div>
        </div>
    }
}
