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
use enso_protocol::project_manager;
use std::rc::Rc;

#[derive(Clone, CloneRef, Debug)]
enum State {
    Ide(super::project::View),
    WelcomeScreen(super::welcome_screen::View),
}

#[derive(Clone, CloneRef, Debug)]
pub struct View {
    application: Application,
    state: State,
}

impl View {
    pub fn new(app: &Application, project_manager: Rc<dyn project_manager::API>) -> Self {
        let welcome_screen = app.new_view::<super::welcome_screen::View>();
        let state = State::WelcomeScreen(welcome_screen.clone_ref());
        app.display.add_child(&welcome_screen);
        Self { application: app.clone_ref(), state }
    }
}

