use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub navbar: bool,

    pub class: String,
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
        let mut classes = String::from("nav");
        if self.props.navbar {
            classes = format!("navbar-{}", &classes);
        }

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <ul class=classes>
            { for (self.props.children).iter() }
            </ul>
        }
    }
}
