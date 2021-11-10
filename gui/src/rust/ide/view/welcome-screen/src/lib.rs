use ensogl::prelude::*;
use ensogl::{
    system::web::{self, NodeInserter},
    application::{self, Application},
    display::{self, DomSymbol, shape::StyleWatchFrp}
};
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlElement, MouseEvent};
use enso_frp as frp;

const CONTENT: &str = include_str!("../assets/templates-view.html");

#[derive(Clone, CloneRef, Debug)]
#[allow(missing_docs)]
pub struct Model {
    application: Application,
    logger: Logger,
    dom: DomSymbol,
    display_object: display::object::Instance,
}

impl Model {
    pub fn new(app: &Application) -> Self {
        let application = app.clone_ref();
        let logger = Logger::new("WelcomeScreen");
        let display_object = display::object::Instance::new(&logger);
        let root = DomSymbol::new(&web::create_div());
        root.dom().set_class_name("templates-view");
        root.dom().set_id("templates-view");
        let container = web::create_div();
        container.set_class_name("container");
        root.append_or_panic(&container);
        let side_menu = web::create_element("aside");
        side_menu.set_class_name("side-menu");
        let your_projects = web::create_element("h2");
        your_projects.set_text_content(Some("Your projects"));
        side_menu.append_or_panic(&your_projects);
        container.append_or_panic(&side_menu);


        display_object.add_child(&root);
        app.display.scene().dom.layers.back.manage(&root);


        let model = Self {
            application,
            logger,
            dom: root,
            display_object,
        }; 

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
    model: Model,
    styles: StyleWatchFrp,
    frp: Frp,
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
        let styles = StyleWatchFrp::new(&scene.style_sheet);
        let frp = Frp::new();
        let network = &frp.network;
        frp::extend! { network
            let shape  = app.display.scene().shape();
            position <- map(shape, |scene_size| {
                let x = -scene_size.width / 2.0;
                let y =  scene_size.height / 2.0;
                Vector2(x, y)
            });
            eval position ((pos) model.display_object.set_position_xy(*pos));
        }
        Self {
            model,
            styles,
            frp,
        }
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
        "WelcomeScreen"
    }

    fn new(app: &Application) -> Self {
        Self::new(app)
    }

    fn app(&self) -> &Application {
        &self.model.application
    }
}
