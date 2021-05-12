use super::{
    build_data_tree::{BuildDataTree, Info},
    data_tree::DataTree,
    os_string_display::OsStringDisplay,
    reporter::{error_report::Operation::*, ErrorReport, Event, Reporter},
    size::Size,
};
use pipe_trait::Pipe;
use std::{
    fs::{read_dir, symlink_metadata, Metadata},
    path::PathBuf,
};

/// Build a [`DataTree`] from a directory tree using [`From`] or [`Into`].
#[derive(Debug)]
pub struct BuildDataTreeFromFilesystem<Data, GetData, Report, PostProcessChildren>
where
    Data: Size + Send + Sync,
    GetData: Fn(&Metadata) -> Data + Sync,
    Report: Reporter<Data> + Sync,
    PostProcessChildren: Fn(&mut Vec<DataTree<OsStringDisplay, Data>>) + Copy + Send + Sync,
{
    /// Root of the directory tree.
    pub root: PathBuf,
    /// Returns size of an item.
    pub get_data: GetData,
    /// Reports progress to external system.
    pub reporter: Report,
    /// Processes lists of children after forming.
    pub post_process_children: PostProcessChildren,
}

impl<Data, GetData, Report, PostProcessChildren>
    From<BuildDataTreeFromFilesystem<Data, GetData, Report, PostProcessChildren>>
    for DataTree<OsStringDisplay, Data>
where
    Data: Size + Send + Sync,
    GetData: Fn(&Metadata) -> Data + Sync,
    Report: Reporter<Data> + Sync,
    PostProcessChildren: Fn(&mut Vec<DataTree<OsStringDisplay, Data>>) + Copy + Send + Sync,
{
    fn from(
        builder: BuildDataTreeFromFilesystem<Data, GetData, Report, PostProcessChildren>,
    ) -> Self {
        let BuildDataTreeFromFilesystem {
            root,
            get_data,
            reporter,
            post_process_children,
        } = builder;

        BuildDataTree::<PathBuf, OsStringDisplay, Data, _, _, PostProcessChildren> {
            name: root.file_name().map_or_else(
                || ".".pipe(OsStringDisplay::os_string_from),
                OsStringDisplay::os_string_from,
            ),

            path: root,

            get_info: |path| {
                reporter.report(Event::BeginScanning);

                let stats = match symlink_metadata(&path) {
                    Err(error) => {
                        reporter.report(Event::EncounterError(ErrorReport {
                            operation: SymlinkMetadata,
                            path,
                            error,
                        }));
                        return Info {
                            data: Data::default(),
                            children: Vec::new(),
                        };
                    }
                    Ok(stats) => stats,
                };

                let children: Vec<_> = if stats.file_type().is_dir() {
                    match read_dir(path) {
                        Err(error) => {
                            reporter.report(Event::EncounterError(ErrorReport {
                                operation: ReadDirectory,
                                path,
                                error,
                            }));
                            return Info::default();
                        }
                        Ok(entries) => entries,
                    }
                    .into_iter()
                    .filter_map(|entry| match entry {
                        Err(error) => {
                            reporter.report(Event::EncounterError(ErrorReport {
                                operation: AccessEntry,
                                path,
                                error,
                            }));
                            None
                        }
                        Ok(entry) => entry.file_name().pipe(OsStringDisplay::from).pipe(Some),
                    })
                    .collect()
                } else {
                    Vec::new()
                };

                reporter.report(Event::FinishScanning);

                let data = get_data(&stats);
                reporter.report(Event::ReceiveData(data));

                Info { data, children }
            },

            join_path: |prefix, name| prefix.join(&name.0),

            post_process_children,
        }
        .into()
    }
}
