use yew::prelude::{html, Html, function_component};

#[function_component]
pub fn LatestBlog() -> Html {
    html! {
        <section id="latest-blogs">
        <div class="header">
            <h2>{ "Check out our Blog" }</h2>
            <p>{"Stay updated with 44EAST and learn more about the latest industry news."}</p>
        </div>
        <div class="row">
            <div class="single-blog"
                data-href="http://localhost:8080/keep-your-insurance-company-ahead-of-the-competition/">
                <div class="img-wrapper">
                    <img src="https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2021/11/jakob_blog_post-scaled-1.webp?media=1735832194"
                        alt="Keep your insurance company ahead of the competition "
                        class="full-size-img full-size-img-cover"/>
                </div>
                <div class="meta">
                    <p>{ "November 11, 2021 - 6 min read" }</p>
                </div>
                <h2>{"Keep your insurance company ahead of the competition "}</h2>
                <a href="http://localhost:8080/keep-your-insurance-company-ahead-of-the-competition/" class="btn">
                    {"Read More "}<span class="material-symbols-outlined">{ "arrow_circle_right" }</span>
                </a>
            </div>
        </div>
    </section>
        }
}
