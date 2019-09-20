// use yew::html::Children;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    // noGutters: bool,
    // form: bool,
    
    pub class: Classes,
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
        let classes = self.props.class.extend("row");

        html! {
            <div class=classes>
            { for (self.props.children).iter() }
            </div>
        }
    }
}
