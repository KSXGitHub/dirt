pub mod app;
pub mod args;
pub mod build_data_tree;
pub mod build_data_tree_from_fs;
pub mod data_tree;
pub mod display_os_string;
pub mod measurement_system;
pub mod reporter;
pub mod size;
pub mod size_getters;
pub mod visualize;

/// The main program.
pub fn main() {
    app::App::from_env().run()
}
