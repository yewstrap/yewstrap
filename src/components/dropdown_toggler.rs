use yew::prelude::*;

pub struct DropdownToggler {
    props: Props,
}

#[derive(Properties)]
pub struct Props {
    pub caret: bool,
    pub nav: bool,
    // #[props(required)]
    // pub onclick: Callback<()>,
    pub class: Classes,
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
        let mut classes = self.props.class.clone();
        if self.props.caret {
            classes.push("dropdown-toggle");
        }
        if self.props.nav {
            classes.push( "nav-link");
        }

        html! {
            <a class=classes href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false" onclick=|_| Msg::Toggle >
                { for (self.props.children).iter() }
            </a>
        }
    }
}
