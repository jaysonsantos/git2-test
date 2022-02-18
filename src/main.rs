use std::{
    sync::{Arc, Mutex},
    thread::spawn,
};

use color_eyre::{eyre::Context, install, Result};
use git2::Repository;

fn main() -> Result<()> {
    install()?;
    let repo = Repository::clone("https://github.com/jaysonsantos/bunderwar", "bunderwar")
        .wrap_err("failed to clone")?;

    let revwalk = repo.revwalk()?;
    let commits: Vec<_> = revwalk.take(10).collect();

    let mut handles = Vec::with_capacity(10);

    let repo = Arc::new(Mutex::new(repo));

    for commit in commits {
        let commit_oid = commit.wrap_err("failed to read revlog")?;
        let repo = repo.clone();
        handles.push(spawn(move || {
            let repo = repo.lock().expect("failed to get a lock for the repo");
            let commit = repo.find_commit(commit_oid).expect("failed to get commit");
            String::from_utf8_lossy(commit.message_bytes()).into_owned()
        }))
    }
    Ok(())
}
