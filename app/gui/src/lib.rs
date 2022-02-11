//! The Enso IDE GUI.
//!
//! This rust crate is compiled to WASM library with all the logic of the Enso IDE GUI layer.
//! See README of the repository for the presentation of Enso IDE and its features.
//!
//! ## Where Things Start
//!
//! The function point which should be called by the web page embedding the Enso IDE is
//! `entry_point_main`.
//!
//! ## Main Layers
//!
//! - **Backend (Engine)**: The Enso IDE GUI uses the Engine Services as backend to manage and
//!   evaluate the Enso modules. The API of the services is described in the
//!   [Enso Protocol Documentation](https://enso.org/docs/developer/enso/language-server/protocol-architecture.html).
//!   and implemented in the [`engine_protocol`] crate (`controller/engine-protocol`).
//! - **Engine Model** (the [`model`] module): The Engine Model reflects the state of the Engine
//!   services: opened project, modules, attached visualizations and other entities. This Model is
//!   responsible for caching and synchronizing its state with the Engine Services.
//! - **Controllers** (the [`controller`] module). The controllers implement the logic of the Enso
//!   GUI and exposes the API to be easily used by the presenter.
//!   - **Double Representation** (the [`double_representation`] crate in
//!     `controller/double-representation`): The particular part of controllers: a library
//!     implementing conversion between textual and graph representation of Enso language.
//! - **View** (the [`ide-view`] crate in the `view` directory): A typical view layer: controls,
//!   widgets etc. implemented on the EnsoGL framework (See [`ensogl`] crate).
//! - **Presenter** (the [`presenter`] module): Synchronizes the state of the engine entities with
//!   the view, and passes the user interations to the controllers.



use wasm_bindgen::prelude::*;

pub use uuid::Uuid;

/// IDE startup function.
#[wasm_bindgen]
#[allow(dead_code)]
pub fn entry_point_ide() {}
