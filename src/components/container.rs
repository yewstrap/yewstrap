use yew::html::Children;
use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub fluid: bool,

    pub class: String,
    pub children: Children<Container>,
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

        classes = merge_classes(&classes ,&self.props.class);

        html! {
            <div class=classes>
                { for (self.props.children).iter() }
            </div>
        }
    }
}
