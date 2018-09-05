# mgit

### Setup
- `cargo new mgit --bin`
- add mgit to PATH: `export PATH=$PATH:~/src/mgit/target/release/`
- add `/.idea/` to `.gitignore`

### Functionality
- `mgit`
  - [x] `init` Create an empty Git repository
  - [x] `add` Add file contents to the index
  - [x] `commit` Record changes to the repository
  - [x] `status` Show the working tree status
  - [x] `cat-file -p` Show the decompressed contents of an mgit object
  - [x] `log` Show commit logs


### Interesting
- view ./git/index contents with `xxd ./.git/index`
- [technical details of git index](https://github.com/git/git/blob/master/Documentation/technical/index-format.txt)
- view lines of code with `git ls-files | xargs wc -l`