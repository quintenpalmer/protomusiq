use std::collections::BTreeMap;
use std::env;
use std::path::Path;

mod commands;
mod games;
mod movies;
mod music;

pub use commands::{AppCmd, FlexibleCmd};

fn help_text(cmds: &Vec<String>, extra: String) {
    eprintln!("Must supply <cmd> <library-path>");
    eprintln!("available commands are:");
    for cmd in cmds.into_iter() {
        eprintln!("  {}", cmd);
    }
    eprintln!("{}", extra);
    panic!("exiting with non-zero status");
}

pub struct ParentCommand {
    pub sub_commands: BTreeMap<&'static str, Command>,
}

pub enum Command {
    Parent(ParentCommand),
    Specific(Box<dyn AppCmd>),
    Flexible(Box<dyn FlexibleCmd>),
}

impl Command {
    pub fn new_parent(btree_map: BTreeMap<&'static str, Command>) -> Self {
        Command::Parent(ParentCommand {
            sub_commands: btree_map,
        })
    }

    pub fn operate_on_args(&self, mut args: Vec<String>) {
        eprintln!("operating on: {:?}", args);
        match self {
            Command::Specific(inner) => {
                if args.len() < 1 {
                    panic!("no arguments provided, this command currently just needs a path as an argument");
                }
                let lib_path = args[0].clone();
                inner.operate(Path::new(lib_path.as_str()).to_path_buf())
            }
            Command::Flexible(inner) => inner.flex_operate(args),
            Command::Parent(parent) => {
                let printable_cmds = parent.sub_commands.keys().map(|x| x.to_string()).collect();

                if args.len() < 1 {
                    help_text(&printable_cmds, "did not provide any command".to_string())
                }

                let parsed_cmd = args.remove(0);

                eprintln!("parsed cmd: {}", parsed_cmd);

                match parent.sub_commands.get(parsed_cmd.to_string().as_str()) {
                    Some(f) => f.operate_on_args(args),
                    None => help_text(
                        &printable_cmds,
                        format!("unknown command: {}", parsed_cmd.clone()),
                    ),
                }
            }
        }
    }
}

fn main() {
    let available_commands: BTreeMap<&'static str, Command> = vec![
        (
            "music",
            Command::new_parent(
                vec![
                    (
                        "list-files",
                        Command::Specific(Box::new(music::MusicFileLister {})),
                    ),
                    ("tree", Command::Specific(Box::new(music::TreeViewer {}))),
                    (
                        "table-view",
                        Command::Specific(Box::new(music::TableViewer {})),
                    ),
                    (
                        "conflicts",
                        Command::Specific(Box::new(music::ConflictLister {})),
                    ),
                    (
                        "diff-libs",
                        Command::Flexible(Box::new(music::LibDiffer {})),
                    ),
                    (
                        "csv-history",
                        Command::Flexible(Box::new(music::CSVHistoryGenerator {})),
                    ),
                    (
                        "reconcile-tracker-files",
                        Command::Flexible(Box::new(music::TrackerReconciler {})),
                    ),
                    (
                        "misc",
                        Command::new_parent(
                            vec![
                                (
                                    "covers",
                                    Command::Specific(Box::new(music::misc::AlbumCoverChecker {})),
                                ),
                                (
                                    "tracks",
                                    Command::Specific(Box::new(music::misc::TrackLister {})),
                                ),
                                (
                                    "json",
                                    Command::Specific(Box::new(music::misc::JsonProducer {})),
                                ),
                                (
                                    "dates",
                                    Command::Specific(Box::new(music::misc::DateDisplayer {})),
                                ),
                                (
                                    "length",
                                    Command::Specific(Box::new(music::misc::LengthCalcer {})),
                                ),
                                (
                                    "length-check",
                                    Command::Specific(Box::new(music::misc::LengthChecker {})),
                                ),
                                (
                                    "flac-tags",
                                    Command::Specific(Box::new(music::misc::FlacTagCollector {})),
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        ),
        (
            "movies",
            Command::new_parent(
                vec![(
                    "tree",
                    Command::Specific(Box::new(movies::MovieTreeViewer {})),
                )]
                .into_iter()
                .collect(),
            ),
        ),
        (
            "games",
            Command::new_parent(
                vec![
                    (
                        "copy-art",
                        Command::Specific(Box::new(games::GameArtCopier {})),
                    ),
                    (
                        "list-consoles",
                        Command::Flexible(Box::new(games::ConsoleLister {})),
                    ),
                    (
                        "list-games",
                        Command::Flexible(Box::new(games::ConsoleGameLister {})),
                    ),
                    (
                        "gen-desktop-file",
                        Command::Flexible(Box::new(games::GameDesktopCreator {})),
                    ),
                ]
                .into_iter()
                .collect(),
            ),
        ),
    ]
    .into_iter()
    .collect();

    let command = Command::new_parent(available_commands);

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    eprintln!("args: {:?}", args);

    command.operate_on_args(args)
}
