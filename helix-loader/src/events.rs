use helix_event::{events, register_event};
use std::path::PathBuf;

events! {
    CurrentWorkingDirDidChange<'a> { cwd: &'a PathBuf }
}

pub fn register() {
    register_event::<CurrentWorkingDirDidChange>();
}
