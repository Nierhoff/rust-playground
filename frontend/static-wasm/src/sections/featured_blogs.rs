use yew::prelude::{function_component, html, Html, Properties};
use yew::virtual_dom::AttrValue;

#[derive(Properties, Clone, PartialEq, Eq)]
struct BlogProps {
    pub no: i8,
    pub image_link: AttrValue,
    pub image_title: AttrValue,
    pub style: AttrValue,
    pub title: AttrValue,
    pub sub_title: AttrValue,
    pub link: AttrValue,
    pub class: AttrValue,
}

#[function_component]
fn BlogCard(props: &BlogProps) -> Html {
    html! {
        <div class={ props.class.clone() }
                     style={ props.style.clone() }
                     data-href={ props.link.clone() }>
                    <div class="img-wrapper">
                        <img src={ props.image_link.clone() }
                             alt={ props.image_title.clone() }
                             class="full-size-img full-size-img-contain"/>
                    </div>
                    <h3>{ props.title.clone() }</h3>
                    <p>{ props.sub_title.clone() }</p>
                    <a href={ props.link.clone() }>
                                {"View More"}					<span class="material-symbols-outlined">
                                    {"arrow_circle_right"}
                            </span>
                    </a>
                </div>
    }
}

#[function_component]
pub fn FeaturedBlogs() -> Html {
    let blogs: Vec<BlogProps> = vec![
		BlogProps {
			no: 0,
			class: "blog-card blog-card-featured swiper-slide swiper-slide-active".into(),
			image_link: "https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/device-message.webp?media=1735832194".into(),
			image_title: "Senior Financial Services Industry Advisor".into(),
			style: "--bgColor1: #070E2F; width: 249.333px; margin-right: 20px;".into(),
			title: "Senior Financial Services Industry Advisor".into(),
			sub_title: "44EAST is on a fast track succeeding as the new breed within financial services advisory, as the diffe...".into(),
			link: "http://localhost:8080/careers/senior-financial-services-industry-advisor/".into()
		},
		BlogProps {
			no: 1,
			class: "blog-card blog-card-featured swiper-slide swiper-slide-next".into(),
			image_link: "https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/device-message.webp?media=1735832194".into(),
			image_title: "Digital Transformation".into(),
			style: "--bgColor2: #25316D; width: 249.333px; margin-right: 20px;".into(),
			title: "Digital Transformation".into(),
			sub_title: "The key to your future lies in keeping up with global digital developments. We help you create innovat...".into(),
			link: "http://localhost:8080/services/digital-transformation/".into()
		},
		BlogProps {
			no: 2,
			class: "blog-card blog-card-featured swiper-slide".into(),
			image_link: "https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/device-message.webp?media=1735832194".into(),
			image_title: "Data migration and Execution".into(),
			style: "--bgColor3: #2D4E9C; width: 249.333px; margin-right: 20px;".into(),
			title: "Data migration and Execution".into(),
			sub_title: "Are you about to upgrade or replace your systems, and therefore need to have all your data migrated? Y...".into(),
			link: "http://localhost:8080/services/data-migration-and-execution/".into()
		},
	];

    html! {
        <section id="featured-blogs">
        <div class="blogs-wrapper swiper swiper-initialized swiper-horizontal swiper-backface-hidden">
            <div class="swiper-wrapper"
                 style="transform: translate3d(0px, 0px, 0px);">
                 { for blogs.iter().cloned().map(|blog_prop| html! {
                    <BlogCard ..blog_prop />
                }) }
            </div>
            <div class="swiper-pagination swiper-pagination-clickable swiper-pagination-bullets swiper-pagination-horizontal">
                { for blogs.iter().enumerate().map(|(index, _blog_prop)| html! {
                    { match index {
                        0 => html! { <span class="swiper-pagination-bullet swiper-pagination-bullet-active"/> },
                        _ => html! { <span class="swiper-pagination-bullet"/> },
                    }}
                    })
                }
            </div>
        </div>
    </section>
        }
}
