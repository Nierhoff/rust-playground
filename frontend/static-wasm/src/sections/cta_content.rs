use yew::prelude::{html, Html, function_component};

#[function_component]
pub fn CtaContent() -> Html {
    html! {
        <section id="cta-contact">
			<div class="row">
				<p>{" Ready to take your business to the next level? Contact our team now "}</p>
				<a href="/contact" class="btn btn-light btn-light-lg">{"Get In Touch"}</a>
			</div>
        </section>
    }
}
