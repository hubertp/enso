//! The integration layer between IDE controllers and the view.

use enso_frp as frp;

pub mod file_system;
pub mod project;
pub mod visualization;

use crate::prelude::*;

use crate::controller::ide::StatusNotification;
use crate::model::undo_redo::Aware;

use ide_view::graph_editor::SharedHashMap;


// =======================
// === IDE Integration ===
// =======================

// === Model ===

/// The model of integration object. It is extracted and kept in Rc, so it can be referred to from
/// various FRP endpoints or executor tasks.
#[derive(Debug)]
struct Model {
    logger:              Logger,
    controller:          controller::Ide,
    view:                ide_view::root::View,
    project_integration: RefCell<Option<project::Integration>>,
}

impl Model {
    /// Create a new project integration
    fn setup_and_display_new_project(self: Rc<Self>) {
        // Remove the old integration first. We want to be sure the old and new integrations will
        // not race for the view.
        *self.project_integration.borrow_mut() = None;

        let project_model =
            if let Some(model) = self.controller.current_project() { model } else { return };
        let status_notifications = self.controller.status_notifications().clone_ref();
        let project = controller::Project::new(project_model, status_notifications.clone_ref());

        executor::global::spawn(async move {
            match project.initialize().await {
                Ok(result) => {
                    let view = self.view.project_view().clone_ref();
                    let text = result.main_module_text;
                    let graph = result.main_graph;
                    let ide = self.controller.clone_ref();
                    let project = project.model;
                    let main_module = result.main_module_model;
                    let integration =
                        project::Integration::new(view, graph, text, ide, project, main_module);
                    // We don't want any initialization-related changes to appear on undo stack.
                    integration.graph_controller().undo_redo_repository().clear_all();
                    *self.project_integration.borrow_mut() = Some(integration);
                    error!(self.logger, "Project integration created");
                }
                Err(err) => {
                    let err_msg = format!("Failed to initialize project: {}", err);
                    error!(self.logger, "{err_msg}");
                    status_notifications.publish_event(err_msg)
                }
            }
        });
    }

    fn open_project(&self, name: &str) {
        let logger = self.logger.clone_ref();
        let controller = self.controller.clone_ref();
        let name = name.to_owned();
        crate::executor::global::spawn(async move {
            if let Ok(managing_api) = controller.manage_projects() {
                match managing_api.list_projects().await {
                    Ok(projects) => {
                        if let Some(uuid) = projects
                            .into_iter()
                            .find(|project| project.name.0 == name)
                            .map(|project| project.id)
                        {
                            if let Err(err) = managing_api.open_project(uuid).await {
                                error!(logger, "Could not open open project `{name}`: {err}");
                            }
                        } else {
                            error!(logger, "Could not find project with name {name}");
                        }
                    }
                    Err(err) => error!(logger, "Could not list projects: {err}")
                }
            }
        })
    }

    fn create_project(&self) {
        let logger = self.logger.clone_ref();
        let controller = self.controller.clone_ref();
        crate::executor::global::spawn(async move {
            if let Ok(managing_api) = controller.manage_projects() {
                if let Err(err) = managing_api.create_new_project().await {
                    error!(logger, "Could not create new project: {err}");
                }
            }
        })
    }
}

// === Integration ===

/// The Integration Object
///
/// It is responsible for integrating IDE controllers and views, so user actions will work, and
/// notifications from controllers will update the view.
#[derive(Clone, CloneRef, Debug)]
pub struct Integration {
    model:   Rc<Model>,
    network: frp::Network,
}

impl Integration {
    /// Create the integration of given controller and view.
    pub fn new(controller: controller::Ide, view: ide_view::root::View) -> Self {
        let logger = Logger::new("ide::Integration");
        let project_integration = default();
        let welcome_view_frp = view.welcome_view().frp.clone_ref();
        let model = Rc::new(Model { logger, controller, view: view.clone_ref(), project_integration });

        frp::new_network! { network
            let opened_project = welcome_view_frp.opened_project.clone_ref();
            project_opened <- opened_project.filter_map(|name| name.clone());
            let root_frp = view.frp.clone_ref();
            eval project_opened((name) {
                model.open_project(name);
                root_frp.switch_view.emit(());
            });

            let root_frp = view.frp.clone_ref();
            eval_ welcome_view_frp.create_project({
                model.create_project();
                root_frp.switch_view.emit(());
            });
        };
        Self { model, network }.init()
    }

    /// Initialize integration, so FRP outputs of the view will call the proper controller methods,
    /// and controller notifications will be delivered to the view accordingly.
    pub fn init(self) -> Self {
        self.initialize_status_bar_integration();
        self.initialize_controller_integration();
        self.initialize_welcome_screen();
        self.model.clone_ref().setup_and_display_new_project();
        self
    }

    fn initialize_status_bar_integration(&self) {
        use controller::ide::BackgroundTaskHandle as ControllerHandle;
        use ide_view::status_bar::process::Id as ViewHandle;

        let logger = self.model.logger.clone_ref();
        let process_map = SharedHashMap::<ControllerHandle, ViewHandle>::new();
        let status_bar = self.model.view.status_bar().clone_ref();
        let status_notif_sub = self.model.controller.status_notifications().subscribe();
        let status_notif_updates = status_notif_sub.for_each(move |notification| {
            info!(logger, "Received notification {notification:?}");
            match notification {
                StatusNotification::Event { label } => {
                    status_bar.add_event(ide_view::status_bar::event::Label::new(label));
                }
                StatusNotification::BackgroundTaskStarted { label, handle } => {
                    status_bar.add_process(ide_view::status_bar::process::Label::new(label));
                    let view_handle = status_bar.last_process.value();
                    process_map.insert(handle, view_handle);
                }
                StatusNotification::BackgroundTaskFinished { handle } => {
                    if let Some(view_handle) = process_map.remove(&handle) {
                        status_bar.finish_process(view_handle);
                    } else {
                        warning!(logger, "Controllers finished process not displayed in view");
                    }
                }
            }
            futures::future::ready(())
        });

        executor::global::spawn(status_notif_updates)
    }

    fn initialize_controller_integration(&self) {
        let stream = self.model.controller.subscribe();
        let weak = Rc::downgrade(&self.model);
        executor::global::spawn(stream.for_each(move |notification| {
            if let Some(model) = weak.upgrade() {
                match notification {
                    controller::ide::Notification::NewProjectCreated
                    | controller::ide::Notification::ProjectOpened =>
                        model.setup_and_display_new_project(),
                }
            }
            futures::future::ready(())
        }));
    }

    fn initialize_welcome_screen(&self) {
        let controller = self.model.controller.clone_ref();
        let welcome_view_frp = self.model.view.welcome_view().frp.clone_ref();
        let logger = self.model.logger.clone_ref();
        crate::executor::global::spawn(async move {
            if let Ok(project_manager) = controller.manage_projects() {
                match project_manager.list_projects().await {
                    Ok(projects) => {
                        let names = projects
                            .into_iter()
                            .map(|project| project.name.into())
                            .collect::<Vec<_>>();
                        welcome_view_frp.projects_list(names);
                    }
                    Err(err) => error!(logger, "Unable to get list of projects: {err}"),
                }
            }
        });
    }
}
