use yew::html::*;
use yew::prelude::*;
use yew::virtual_dom::VChild;

// type Children<T> = Box<dyn Fn() -> Vec<VChild<T, Row>>>;

#[derive(Default, Properties)]
pub struct Props {
    // noGutters: bool,
    // form: bool,
}

pub struct Row {
    props: Props,
}

pub enum Msg {}

impl Component for Row {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Row { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Row> for Row {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="row">
            // { for (self.props.children)().into_iter() }
            </div>
        }
    }
}
