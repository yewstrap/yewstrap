use yew::html::Children;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub fluid: bool,

    pub class: Classes,
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

        let classes = if self.props.fluid {
            self.props.class.clone().extend("container-fluid")
        } else {
            self.props.class.clone().extend("container")
        };

        html! {
            <div class=classes>
                { for (self.props.children).iter() }
            </div>
        }
    }
}
