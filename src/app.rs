pub mod sub;

pub use sub::Sub;

use crate::{
    args::{Args, Quantity},
    data_tree::DataTree,
    reporter::{ErrorOnlyReporter, ErrorReport, ProgressAndErrorReporter, ProgressReport},
    runtime_error::RuntimeError,
    size::{Bytes, Size},
    size_getters::GET_APPARENT_SIZE,
    visualizer::Direction,
};
use std::time::Duration;
use structopt_utilities::StructOptUtils;

#[cfg(unix)]
use crate::{
    size::Blocks,
    size_getters::{GET_BLOCK_COUNT, GET_BLOCK_SIZE},
};

/// The main application.
pub struct App {
    /// The CLI arguments.
    args: Args,
}

impl App {
    /// Initialize the application from the environment.
    pub fn from_env() -> Self {
        App {
            args: Args::strict_from_args(),
        }
    }

    /// Run the application.
    pub fn run(self) -> Result<(), RuntimeError> {
        // DYNAMIC DISPATCH POLICY:
        //
        // Errors rarely occur, therefore, using dynamic dispatch to report errors have an acceptable
        // impact on performance.
        //
        // The other operations which are invoked frequently should not utilize dynamic dispatch.

        let column_width_distribution = self
            .args
            .column_width_distribution()
            .expect("get column width distribution");

        let report_error = if self.args.silent_errors {
            ErrorReport::SILENT
        } else {
            ErrorReport::TEXT
        };

        fn error_only_reporter<Data: Size>(
            report_error: fn(ErrorReport),
        ) -> ErrorOnlyReporter<fn(ErrorReport)> {
            ErrorOnlyReporter::new(report_error)
        }

        fn progress_and_error_reporter<Data>(
            report_error: fn(ErrorReport),
        ) -> ProgressAndErrorReporter<Data, fn(ErrorReport)>
        where
            Data: Size + Into<u64> + Send + Sync,
            ProgressReport<Data>: Default + 'static,
        {
            ProgressAndErrorReporter::new(
                ProgressReport::TEXT,
                Duration::from_millis(100),
                report_error,
            )
        }

        fn sort<Name, Data: Size>(children: &mut Vec<DataTree<Name, Data>>) {
            children.sort_by(|left, right| left.data().cmp(&right.data()).reverse());
        }

        fn no_sort<Name, Data: Size>(_: &mut Vec<DataTree<Name, Data>>) {}

        macro_rules! sub {
            (
                $data:ty => $format:expr;
                $quantity:ident => $get_data:ident;
                $progress:literal => $create_reporter:ident;
                $no_sort:literal => $post_process_children:ident;
            ) => {
                if let Args {
                    quantity: Quantity::$quantity,
                    progress: $progress,
                    no_sort: $no_sort,
                    files,
                    bytes_format,
                    top_down,
                    max_depth,
                    minimal_ratio,
                    ..
                } = self.args
                {
                    return Sub {
                        direction: Direction::from_top_down(top_down),
                        get_data: $get_data,
                        post_process_children: $post_process_children,
                        reporter: $create_reporter::<$data>(report_error),
                        bytes_format: $format(bytes_format),
                        files,
                        column_width_distribution,
                        max_depth,
                        minimal_ratio,
                    }
                    .run();
                }
            };
        }

        sub! {
            Bytes => |x| x;
            ApparentSize => GET_APPARENT_SIZE;
            false => error_only_reporter;
            false => sort;
        }

        sub! {
            Bytes => |x| x;
            ApparentSize => GET_APPARENT_SIZE;
            false => error_only_reporter;
            true => no_sort;
        }

        sub! {
            Bytes => |x| x;
            ApparentSize => GET_APPARENT_SIZE;
            true => progress_and_error_reporter;
            false => sort;
        }

        sub! {
            Bytes => |x| x;
            ApparentSize => GET_APPARENT_SIZE;
            true => progress_and_error_reporter;
            true => no_sort;
        }

        #[cfg(unix)]
        sub! {
            Bytes => |x| x;
            BlockSize => GET_BLOCK_SIZE;
            false => error_only_reporter;
            false => sort;
        }

        #[cfg(unix)]
        sub! {
            Bytes => |x| x;
            BlockSize => GET_BLOCK_SIZE;
            false => error_only_reporter;
            true => no_sort;
        }

        #[cfg(unix)]
        sub! {
            Bytes => |x| x;
            BlockSize => GET_BLOCK_SIZE;
            true => progress_and_error_reporter;
            false => sort;
        }

        #[cfg(unix)]
        sub! {
            Bytes => |x| x;
            BlockSize => GET_BLOCK_SIZE;
            true => progress_and_error_reporter;
            true => no_sort;
        }

        #[cfg(unix)]
        sub! {
            Blocks => |_| ();
            BlockCount => GET_BLOCK_COUNT;
            false => error_only_reporter;
            false => sort;
        }

        #[cfg(unix)]
        sub! {
            Blocks => |_| ();
            BlockCount => GET_BLOCK_COUNT;
            false => error_only_reporter;
            true => no_sort;
        }

        #[cfg(unix)]
        sub! {
            Blocks => |_| ();
            BlockCount => GET_BLOCK_COUNT;
            true => progress_and_error_reporter;
            false => sort;
        }

        #[cfg(unix)]
        sub! {
            Blocks => |_| ();
            BlockCount => GET_BLOCK_COUNT;
            true => progress_and_error_reporter;
            true => no_sort;
        }

        dbg!(self.args);
        panic!("Invalid combination of arguments")
    }
}
