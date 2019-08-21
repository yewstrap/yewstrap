use yew::prelude::*;

use crate::merge_classes;

pub struct DropdownToggler {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub caret: bool,
    pub nav: bool,
    // #[props(required)]
    // pub onclick: Callback<()>,
    pub class: String,
    pub children: Children<DropdownToggler>,
}

pub enum Msg {
    Toggle,
}

impl Component for DropdownToggler {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DropdownToggler { props }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                // self.props.onclick.emit(());
            }
        }
        false
    }
}

impl Renderable<DropdownToggler> for DropdownToggler {
    fn view(&self) -> Html<Self> {
        let mut classes = String::new();

        if self.props.caret {
            classes = merge_classes(&classes, "dropdown-toggle");
        }
        if self.props.nav {
            classes = merge_classes(&classes, "nav-link");
        }

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <a class=classes href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" onclick=|_| Msg::Toggle >
                { for (self.props.children).iter() }
            </a>
        }
    }
}
