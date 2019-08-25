#![recursion_limit = "128"]
#![feature(decl_macro, proc_macro_hygiene, try_trait)]

extern crate gettext;
#[macro_use]
extern crate gettext_macros;
#[macro_use]
extern crate lazy_static;
extern crate pulldown_cmark;
#[macro_use]
extern crate stdweb;
extern crate serde_json;

use stdweb::web::{event::*, *};

init_i18n!(
    "plume-front",
    ar,
    bg,
    ca,
    cs,
    de,
    en,
    eo,
    es,
    fr,
    gl,
    hi,
    hr,
    it,
    ja,
    nb,
    pl,
    pt,
    ro,
    ru,
    sr,
    sk,
    sv
);

mod editor;

compile_i18n!();

lazy_static! {
    static ref CATALOG: gettext::Catalog = {
        let catalogs = include_i18n!();
        let lang = js! { return navigator.language }.into_string().unwrap();
        let lang = lang.splitn(2, '-').next().unwrap_or("en");
        catalogs
            .iter()
            .find(|(l, _)| l == &lang)
            .unwrap_or(&catalogs[0])
            .clone()
            .1
    };
}

fn main() {
    std::panic::set_hook(Box::new(|info: &std::panic::PanicInfo| {
        let mut msg = info.to_string();
        msg.push_str("\n\nStack:\n\n");
        let e = error::Error::new("Panicked");
        let stack = js! { return @{&e}.stack; }
            .into_string()
            .unwrap_or_default();
        msg.push_str(&stack);
        msg.push_str("\n\n");
        console!(error, msg);
    }));

    menu();
    search();
    editor::init()
        .map_err(|e| console!(error, format!("Editor error: {:?}", e)))
        .ok();
}

/// Toggle menu on mobile device
///
/// It should normally be working fine even without this code
/// But :focus-within is not yet supported by Webkit/Blink
fn menu() {
    if let Some(button) = document().get_element_by_id("menu") {
        if let Some(menu) = document().get_element_by_id("content") {
            button.add_event_listener(|_: ClickEvent| {
                document()
                    .get_element_by_id("menu")
                    .map(|menu| menu.class_list().add("show"));
            });
            menu.add_event_listener(|_: ClickEvent| {
                document()
                    .get_element_by_id("menu")
                    .map(|menu| menu.class_list().remove("show"));
            });
        }
    }
}

/// Clear the URL of the search page before submitting request
fn search() {
    if let Some(form) = document().get_element_by_id("form") {
        form.add_event_listener(|_: SubmitEvent| {
            document()
                .query_selector_all("#form input")
                .map(|inputs| {
                    for input in inputs {
                        js! {
                            if (@{&input}.name === "") {
                                @{&input}.name = @{&input}.id
                            }
                            if (@{&input}.name && !@{&input}.value) {
                                @{&input}.name = "";
                            }
                        }
                    }
                })
                .ok();
        });
    }
}
