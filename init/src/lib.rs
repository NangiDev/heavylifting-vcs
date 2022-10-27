use std::fs;
use std::path::Path;

/*

    How to run:
        ./hl init test_repo

    Init takes a single argument, path to folder to initialize a repo inside.
    It can not be an existing directory.
    Inside the directory '.repo' will be created.
    This will contain 'snapshots' and other vcs files we need in the future.

    .repo
    |---refs
    |   |---tags
    |---snapshots
    |   |---1
    |   |   ---.commit
    |   |---2
    |   |   ---.commit
    |   |---n
    |   |   ---.commit

*/

pub fn init(dir: Option<String>) -> String {
    let mut dir = dir.expect("path to init was not supplied");

    if Path::new(&dir).exists() {
        panic!("Directory already exists")
    }

    fs::create_dir(&dir).expect("Could not init directory provided");

    dir.push_str("/.repo");
    fs::create_dir(&dir).expect("Directory already contains a '.repo'");

    let mut refs = dir.clone();
    refs.push_str("/refs");
    fs::create_dir(&refs).expect("Directory already contains a 'refs'");

    let mut snapshots = dir.clone();
    snapshots.push_str("/snapshots");
    fs::create_dir(&snapshots).expect("Directory already contains a 'snapshots'");

    refs.push_str("/tags");
    fs::create_dir(&refs).expect("Directory already contains a 'tags'");

    println!("Init {dir} for heavy lifting");
    dir
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "path to init was not supplied")]
    fn init_panic_when_no_path_supplied() {
        init(None);
    }
}
