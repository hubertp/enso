//! Application top-level structure definition. Handles views, keyboard shortcuts and more.

pub mod args;
pub mod command;
pub mod frp;
pub mod shortcut;
pub mod view;

pub use view::View;

use crate::prelude::*;

use crate::control::callback;
use crate::display;
use crate::display::style::theme;
use crate::display::world::World;
use crate::gui::cursor::Cursor;
use crate::system::web;
use enso_web::traits::*;

use crate::animation;


// ===================
// === Application ===
// ===================

/// A top level structure for an application. It combines a view, keyboard shortcut manager, and is
/// intended to also manage layout of visible panes.
#[derive(Debug, Clone, CloneRef)]
#[allow(missing_docs)]
pub struct Application {
    pub logger:           Logger,
    pub cursor:           Cursor,
    pub display:          World,
    pub commands:         command::Registry,
    pub shortcuts:        shortcut::Registry,
    pub views:            view::Registry,
    pub themes:           theme::Manager,
    update_themes_handle: callback::Handle,
}

impl Application {
    /// Constructor.
    pub fn new(dom: &web::HtmlElement) -> Self {
        let logger = Logger::new("Application");
        let display = World::new();
        let scene = &display.default_scene;
        scene.display_in(dom);
        let commands = command::Registry::create(&logger);
        let shortcuts =
            shortcut::Registry::new(&logger, &scene.mouse.frp, &scene.keyboard.frp, &commands);
        let views = view::Registry::create(&logger, &display, &commands, &shortcuts);
        let themes = theme::Manager::from(&display.default_scene.style_sheet);
        let cursor = Cursor::new(&display.default_scene);
        display.add_child(&cursor);
        web::document.body().set_style_or_panic("cursor", "none");
        // let update_themes_handle = display.on.before_frame.add(f_!(themes.update()));
        let t = themes.clone_ref();
        let update_themes_handle =
            display.on.before_frame.add(move |_: animation::TimeInfo| t.update());
        Self { logger, cursor, display, commands, shortcuts, views, themes, update_themes_handle }
    }

    /// Create a new instance of a view.
    pub fn new_view<T: View>(&self) -> T {
        self.views.new_view(self)
    }
}

impl display::Object for Application {
    fn display_object(&self) -> &display::object::Instance {
        self.display.display_object()
    }
}

impl AsRef<theme::Manager> for Application {
    fn as_ref(&self) -> &theme::Manager {
        &self.themes
    }
}
