# Recall

Recall is a command line app to help recall information.

## Building
`cargo build --release`

This outputs the binary which can be found in targets/release/recall

## Running
`RECALL_PATH=./test/test_dir cargo run [args]`


## Usage
- Commands - Able to dig deeper with more arugments
    - `recall tmux` - Outputs all things about tmux
    - `recall tmux layouts` - Would just contain things about tmux layouts
    - `recall vim surround`
    - `recall grep`

## Ideas
- At the moment there isn't a plan to be able to add new notes so the following command will just open the file in vim.
    - `recall -o`
- A quick way to backup would be nice, aka to git. Maybe it auto trys to pull the most recent changes.
- Use a git repo as storage. 
    - On launch try to pull most recent changes
    - If any changes, warn user and tell them to push changes. Offer to do it for them?
    - Need a way to specify repo.
        - Maybe we just specify a directory (possibly an env variable)
        - The specified directory should have git enabled so we can just invoke commands
