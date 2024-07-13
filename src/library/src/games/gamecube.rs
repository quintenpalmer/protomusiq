use std::fs;
use std::io;
use std::path;

pub fn scan_for_gamecube_rom_files<P: AsRef<path::Path>>(
    path: &P,
) -> io::Result<Vec<path::PathBuf>> {
    scan_for_gamecube_rom_files_helper(&path, &path)
}

fn scan_for_gamecube_rom_files_helper<O: AsRef<path::Path>, P: AsRef<path::Path>>(
    orig_prefix: &O,
    path: &P,
) -> io::Result<Vec<path::PathBuf>> {
    let mut path_info_list = Vec::new();

    for path in fs::read_dir(path)? {
        let path = path?;
        if path.file_type()?.is_dir() {
            path_info_list.append(&mut scan_for_gamecube_rom_files_helper(
                orig_prefix,
                &path.path(),
            )?);
        }
        if path.file_type()?.is_file() {
            match path
                .path()
                .extension()
                .map(|a| a.to_str().map(|x| x.to_lowercase()))
            {
                Some(Some(ext)) => match ext.as_str() {
                    "rvz" => path_info_list.push(path.path().to_path_buf()),
                    _ => eprintln!("skipping non-gamecube file"),
                },
                _ => eprintln!("skipping file with extension we didn't know what to do with"),
            }
        }
    }

    Ok(path_info_list)
}
