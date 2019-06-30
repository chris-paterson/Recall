# Recall

Recall is a command line app to help recall information.

## Testing
`cargo test --package recall --lib generate_sub_root_dir_accepts_zero -- --exact`

## Building
`cargo build --release`

This outputs the binary which can be found in targets/release/recall

## Running
`RECALL_PATH=./test/test_dir cargo run [args]`


## Current Usage
- Commands - Able to dig deeper with more arugments
    - `recall tmux` - Outputs all things about tmux
    - `recall tmux layouts` - Would just contain things about tmux layouts
    - `recall vim surround`
    - `recall grep`


### New
Syntax: `recall -n swift class`

If `swift` or `class` not exist, the program should create the directories, files, and insert the headings of the files. 

### Editing
Syntax: `recall -e swift class`

This will open the `class` file for editing (likely using vim for now)

If the directory doesn't exit, `create` will be run first and then edit


### Deleting
Syntax: `recall -d tmux layouts main`

- This deletes all children of that directory too
- Confirmation required

## Future Usage

### Merge
Syntax: `recall -m tmux layouts main -t tmux layouts`

- `-m` merge
- `-t` target

Puts the contents and children into the target

### Backup (Maybe, if not automatic)
Syntax: `recall -b`

The files are backed by git

Even if we backup automatically we may want to include this to manually sync in case of no network at time of run


## Ideas
- A quick way to backup would be nice, aka to git. Maybe it auto trys to pull the most recent changes.
- Use a git repo as storage. 
    - On launch try to pull most recent changes
    - If any changes, warn user and tell them to push changes. Offer to do it for them?
    - Need a way to specify repo.
        - Maybe we just specify a directory (possibly an env variable)
        - The specified directory should have git enabled so we can just invoke commands


## TODO

- Print example usage for command attempted, not just the generic one
