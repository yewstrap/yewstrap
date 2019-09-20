use yew::prelude::*;

pub struct Col {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub class: Classes,
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
        let mut classes = self.props.class.extend("col");

        html! {
            <div class=classes>
            { for (self.props.children).iter() }
            </div>
        }
    }
}
