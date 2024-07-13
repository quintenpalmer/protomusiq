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

    match (
        game_library_state.get_gba_prefix_path(),
        game_library_state.get_gba_rom_paths(),
    ) {
        (Some(rom_prefix), Some(rom_paths)) => {
            let console = musiqcore::model::gl::consoles::GameConsole::GameBoyAdvance;

            for game in rom_paths.iter() {
                let source_image_path = game.image_path.clone();

                let dest_prefix = config_state.games.clone().unwrap().image_path;

                let dest_console_piece = path::PathBuf::from(console.full_name());

                let dest_base_name = game
                    .path
                    .strip_prefix(rom_prefix)
                    .unwrap()
                    .with_extension("png");

                let dest_image_path = dest_prefix.join(dest_console_piece).join(dest_base_name);

                println!("cp {:?} {:?}", source_image_path, dest_image_path);
            }
        }
        _ => println!("skipping gbas"),
    }
}
