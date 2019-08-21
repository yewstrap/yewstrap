use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub active: bool,

    pub class: String,
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
        let mut classes = String::from("nav-item");

        if self.props.active {
            classes = merge_classes(&classes, "active");
        }

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <li class=classes>
            { for (self.props.children).iter() }
            </li>
        }
    }
}
