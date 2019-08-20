use yew::prelude::*;

use crate::components::dropdown::Dropdown;

#[derive(PartialEq, Default, Clone, Properties)]
pub struct Props {
    pub title: String,
}


pub struct Navbar {
    props: Props
}

pub enum Msg {}

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
        html! {
            <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
                <div class="container">
                    <a class="navbar-brand" href="#">
                        {&self.props.title}
                    </a>
                    <button class="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarSupportedContent">
                        <ul class="navbar-nav ml-auto">
                            <li class="nav-item active">
                                <a class="nav-link" href="#">{"Home"} <span class="sr-only">{"(current)"}</span></a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="#">{"Link"}</a>
                            </li>
                            <Dropdown />
                            <li class="nav-item">
                                <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">{"Disabled"}</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>
        }
    }
}
