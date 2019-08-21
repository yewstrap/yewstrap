use yew::html::Children;
use yew::prelude::*;

use crate::merge_classes;

#[derive(Properties)]
pub struct Props {
    pub active: bool,
    pub is_open: bool,
    pub nav: bool,

    /**
     * @FIXME Make this props resuable, optional, multitype children support needed
     * @TODO Add `dropdown-divider` as an child option
     */
    pub class: String,
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
        let mut classes = String::from("dropdown");

        if self.props.nav {
            classes = merge_classes(&classes, "nav-item");

            if self.props.active {
                classes = merge_classes(&classes, "active");
            }
        }

        classes = merge_classes(&classes, &self.props.class);

        html! {
            <li class={classes}>
                { for (self.props.children).iter() }
            </li>
        }
    }
}
