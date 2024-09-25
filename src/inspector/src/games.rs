use std::path::PathBuf;

use super::commands::{AppCmd, FlexibleCmd};

pub struct GameArtCopier {}

impl AppCmd for GameArtCopier {
    fn operate(&self, path: PathBuf) {
        let image_mode = musiqcore::model::gl::ImageMode::BestMatch(path);

        let config_state = musiqcore::model::app::AppConfigState::get_default();

        let game_library = musiqcore::model::gl::GameLibrary::new(&image_mode, &config_state.games);

        let game_library_state = musiqcore::model::gl::GameLibraryState::new(game_library);

        let consoles = musiqcore::model::gl::consoles::GameConsole::all();

        println!("# Please replace all \\' values with just '");
        println!("# Please replace all $ values with \\$");

        for console in consoles.iter() {
            let dest_prefix = config_state.games.clone().unwrap().image_path;

            let dest_console_piece = PathBuf::from(console.full_name());

            let dest_image_path = dest_prefix.join(dest_console_piece);

            println!("mkdir -p {:?}", dest_image_path);
        }

        for console in consoles.into_iter() {
            let maybe_prefix_roms = game_library_state.get_generic_game_and_prefix(&console);

            match maybe_prefix_roms {
                Some((rom_paths, rom_prefix)) => {
                    for game in rom_paths.iter() {
                        let source_image_path = game.get_matched_source_image_path().clone();

                        let dest_prefix = config_state.games.clone().unwrap().image_path;

                        let dest_console_piece = PathBuf::from(console.full_name());

                        let dest_base_name = game
                            .get_rom_path()
                            .strip_prefix(rom_prefix)
                            .unwrap()
                            .with_extension("png");

                        let dest_image_path =
                            dest_prefix.join(dest_console_piece).join(dest_base_name);

                        println!("cp {:?}\t{:?}", source_image_path, dest_image_path);
                    }
                }
                _ => eprintln!("skipping console: {:?}", console),
            }
        }
    }
}

pub struct ConsoleLister {}

impl FlexibleCmd for ConsoleLister {
    fn flex_operate(&self, _args: Vec<String>) {
        let consoles = musiqcore::model::gl::consoles::GameConsole::all();
        for console in consoles.into_iter() {
            println!("{}", console.full_name());
        }
    }
}

pub struct ConsoleGameLister {}

impl FlexibleCmd for ConsoleGameLister {
    fn flex_operate(&self, args: Vec<String>) {
        if args.len() != 1 {
            panic!("game lister needs <console>");
        }

        let console_str = args[0].as_str().to_string();

        let console = musiqcore::model::gl::consoles::GameConsole::from_full_name(console_str)
            .expect("unknown console provided");

        let config_state = musiqcore::model::app::AppConfigState::get_default();

        let game_library =
            musiqcore::model::gl::GameLibraryState::new(musiqcore::model::gl::GameLibrary::new(
                &musiqcore::model::gl::ImageMode::ExactMatch,
                &config_state.games,
            ));

        for game in game_library
            .get_generic_game_and_prefix(&console)
            .expect("we don't know about that console")
            .0
        {
            println!("{}", game.get_name());
        }
    }
}

pub struct GameDesktopCreator {}

impl FlexibleCmd for GameDesktopCreator {
    fn flex_operate(&self, args: Vec<String>) {
        if args.len() != 2 {
            panic!("desktop file generation needs <console> <game-rom-path>");
        }

        let console_str = args[0].as_str().to_string();
        let game_name = args[1].as_str().to_string();

        let config_state = musiqcore::model::app::AppConfigState::get_default();

        let console = musiqcore::model::gl::consoles::GameConsole::from_full_name(console_str)
            .expect("unknown console provided");

        let game_library =
            musiqcore::model::gl::GameLibraryState::new(musiqcore::model::gl::GameLibrary::new(
                &musiqcore::model::gl::ImageMode::ExactMatch,
                &config_state.games,
            ));

        let game = find_game(&game_library, &console, &game_name).expect(
            &format!(
                "could not find console {}'s game: {}",
                console.full_name(),
                game_name
            )
            .to_string(),
        );

        println!("{}", gen_game_desktop_file(&game_library, &console, game))
    }
}

fn find_game<'a>(
    game_library: &'a musiqcore::model::gl::GameLibraryState,
    console: &musiqcore::model::gl::consoles::GameConsole,
    game_name: &String,
) -> Option<Box<&'a dyn musiqcore::model::gl::GenericGame>> {
    for game in game_library
        .get_generic_game_and_prefix(&console)
        .expect("we don't know about that console")
        .0
    {
        if game.get_name() == *game_name {
            return Some(game.clone());
        }
    }
    None
}

fn gen_game_desktop_file<'a>(
    game_library: &'a musiqcore::model::gl::GameLibraryState,
    console: &musiqcore::model::gl::consoles::GameConsole,
    game: Box<&'a dyn musiqcore::model::gl::GenericGame>,
) -> String {
    let cwd = game_library
        .get_console_prefix(&console)
        .expect("no prefix dir for console");

    let (spawn_command, spawn_args) = console.get_spawn_command();

    let spawn_args_str = spawn_args.join(" ");

    vec![
        format!("[Desktop Entry]"),
        format!("Name={}", game.get_name()),
        //Comment=Play this game on Steam
        format!(
            "Path=\"{}\"",
            musiqcore::model::functions::best_effort_path_to_string(cwd)
        ),
        format!(
            "Exec={} {} \"{}\"",
            spawn_command,
            spawn_args_str,
            musiqcore::model::functions::best_effort_path_to_string(game.get_rom_path())
        ),
        format!(
            "Icon=\"{}\"",
            musiqcore::model::functions::best_effort_path_to_string(
                game.get_matched_source_image_path()
            )
        ),
        format!("Terminal=false"),
        format!("Type=Application"),
        format!("Categories=Game"),
    ]
    .join("\n")
}
