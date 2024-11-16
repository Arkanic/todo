# todo
Features both CLI & GUI modes, uses small sqlite database stored in default local folder for persistence. Made to be small, simple, and to just... work without any hassle, for personal use.

## Usage
### CLI
- `todo`, `todo list` (no options) lists unfinished tasks
- `todo all`, `todo list all` lists all tasks, including finished tasks
- `todo add <XXX>` add a task to the list
- `todo done <ID>`, `todo finish <ID>` mark a task as done
- `todo redo <ID>` unmark as task as done
- `todo delete <ID>` delete a task entirely

### GUI
Launch via `todo gui`, provides a simple immediate mode UI for interaction.

## Install
Either download a binary from the releases page, or follow the build steps below.

## Build
`cargo build --release`, use binary located in target/release as you wish afterward - there is a bunch of optimizations for release builds so expect your compiler to fry itself for a bit making the final binary



## Potential improvements
- Switch to a non-immediate-mode gui library to save some resources?
- Reduce binary size (current release binary is 6.2MB), could potentially provide a CLI-only compile mode?
- Multiple todo lists, control default DB location & remember between uses - do not harrass user with this unless they specifically go looking for it
- Desktop widget? is that even feasible?

## Contributing
Would be appreciated
