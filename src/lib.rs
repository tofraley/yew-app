use wasm_bindgen::prelude::*;
use yew::prelude::*;
struct Model {
    link: ComponentLink<Self>,
    is_ready: bool,
    node_ref: NodeRef,
}

enum Msg {
    StartGame,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            is_ready: false,
            node_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::StartGame => {
                self.is_ready = true;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let is_ready = self.is_ready;
        html! {
            <div>
                <canvas ref=self.node_ref.clone() id="glcanvas" tabindex='1'></canvas>
                <button onclick=self.link.callback(|_| Msg::StartGame)>
                    { "Start Game" }
                </button>
                <p>{ "Move or wall-grab with A or D | Jump/Double jump with SPACE" }</p>
                <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
                {
                    if is_ready {
                        html! {
                            <>
                            <script>{ "load('macroquad-test.wasm');" }</script>
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.update(Msg::StartGame);
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
