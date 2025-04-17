use yew::prelude::*;
extern crate qrc;
use qrc::*; 

#[function_component(App)]
fn app() -> Html {
    let qrcode = QRCode::from_string("test".to_string());
    html! {
        <>
            <title>{"I love rust!"}</title>
            <h1>{"Hello World!"}</h1>
            <button onclick={Callback::from(|_| ())}>{"create QR code"}</button>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
        }
