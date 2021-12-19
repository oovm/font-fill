use sycamore::{prelude::*, rt::Event};
use web_event::{OnClick, OnDoubleClick};
use web_sys::HtmlElement;

#[component(App < G >)]
fn app() -> View<G> {
    let name = Signal::new(String::new());
    let name2 = name.clone();
    let c = create_selector(cloned!((name) => move || !name.get().is_empty()));

    let on_pause = cloned!(() => move |e:Event| {
        log::info!("{:#?}", e);
    });
    // let on_mouseup = cloned!(() => move |e:Event| {
    //     let event: MouseEvent = e.unchecked_into();
    //     log::info!("{:#?}", event);
    // });
    let on_input = cloned!(() => move |e:Event| {
        log::info!("on_input {:#?}", e);
    });
    let on_blur = cloned!(() => move |e:Event| {
        // let event: PointerEvent = e.unchecked_into();
        log::info!("on_blur {:#?}", e);
    });
    let on_click = cloned!(() => move |e:Event| {
        let event = OnClick::from(e);
        log::info!("on_click {:#?}", event);
    });
    let on_db_click = cloned!(() => move |e:Event| {
        let event = OnDoubleClick::from(e);
        log::info!("on_db_click {:#?}", event);
    });

    view! {
        h1(
            on:pause = on_pause,
            on:input = on_input,
            on:blur = on_blur,
            on:click = on_click,
            on:dblclick = on_db_click,
        ) {
            "Hello "
            (if *c.get() {
                cloned!((name) => view! { span { (name.get()) } })
            }
            else {
                view! { span { "World" } }
            })
            "!"
        }
        input(bind:value=name2) {"123"}
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|| {
        view! {
                    pre(
                contenteditable = true,
                class = "notedown",
            ) {
                App()
            }
        }
    });
}
