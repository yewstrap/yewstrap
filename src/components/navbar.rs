use yew::html::Children;
use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub class: String,
    pub children: Children<Navbar>,
}

pub struct Navbar {
    props: Props,
}

pub enum Msg {
    Toggle,
}

impl Component for Navbar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Navbar { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Navbar> for Navbar {
    fn view(&self) -> Html<Self> {
        let classes = merge_classes(
            "navbar navbar-expand-lg navbar-dark bg-dark",
            &self.props.class,
        );
        html! {
            <nav class=classes>
                { for (self.props.children).iter() }
            </nav>
        }
    }
}
