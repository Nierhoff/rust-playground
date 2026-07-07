use yew::prelude::{function_component, html, Html};

#[function_component]
pub fn Intro() -> Html {
    html! {
            <section id="intro">
        <div class="col">
            <p class="intro__subtitle">{"WHO WE ARE"}</p>
            <h2 class="intro__title">{"At "}<span>{"44EAST"}</span>{", we help companies within the financial services industry to
                achieve their business goals through digital transformation. We are here to create new paths and"}
                <span>{"business solutions"}</span>{" for our clients"}</h2>
        </div>
        <div class="col">
            <p class="intro__description">{"Given our team's experience, we are one of Denmark's most efficient and
                sought-after advisory firms in the financial sector."}
                <br />
                <br />
                {"As your strategic advisory firm, we realize your digital transformation from idea to implementation. Our
			experienced consultants place their focus entirely on developing tailor-made business solutions to achieve
			your company’s essential goals. At the same time, we provide immense value and full transparency to every
			one of our clients."}
                <br />
                <br />
                {"Welcome to 44EAST. A new breed in consulting."}
            </p>
            <a href="/about-us" class="btn btn-arrow">{"Learn More "}<span class="material-symbols-outlined">
                    {"arrow_circle_right"}
                </span>
            </a>
        </div>
    </section>
            }
}
