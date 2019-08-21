use yew::prelude::*;

use crate::merge_classes;

pub struct Col {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub class: String,
    pub children: Children<Col>,
}

pub enum Msg {}

impl Component for Col {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Col { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Col> for Col {
    fn view(&self) -> Html<Self> {
        let mut classes = String::from("col");

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <div class=classes>
            { for (self.props.children).iter() }
            </div>
        }
    }
}
