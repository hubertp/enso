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
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::MouseEvent;

#[derive(Clone, Debug, PartialEq)]
enum State {
    WelcomeScreen,
    ProjectOpened,
}

#[derive(Clone, CloneRef, Debug)]
#[allow(missing_docs)]
pub struct Model {
    application:    Application,
    logger:         Logger,
    display_object: display::object::Instance,
    state:          Rc<CloneCell<State>>,
    welcome_view:   crate::welcome_screen::View,
    project_view:   crate::project::View,
}

impl Model {
    pub fn new(app: &Application) -> Self {
        let application = app.clone_ref();
        let logger = Logger::new("RootView");
        let display_object = display::object::Instance::new(&logger);
        let welcome_view = app.new_view::<crate::welcome_screen::View>();
        display_object.add_child(&welcome_view);

        let project_view = app.new_view::<crate::project::View>();

        let model = Self {
            application,
            logger,
            display_object,
            welcome_view,
            project_view,
            state: Rc::new(CloneCell::new(State::WelcomeScreen)),
        };

        model
    }

    fn switch_view(&self) {
        self.state.set(State::ProjectOpened);
        self.display_object.remove_child(&self.welcome_view);
        self.display_object.add_child(&self.project_view);
    }
}

ensogl::define_endpoints! {
    Input {
        switch_view(),
    }

    Output {

    }
}


#[derive(Clone, CloneRef, Debug)]
pub struct View {
    model:   Model,
    pub frp: Frp,
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
        frp::extend! { network
            eval_ frp.switch_view(model.switch_view());
        }
        Self { model, frp }
    }

    pub fn status_bar(&self) -> &crate::status_bar::View {
        self.model.project_view.status_bar()
    }

    pub fn project_view(&self) -> &crate::project::View {
        &self.model.project_view
    }

    pub fn welcome_view(&self) -> &crate::welcome_screen::View {
        &self.model.welcome_view
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
