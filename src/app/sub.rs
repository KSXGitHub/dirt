use crate::{
    fs_tree_builder::FsTreeBuilder,
    os_string_display::OsStringDisplay,
    reporter::Reporter,
    size::Size,
    tree::Tree,
    visualizer::{ColumnWidthDistribution, Direction, Visualizer},
};
use std::{fs::Metadata, iter::once, num::NonZeroUsize, path::PathBuf};

/// The sub program of the main application.
pub struct Sub<Data, GetData, Report, PostProcessChildren>
where
    Data: Size + Into<u64> + Send + Sync,
    Report: Reporter<Data> + Copy + Sync,
    GetData: Fn(&Metadata) -> Data + Copy + Sync,
    PostProcessChildren: Fn(&mut Vec<Tree<OsStringDisplay, Data>>) + Copy + Send + Sync,
{
    /// List of files and/or directories.
    pub files: Vec<PathBuf>,
    /// The direction of the visualization.
    pub direction: Direction,
    /// Distribution and maximum number of characters/blocks can be placed in a line
    pub column_width_distribution: ColumnWidthDistribution,
    /// Maximum number of levels that should be visualized.
    pub max_depth: NonZeroUsize,
    /// Returns measured quantity of the files/directories.
    pub get_data: GetData,
    /// Reports measurement progress.
    pub reporter: Report,
    /// Processes lists of children after forming.
    pub post_process_children: PostProcessChildren,
}

impl<Data, GetData, Report, PostProcessChildren> Sub<Data, GetData, Report, PostProcessChildren>
where
    Data: Size + Into<u64> + Send + Sync,
    Report: Reporter<Data> + Copy + Sync,
    GetData: Fn(&Metadata) -> Data + Copy + Sync,
    PostProcessChildren: Fn(&mut Vec<Tree<OsStringDisplay, Data>>) + Copy + Send + Sync,
{
    /// Run the sub program.
    pub fn run(self) {
        let Sub {
            files,
            direction,
            column_width_distribution,
            max_depth,
            get_data,
            reporter,
            post_process_children,
        } = self;

        let mut iter = files
            .into_iter()
            .map(|root| -> Tree<OsStringDisplay, Data> {
                FsTreeBuilder {
                    root,
                    get_data,
                    reporter,
                    post_process_children,
                }
                .into()
            });

        let tree = if let Some(tree) = iter.next() {
            tree
        } else {
            return Sub {
                files: vec![".".into()],
                ..self
            }
            .run();
        };

        // ExactSizeIterator::is_empty is unstable
        let tree = if iter.len() == 0 {
            tree
        } else {
            let children: Vec<_> = once(tree).chain(iter).collect();
            Tree::dir(
                OsStringDisplay::os_string_from("..."),
                Data::default(),
                children,
            )
        };

        let visualizer = Visualizer {
            tree: &tree,
            direction,
            column_width_distribution,
            max_depth,
        };

        println!("{}", visualizer);
    }
}