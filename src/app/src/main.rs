mod altmodes;
mod backend;
mod datastore;
mod gui;
mod model;
mod services;
mod shared;
mod util;

use std::env;

use iced::Application;

use crate::gui::state;

pub enum RunMode {
    GUI,
    Background,
    ReportToTracker,
    GenerateReport,
    Prototype,
    Explore,
}

#[derive(Debug)]
pub enum AppError {
    Iced(iced::Error),
    BackgroundServer(altmodes::background::Error),
    Tracker(altmodes::repl::TrackerError),
    ReportGeneration(altmodes::report::Error),
    Prototype(altmodes::proto::Error),
    Explore(altmodes::explore::Error),
}

pub fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();

    let mut run_mode = RunMode::GUI;

    for arg in args.into_iter() {
        if arg == "--background" {
            run_mode = RunMode::Background;
        }
        if arg == "--tracker" {
            run_mode = RunMode::ReportToTracker;
        }
        if arg == "--report" {
            run_mode = RunMode::GenerateReport;
        }
        if arg == "--prototype" {
            run_mode = RunMode::Prototype;
        }
        if arg == "--explore" {
            run_mode = RunMode::Explore;
        }
    }

    match run_mode {
        RunMode::GUI => state::App::run(iced::Settings::default()).map_err(AppError::Iced),
        RunMode::Background => {
            altmodes::background::run_server().map_err(AppError::BackgroundServer)
        }
        RunMode::ReportToTracker => altmodes::repl::report_tracks().map_err(AppError::Tracker),
        RunMode::GenerateReport => {
            altmodes::report::generate_year_end_report().map_err(AppError::ReportGeneration)
        }
        RunMode::Prototype => altmodes::proto::entry_point().map_err(AppError::Prototype),
        RunMode::Explore => altmodes::explore::entry_point().map_err(AppError::Explore),
    }
}
