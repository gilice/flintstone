# 🪨 flintstone

[![DeepSource](https://deepsource.io/gh/riceicetea/flintstone.svg/?label=active+issues&token=DS9hMrKlcqbwzjZiaFuAVJJC)](https://deepsource.io/gh/riceicetea/flintstone/)

`flintstone` is a simple script I wrote to make setting [Obsidian](https://obsidian.md/) as the default Markdown editor easy. It can run on all operating systems* (but the only reason it's written in Rust is that bash can't parse json without external programs). 

(* I have no idea where the `obsidian.json` file is on non-*nix OSes. Probably `AppData` or smth, but I didn't hard-code it without testing. You can provide its location with the `-p/--path` argument)

## Installation

Clone this respository, then install it using `cargo`.
```console
  $ git clone https://github.com/riceicetea/wwhatis && cd wwhatis
  $ cargo install --path .
```

or, on *Nix,

```console
$ direnv allow
$ nix build
```

You can also install the flake as a system package. 

## Usage/Examples

```console
USAGE:
    flintstone [OPTIONS] [FILES]...

ARGS:
    <FILES> List of files to open. The program will decide if it's inside an obsidian vault or not, then open accordingly

OPTIONS:
    -a, --about          Print the about page and info about used libraries
    -p, --path           a path to your obsidian.json file. 
```

On GNOME, for example, open **Files**, right click a `.md` file, select _Open With_, then choose `flintstone`. The app doesn't have an icon yet, but that is a work in progress. 

## Acknowledgements
Check `Cargo.toml` or `thirdparty/THIRDPARTY` for the full list of packages.

- **Programming language:** [Rust](https://rust-lang.org)

- [readme.so](https://readme.so), online README *ide*
- [How to write a Good README](https://bulldogjob.com/news/449-how-to-write-a-good-readme-for-your-github-project)
