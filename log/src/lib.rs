use std::{
    fs,
    path::{Path, PathBuf},
};

/*

    How to run:
        ./hl log

    This comment just outputs all commits metadata.
    Metadata from the '.commit' file.

    If '.commit' file does not exists it will output a warning

    Ex:
    id: 4
    user: nangi
    date: 2022-10-24_23:54:46
    comment: 'This is a comment'

*/

pub fn log() -> bool {
    let repo_path = PathBuf::from("./.repo");
    if !Path::new(&repo_path).exists() {
        panic!("directory is not a hl repo!")
    }

    let snap_path = repo_path.join("snapshots");
    if !Path::new(&snap_path).exists() {
        panic!("snapshot directory does not exist")
    }

    let tag_path = repo_path.join("refs/tags");
    if !Path::new(&snap_path).exists() {
        panic!("tags directory does not exist")
    }

    let mut snapshots: Vec<PathBuf> = fs::read_dir(&snap_path)
        .expect("Could not read the snapshots")
        .map(|r| r.unwrap().path())
        .collect();

    let mut tags: Vec<PathBuf> = fs::read_dir(&tag_path)
        .expect("Could not read the tags")
        .map(|r| fs::read_link(r.unwrap().path()).unwrap())
        .collect();

    snapshots.append(&mut tags);

    snapshots.sort();

    for entry in snapshots {
        let contents =
            fs::read_to_string(entry.join(".commit")).expect("Could not read .commit file");
        println!("{contents}");
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "directory is not a hl repo!")]
    fn it_works() {
        assert_eq!(log(), true);
    }
}
