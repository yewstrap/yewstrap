use yew::html::Children;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub active: bool,
    pub is_open: bool,
    pub nav: bool,

    /**
     * @FIXME Make this props resuable, optional, multitype children support needed
     * @TODO Add `dropdown-divider` as an child option
     */
    pub class: Classes,
    pub children: Children<Dropdown>,
}

pub struct Dropdown {
    props: Props,
}

pub enum Msg {}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Dropdown { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Dropdown> for Dropdown {
    fn view(&self) -> Html<Self> {
        let mut classes = self.props.class.extend("dropdown");

        if self.props.nav {
            classes.push("nav-item");

            if self.props.active {
                classes.push("active");
            }
        }

        html! {
            <li class={classes}>
                { for (self.props.children).iter() }
            </li>
        }
    }
}
