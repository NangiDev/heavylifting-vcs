use std::{
    os::unix::fs,
    path::{Path, PathBuf},
};

/*

    How to run:
        ./hl tag 3 test

    Tag requires two optional arguments, snapshot id and tag.
    Before tagging we will check if working directory is a repository root by checking for the '.repo' directory to exist.
    If '.repo/refs/tags' not exists it will panic.
    If snapshot does not exists it will panic.

    Tag will create a symlink file inside '.repo/refs/tags' pointing to the snapshot id given

    TODO:
    * When run 'log', add the tags to respective log instead of outputing multiple times.

*/

pub fn tag(snapshot: Option<String>, tag: Option<String>) -> bool {
    let repo_path = PathBuf::from("./.repo");
    if !Path::new(&repo_path).exists() {
        panic!("directory is not a hl repo!")
    }

    let tags_path = PathBuf::from("./.repo/refs/tags");
    if !Path::new(&tags_path).exists() {
        panic!("Directory structure is invalid")
    }

    if snapshot.is_none() || tag.is_none() {
        panic!("Missing valid arguments")
    }

    let snap_id = snapshot.unwrap();
    let snap_path = PathBuf::from("./.repo/snapshots").join(&snap_id);
    if !Path::new(&snap_path).exists() {
        panic!("Snapshot with given id does not exist")
    }

    let alias = tag.unwrap();

    fs::symlink(&snap_path, &tags_path.join(&alias)).expect("Could not create symlink");

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "directory is not a hl repo!")]
    fn it_works() {
        assert_eq!(tag(None, None), true);
    }
}
