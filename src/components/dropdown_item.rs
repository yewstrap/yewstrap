use yew::html::Children;
use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub href: String,

    pub class: String,
    pub children: Children<DropdownItem>,
}

pub struct DropdownItem {
    props: Props,
}

pub enum Msg {}

impl Component for DropdownItem {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DropdownItem { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<DropdownItem> for DropdownItem {
    fn view(&self) -> Html<Self> {
        let classes: String = merge_classes("dropdown-item", &self.props.class);

        let mut href = String::from(&self.props.href);

        if href == "" {
            href = format!("#");
        }


        html! {
            <a class=classes href=href>
                { for (self.props.children).iter() }
            </a>
        }
    }
}
