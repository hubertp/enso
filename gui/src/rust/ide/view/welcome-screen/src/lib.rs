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

const CONTENT: &str = include_str!("../assets/templates-view.html");

#[derive(Clone, CloneRef, Debug)]
#[allow(missing_docs)]
pub struct Model {
    application:    Application,
    logger:         Logger,
    dom:            DomSymbol,
    display_object: display::object::Instance,
}

impl Model {
    pub fn new(app: &Application) -> Self {
        let application = app.clone_ref();
        let logger = Logger::new("WelcomeScreen");
        let display_object = display::object::Instance::new(&logger);

        let welcome_screen = {
            let welcome_screen = web::create_div();
            welcome_screen.set_class_name("templates-view");
            welcome_screen.set_id("templates-view");

            let container = {
                let container = web::create_div();
                container.set_class_name("container");

                container.append_or_panic(&Self::create_side_menu());
                container.append_or_panic(&Self::create_templates());

                container
            };
            welcome_screen.append_or_panic(&container);
            welcome_screen
        };

        let dom = DomSymbol::new(&welcome_screen);
        display_object.add_child(&dom);
        app.display.scene().dom.layers.back.manage(&dom);


        let model = Self { application, logger, dom, display_object };

        model
    }

    fn create_side_menu() -> web_sys::Element {
        let side_menu = web::create_element("aside");
        side_menu.set_class_name("side-menu");
        let header = {
            let header = web::create_element("h2");
            header.set_text_content(Some("Your projects"));
            header
        };
        side_menu.append_or_panic(&header);

        let projects_list = {
            let projects_list = web::create_element("ul");
            projects_list.set_id("projects-list");

            let new_project = web::create_element("li");
            new_project.set_id("projects-list-new-project");
            new_project
                .set_inner_html(r#"<img src="/assets/new-project.svg" />Create a new project"#);
            projects_list.append_or_panic(&new_project);

            projects_list
        };
        side_menu.append_or_panic(&projects_list);

        side_menu
    }

    fn create_templates() -> web_sys::Element {
        let content = web::create_element("main");
        content.set_class_name("content");

        let templates = {
            let templates = web::create_div();
            let header = {
                let header = web::create_element("h2");
                header.set_text_content(Some("Templates"));
                header
            };
            templates.append_or_panic(&header);
            templates.append_or_panic(&Self::create_cards());
            templates
        };
        content.append_or_panic(&templates);

        content
    }

    fn create_cards() -> web_sys::HtmlDivElement {
        let cards = web::create_div();
        cards.set_class_name("cards");

        let row1 = {
            let row = web::create_div();
            row.set_class_name("row");
            let card_spreadsheets = Self::create_card(
                "card-spreadsheets",
                "card card-spreadsheets",
                Some("/assets/spreadsheets.png"),
                "Combine spreadsheets",
                "Glue multiple spreadsheets together to analyse all your data at once.",
            );

            let card_geo = Self::create_card(
                "card-geo",
                "card card-geo",
                None,
                "Geospatial analysis",
                "Learn where to open a coffee shop to maximize your income.",
            );
            row.append_or_panic(&card_spreadsheets);
            row.append_or_panic(&card_geo);

            row
        };
        let row2 = {
            let row = web::create_div();
            row.set_class_name("row");
            let card_visualize = Self::create_card(
                "card-visualize",
                "card card-visualize",
                None,
                "Analyze GitHub stars",
                "Find out which of Enso's repositories are most popular over time.",
            );
            row.append_or_panic(&card_visualize);

            row
        };
        cards.append_or_panic(&row1);
        cards.append_or_panic(&row2);

        cards
    }

    fn create_card(
        id: &str,
        class: &str,
        img: Option<&str>,
        header: &str,
        content: &str,
    ) -> web_sys::HtmlDivElement {
        let card = web::create_div();
        card.set_id(id);
        card.set_class_name(class);
        if let Some(src) = img {
            let img = web::create_element("img");
            img.set_attribute_or_panic("src", src);
            card.append_or_panic(&img);
        }
        let card_header = web::create_element("h3");
        card_header.set_text_content(Some(header));
        card.append_or_panic(&card_header);
        let p = web::create_element("p");
        p.set_text_content(Some(content));
        card.append_or_panic(&p);

        card
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
    styles: StyleWatchFrp,
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
        Self { model, styles, frp }
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
