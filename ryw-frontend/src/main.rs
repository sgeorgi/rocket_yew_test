use yew::prelude::*;

struct Message {
    title: String,
    body: String
}

impl Component for Message {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            title: "title".to_string(),
            body: "body".to_string()
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ &self.title }</p>
                <p>{ &self.body }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Message>();
}
