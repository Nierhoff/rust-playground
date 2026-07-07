use core::fmt;
use std::{ffi::FromBytesUntilNulError, str::FromStr};
use yew::prelude::*;

use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
enum Language {
    English,
    Danish,
}

impl Language {

    pub fn to_language_string(&self) -> String {
        match *self {
            Language::English => "en-GB".to_string(),
            Language::Danish => "da-DK".to_string(),
        }
    }

    pub fn to_country(&self) -> String {
        match *self {
            Language::English => "GB".to_string(),
            Language::Danish => "DK".to_string(),
        }
    }

    fn from_string(input: String) -> Language {
        match input.as_str()  {
            "English" => Language::English,
            "Danish" => Language::Danish,
            _ => Language::English,
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "English" => Ok(Language::English),
            "Danish" => Ok(Language::Danish),
            _ => Err(()),
        }
    }

}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Language::English => write!(f, "English"),
            Language::Danish => write!(f, "Danish"),
        }
    }
}

#[function_component(LanguageSelector)]
fn language_selector() -> Html {
    let supported_languages = vec![Language::English, Language::Danish];
    let selected_language = use_state(|| {
            Language::from_string(LocalStorage::get("selected_language")
                .unwrap_or_else(|_| Language::English.to_string())
        )
    });

    /*let on_change = {
        let selected_language = selected_language.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target_dyn_into::<web_sys::HtmlSelectElement>()
                .unwrap()
                .value();
            LocalStorage::set("selected_language", &value).unwrap();
            selected_language.set(value);
        })
    };*/

    html! {
        <div class="lang-switcher">
            <p>
                {" en "}
                <span class="material-symbols-outlined">
                    { "expand_more" }
                </span>
            </p>
            <ul class="lang-switcher-dropdown">
                { for supported_languages.iter().cloned().enumerate().map(|(index, lang)| html! { 
                    if (lang != *selected_language ) {
                        { html! { 
                            <li class={ format!("lang-item lang-item-{} lang-item-{} lang-item-first", index, lang.to_country().to_lowercase()) }>
                                <a lang={lang.to_language_string() }
                                    hreflang={lang.to_language_string() }
                                    href="./">{lang.to_country().to_lowercase()}</a>
                            </li> 
                        }
                    }
                    };
                
                }
                    )}
            </ul>
        </div>
    }
}

#[function_component]
pub fn Header() -> Html {
    html! {
        <header id="masthead" class="site-header">
        <a href="http://localhost:8080/" class="logo-wrapper d-block">
            <img src="https://usercontent.one/wp/www.44east.dk/wp-content/uploads/2022/12/Logo.webp?media=1735832194"
                alt="44EAST" class="full-size-img full-size-img-contain d-block" />
        </a>

        <nav id="site-navigation" class="main-navigation">
            <ul id="primary-menu" class="menu">
                <li id="menu-item-449" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-449"><a
                        href="/about-us">{ "About Us" }</a>
                </li>
                <li id="menu-item-444" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-444"><a
                        href="/careers">{ "Careers" }</a>
                </li>
                <li id="menu-item-335" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-335"><a
                        href="/services">{ "Services" }</a>
                </li>
                <li id="menu-item-339" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-339"><a
                        href="/blog">{ "Blog" }</a></li>
                <li id="menu-item-638" class="menu-item menu-item-type-post_type menu-item-object-page menu-item-638"><a
                        href="/contact">{ "Contact" }</a></li>
            </ul>
        </nav>

        <LanguageSelector />
        <button id="menu-trigger" title="mobile menu trigger">
            <span></span>
            <span></span>
            <span></span>
        </button>
    </header>
    }
}
