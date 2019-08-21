// use yew::html::Children;
use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    // noGutters: bool,
    // form: bool,
    
    pub class: String,
    pub children: Children<Row>,
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
        let classes = merge_classes("row", &self.props.class);

        html! {
            <div class=classes>
            { for (self.props.children).iter() }
            </div>
        }
    }
}
