# mgit

### Setup
- `cargo new mgit --bin`
- add mgit to PATH: `export PATH=$PATH:~/src/mgit/target/release/`
- add `/.idea/` to `.gitignore`

### Functionality
- `mgit`
  - #### Must have
  - `init` Create an empty Git repository
  - `add` Add file contents to the index
  - `commit` Record changes to the repository
  - #### Nice to have
  - `status` Show the working tree status
  - `log` Show commit logs
  - `reset` Reset current HEAD to the specified state
  - #### Stretch
  - `remote`, `pull` Fetch from and integrate with another repository or a local branch
  - `branch` List, create, or delete branches

### Interesting
- view ./git/index contents with `xxd ./.git/index`
- [technical details of git index](https://github.com/git/git/blob/master/Documentation/technical/index-format.txt)
- view lines of code with `git ls-files | xargs wc -l`