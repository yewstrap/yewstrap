use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub active: bool,

    pub class: Classes,
    pub children: Children<NavItem>,
}

pub struct NavItem {
    props: Props,
}

pub enum Msg {}

impl Component for NavItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<NavItem> for NavItem {
    fn view(&self) -> Html<Self> {
        let mut classes = self.props.class.extend("nav-link");
        if self.props.active {
            classes.push("active")
        }

        html! {
            <li class=classes>
            { for (self.props.children).iter() }
            </li>
        }
    }
}
