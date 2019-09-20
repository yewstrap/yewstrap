use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub active: bool,
    pub disabled: bool,
    pub href: String,

    pub class: Classes,
    pub children: Children<NavLink>,
}

pub struct NavLink {
    props: Props,
}

pub enum Msg {}

impl Component for NavLink {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        NavLink { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }
}

impl Renderable<NavLink> for NavLink {
    fn view(&self) -> Html<Self> {
        let mut classes = self.props.class.extend("nav-link");
        if self.props.active {
            classes.push("active")
        }
        if self.props.disabled {
            classes.push("disabled")
        }

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
