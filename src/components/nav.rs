use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub navbar: bool,

    pub class: Classes,
    pub children: Children<Nav>,
}

pub struct Nav {
    props: Props,
}

pub enum Msg {}

impl Component for Nav {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Nav { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<Nav> for Nav {
    fn view(&self) -> Html<Self> {
        let classes: Classes = if self.props.navbar {
            self.props.class.clone().extend("navbar-nav")
        } else {
            self.props.class.clone().extend("nav")
        };

        html! {
            <ul class=classes>
            { for (self.props.children).iter() }
            </ul>
        }
    }
}
