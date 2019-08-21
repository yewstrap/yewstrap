use yew::prelude::*;

use crate::merge_classes;

pub struct NavbarBrand {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub href: String,

    pub class: String,
    pub children: Children<NavbarBrand>,
}

pub enum Msg {}

impl Component for NavbarBrand {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavbarBrand { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<NavbarBrand> for NavbarBrand {
    fn view(&self) -> Html<Self> {
        let classes = merge_classes("navbar-brand", &self.props.class);

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
