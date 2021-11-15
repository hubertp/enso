use enso_frp as frp;
use ensogl::application::Application;
use ensogl::application::{self};
use ensogl::display::shape::StyleWatchFrp;
use ensogl::display::DomSymbol;
use ensogl::display::{self};
use ensogl::prelude::*;
use ensogl::system::web::AttributeSetter;
use ensogl::system::web::NodeInserter;
use ensogl::system::web::StyleSetter;
use ensogl::system::web::{self};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::MouseEvent;
use std::rc::Rc;

#[derive(Clone, CloneRef, Debug)]
enum State {
    WelcomeScreen(crate::welcome_screen::View),
    Ide(crate::project::View),
}

#[derive(Clone, CloneRef, Debug)]
#[allow(missing_docs)]
pub struct Model {
    application:    Application,
    logger:         Logger,
    display_object: display::object::Instance,
    state: State,
}

impl Model {
    pub fn new(app: &Application) -> Self {
        let application = app.clone_ref();
        let logger = Logger::new("WelcomeScreen");
        let display_object = display::object::Instance::new(&logger);
        let welcome_screen_view = app.new_view::<crate::welcome_screen::View>();
        display_object.add_child(&welcome_screen_view);
        let state = State::WelcomeScreen(welcome_screen_view);

        let model = Self { application, logger, display_object, state };

        model
    }
}

ensogl::define_endpoints! {
    Input {

    }

    Output {

    }
}


#[derive(Clone, CloneRef, Debug)]
pub struct View {
    model:  Model,
    frp:    Frp,
}

impl Deref for View {
    type Target = Frp;
    fn deref(&self) -> &Self::Target {
        &self.frp
    }
}

impl View {
    pub fn new(app: &Application) -> Self {
        let model = Model::new(&app);
        let scene = app.display.scene();
        let frp = Frp::new();
        let network = &frp.network;
        Self { model, frp }
    }
}

impl display::Object for View {
    fn display_object(&self) -> &display::object::Instance {
        &self.model.display_object
    }
}

impl application::command::FrpNetworkProvider for View {
    fn network(&self) -> &frp::Network {
        &self.frp.network
    }
}

impl application::View for View {
    fn label() -> &'static str {
        "RootView"
    }

    fn new(app: &Application) -> Self {
        Self::new(app)
    }

    fn app(&self) -> &Application {
        &self.model.application
    }
}
