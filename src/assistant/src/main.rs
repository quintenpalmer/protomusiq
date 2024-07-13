use std::env;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("must provide the source image path");
        return;
    }

    let source_image_path = Some(path::PathBuf::from(args[1].clone()));

    let config_state = musiqcore::model::app::AppConfigState::get_default();

    let game_library =
        musiqcore::model::gl::GameLibrary::new(&source_image_path, &config_state.games);

    let game_library_state = musiqcore::model::gl::GameLibraryState::new(game_library);

    let consoles = musiqcore::model::gl::consoles::GameConsole::all();

    for console in consoles.into_iter() {
        let maybe_prefix_roms = game_library_state.get_generic_game_and_prefix(&console);

        match maybe_prefix_roms {
            Some((rom_paths, rom_prefix)) => {
                for game in rom_paths.iter() {
                    let source_image_path = game.get_matched_source_image_path().clone();

                    let dest_prefix = config_state.games.clone().unwrap().image_path;

                    let dest_console_piece = path::PathBuf::from(console.full_name());

                    let dest_base_name = game
                        .get_rom_path()
                        .strip_prefix(rom_prefix)
                        .unwrap()
                        .with_extension("png");

                    let dest_image_path = dest_prefix.join(dest_console_piece).join(dest_base_name);

                    println!("cp {:?}\t{:?}", source_image_path, dest_image_path);
                }
            }
            _ => eprintln!("skipping console: {:?}", console),
        }
    }
}
