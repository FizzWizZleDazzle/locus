//! Physics learning platform UI components.

mod canvas;
mod challenge_panel;
mod controls;
mod equation_builder;
mod fbd_builder;
mod prediction_input;
mod quantity_selector;
mod reflection_panel;
mod what_if_explorer;

pub use canvas::PhysicsCanvas;
pub use challenge_panel::ChallengePanel;
pub use controls::PhysicsControls;
pub use equation_builder::EquationBuilder;
pub use fbd_builder::FbdBuilder;
pub use prediction_input::PredictionInput;
pub use quantity_selector::QuantitySelector;
pub use reflection_panel::ReflectionPanel;
pub use what_if_explorer::WhatIfExplorer;
