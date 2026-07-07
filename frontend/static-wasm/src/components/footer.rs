use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer id="colophon" class="site-footer">
    <div class="site-info">
        <div class="col col-about">
            <div class="logo-wrapper">
                <img src="https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/Logo.webp?media=1735832194"
                    alt="44EAST" class="full-size-img full-size-img-contain" />
            </div>
            <p>{ "Specialized consultancy company operating within the financial services industry in the Nordics" }</p>
            <ul class="social">
                <li>
                    <a href="https://www.linkedin.com/company/44-east" target="_blank" rel="noopener noreferrer"
                        title="LinkedIn">
                        <span class="screen-reader-text">{ "LinkedIn" }</span>
                        <i class="fab fa-linkedin-in"></i>
                    </a>
                </li>
            </ul>
        </div>
        <div class="col col-contact">
            <h3 class="footer-title">{ "Contact" }</h3>
            <ul class="contact_details">
                <li>
                    <a href="mailto:office@44east.dk" title="Email us">{ "office@44east.dk" }</a>
                </li>
                <li>
                    <a href="tel:+45 5087 9613" title="Call us">{ "+45 5087 9613" }</a>
                </li>
                <li>
                    <a href="tel:+45 4254 9111" title="Call us">{ "+45 4254 9111" }</a>
                </li>
            </ul>
        </div>
        <div class="col col-address">
            <h3 class="footer-title">{ "Address" }</h3>
            <p>{ "44 East ApS
                Kratholmsvej 9
                2830 Virum" }</p>
        </div>
        <div class="col col-newsletter">
            <h3 class="footer-title">{ "Subscribe" }</h3>
            <p>{ "Subscribe to our newsletter and
                get all news straight to your inbox" }</p>
            <form action="#" class="news-letter newsletter-form" novalidate=true>
                <div class="form-control">
                    <input type="email" name="newsletterEmail" id="newsletterEmail" placeholder="E-mail address"/>
                    <input type="submit" value="Subscribe"/>
                </div>
            </form>
        </div>
    </div>
    <div class="copy-bar">
        <p>{ "&copy; " }<span class="current-year"></span>{ "All Rights Reserved." } </p>
        <ul id="legal-links" class="menu">
            <li id="menu-item-747" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-747"><a
                    href="/terms-conditions">{ "Terms &#038; Conditions" }</a></li>
            <li id="menu-item-746"
                class="menu-item menu-item-type-post_type menu-item-object-page menu-item-privacy-policy menu-item-746">
                <a rel="privacy-policy" href="/privacy-policy">{ "Privacy Policy" }</a></li>
        </ul>
    </div>
</footer>
    }
}
