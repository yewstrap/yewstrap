use yew::html::*;
use yew::prelude::*;
use yew::virtual_dom::VChild;

use crate::Row;


type Children<T> = Box<dyn Fn() -> Vec<VChild<T, Container>>>;

#[derive(Properties)]
pub struct Props {
    pub fluid: bool,
    pub class: String,
    #[props(required)]
    pub children: Children<Row>,
}

pub struct Container {
    props: Props,
}

pub enum Msg {}

impl Component for Container {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Container { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Container> for Container {
    fn view(&self) -> Html<Self> {
        let mut classes = String::from("container");

        if self.props.fluid {
            classes = format!("{}-fluid", classes);
        }

        classes = format!("{} {}", classes, self.props.class);

        html! {
            <div class={classes}>
            { for (self.props.children)().into_iter() }
            </div>
        }
    }
}
