## Audiobook Tagger
CLI tool to prepare audiobook files by changing metadata and combining multiple mp3 files into one m4b.

### Installation
Installation/updates via Cargo (requires the [Rust toolchain](https://rustup.rs/)):
```
cargo install --git https://github.com/JSogaard/audiobook_tagger
```
### Commands
```
show-tags           Show common ID3 tags from files
number-files        Update the track number tag of each file with a sequential number
number-file-titles  Update the title tag of each file with a name based on a naming scheme, replacing '%n' with a sequential number
change-title        Change the title tag of each specified file to the given title
change-author       Change the author tag of each specified file to the given author name
change-narrator     Change the narrator (composer) tag of each specified file to the given name
change-tag          Change a specified ID3 tag of each file to the given value
combine-files       Combine multiple audio files into a single file, with the input files as chapter markers
show-chapters       Show the embedded chapters in an audiobook file (e.g. m4b or mp4)
chapters-to-toml    Reads embedded chapters from audiobook file and outputs TOML to file or stdout
toml-to-chapters    Reads TOML-file with chapters and writes them to an audiobook file
example-toml        Outputs an example TOML to stdout
```
