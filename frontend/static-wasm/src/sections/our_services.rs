use yew::prelude::{html, Html, function_component};

#[function_component]
pub fn OurServices() -> Html {
    html! {
            <section id="our-services">
        <div class="top-bar">
            <h2 class="top-bar__title">{"Services"}</h2>
            <a href="/services" class="btn btn-primary btn-primary-lg">{"View All Services "}</a>
        </div>
        <div class="services-wrapper">
            <div class="single-service" style="--bgColor:#0D174A" data-href="/services/executive-advisory/">
                <div class="top"
                    style="--bgImage:url(https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2023/02/CIO-advisory.webp?media=1735832194)">
                    <h2>{"Executive Advisory"}</h2>
                </div>
                <div class="bottom" style="--bgColor1:#0D174A">
                    <p>{"Digital transformation only works in correlation with the business strategy. Both your business and
                        IT..."}</p>
                    <a href="/services/executive-advisory/" class="btn btn-arrow btn-arrow-white">
                        {"View More"} <span class="material-symbols-outlined">
                            {"arrow_circle_right"}
                        </span>
                    </a>
                </div>
            </div>
            <div class="single-service" style="--bgColor:#2D4E9C"
                data-href="/services/technology-and-infrastructure/">
                <div class="top"
                    style="--bgImage:url(https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/Technical-and-business-solution-architecture.webp?media=1735832194)">
                    <h2>{"Technology and Infrastructure"}</h2>
                </div>
                <div class="bottom" style="--bgColor2:#2D4E9C">
                    <p>{"Technological advancements are invented on a daily basis, and it can be difficult to navigate a
                        jungle..."}</p>
                    <a href="/services/technology-and-infrastructure/"
                        class="btn btn-arrow btn-arrow-white">
                        {"View More"} <span class="material-symbols-outlined">
                            {"arrow_circle_right"}
                        </span>
                    </a>
                </div>
            </div>
            <div class="single-service" style="--bgColor:#64B5F6"
                data-href="/services/agile-execution/">
                <div class="top"
                    style="--bgImage:url(https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2023/02/POPM.webp?media=1735832194)">
                    <h2>{"Agile Execution"}</h2>
                </div>
                <div class="bottom" style="--bgColor3:#64B5F6">
                    <p>{"With agile execution, we accelerate all processes and implementation during your company's digital
                        tra..."}</p>
                    <a href="/services/agile-execution/" class="btn btn-arrow btn-arrow-white">
                        {"View More"} <span class="material-symbols-outlined">
                            {"arrow_circle_right"}
                        </span>
                    </a>
                </div>
            </div>
            <div class="single-service" style="--bgColor:#233178"
                data-href="/services/data-migration-and-execution/">
                <div class="top"
                    style="--bgImage:url(https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/BIdata-scaled-1.webp?media=1735832194)">
                    <h2>{"Data migration and Execution"}</h2>
                </div>
                <div class="bottom" style="--bgColor4:#233178">
                    <p>{"Are you about to upgrade or replace your systems, and therefore need to have all your data
                        migrated? Y..."}</p>
                    <a href="/services/data-migration-and-execution/"
                        class="btn btn-arrow btn-arrow-white">
                        {"View More"} <span class="material-symbols-outlined">
                            {"arrow_circle_right"}
                        </span>
                    </a>
                </div>
            </div>
        </div>
    </section>
            }
}
