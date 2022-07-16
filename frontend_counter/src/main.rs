use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model {
        value: 0
    });

    let onadd = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    let onremove = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value - 1
            })
        })
    };

    html! {
        <div class="container">
            <div class="row">
                <button onclick={onadd}>{" +1 "}</button>
                <p>{ state.value }</p>
                <button onclick={onremove}>{" -1 "}</button>
            </div>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}