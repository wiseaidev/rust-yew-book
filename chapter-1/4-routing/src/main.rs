use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProductProps {
    pub id: String,
}

#[function_component(Product)]
fn product(props: &ProductProps) -> Html {
    html! {
        <div>
            <h1>{format!("Product id: {}", props.id)}</h1>
        </div>
    }
}

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
    #[at("/register")]
    RegisterPage,
    #[at("/login")]
    LoginPage,
    #[at("/product/:id")]
    Product { id: usize },
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! { <div>{"Home Page"}</div> },
        Route::RegisterPage => html! { <div>{"Register Page"}</div> },
        Route::LoginPage => html! {<div>{"Login Page"}</div> },
        Route::Product { id } => html! {<Product id={id.to_string()} />},
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
                <Switch<Route> render={switch} />
                // Other components like spinner and toaster goes here.
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}