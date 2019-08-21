use yew::prelude::*;

use crate::merge_classes;

pub struct NavbarToggler {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    // #[props(required)]
    // pub onclick: Callback<()>,
    pub class: String,
    pub children: Children<NavbarToggler>,
}

pub enum Msg {
    Toggle,
}

impl Component for NavbarToggler {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavbarToggler { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        match _msg {
            Msg::Toggle => {
                // self.props.onclick.emit(());
            }
        }
        false
    }
}

impl Renderable<NavbarToggler> for NavbarToggler {
    fn view(&self) -> Html<Self> {
        let classes = merge_classes("navbar-toggler", &self.props.class);

        html! {
            <button class=classes type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="true" aria-label="Toggle navigation"
                onclick=|_| Msg::Toggle
            >
                <span class="navbar-toggler-icon"></span>
            </button>
        }
    }
}
