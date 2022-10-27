use chrono::Local;
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};
use whoami;

/*

    How to run:
        ./hl commit "This is a comment"

    Commit takes one optional argument, comment for the commit.
    If no argument is given it will leave an empty comment.

    Before commiting we will check if working directory is a repository root by checking for the '.repo' directory to exist.

    If 'snapshots' directory does not exist inside '.repo' it will be created.
    For each commit a new directory inside 'snapshots' will be created.
    The name of this directory is just an number that increments by one every time.

    A recursive copy of all files and directories inside your repo, excluding the '.repo' directory,
    will be copied into the current commit directory.

    Lastly a '.commit' file will be created, together with current snapshot, containing
    snapshot id, username, date and the given comment.

    Ex:
    id: 1
    user: nangi
    date: 2022-10-24_23:54:46
    comment: 'This is a comment'

    TODO:
    * Imlement an ignore file (.hlignore)
    * Use sha1 hash instead of incremental integers for snapshot directories

*/

pub fn commit(mut cur_snap: i32, comment: Option<String>) -> bool {
    let repo_path = PathBuf::from("./.repo");
    if !Path::new(&repo_path).exists() {
        panic!("directory is not a hl repo!")
    }

    let snap_path = repo_path.join("snapshots").join(cur_snap.to_string());

    let result = fs::create_dir(&snap_path);

    match result {
        Ok(_) => true,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                cur_snap += 1;
                return commit(cur_snap, comment);
            }
            other_error => {
                panic!("Problem creating snapshot directory: {:?}", other_error);
            }
        },
    };

    let work_dir = env::current_dir().expect("Could not get working directory");

    copy_dir_all(&work_dir, &snap_path).expect("Failed copy repo to snapshot");

    let mut commit_meta_data =
        fs::File::create(&snap_path.join(".commit")).expect("Cound not create .commit file");

    let user = whoami::username();
    let local = Local::now().format("%Y-%m-%d_%H:%M:%S").to_string();
    let buf = format!(
        "id: {cur_snap}\nuser: {user}\ndate: {local}\ncomment: '{}'\ntags: []",
        &comment.unwrap_or_default()
    );

    commit_meta_data
        .write_all(buf.as_bytes())
        .expect("Could not write meta data to .commit");

    true
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if name.eq_ignore_ascii_case(".repo") {
            continue;
        }
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "directory is not a hl repo!")]
    fn it_works() {
        assert_eq!(commit(1, None), true);
    }
}
