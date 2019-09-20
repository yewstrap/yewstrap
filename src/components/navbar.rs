use yew::html::Children;
use yew::prelude::*;

#[derive(Properties)]
pub struct Props {
    pub class: Classes,
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
        let classes = self.props.class.extend("navbar navbar-expand-lg navbar-dark bg-dark"); // TODO is dark theming the navbar a good default behavior?
        html! {
            <nav class=classes>
                { for (self.props.children).iter() }
            </nav>
        }
    }
}
