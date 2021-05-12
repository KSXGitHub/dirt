pub mod app;
pub mod args;
pub mod build_data_tree;
pub mod data_tree;
pub mod fs_tree_builder;
pub mod measurement_system;
pub mod os_string_display;
pub mod reporter;
pub mod size;
pub mod size_getters;
pub mod visualize;

/// The main program.
pub fn main() {
    app::App::from_env().run()
}
