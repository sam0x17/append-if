# append-if

![Crates.io](https://img.shields.io/crates/v/append-if) ![Crates.io](https://img.shields.io/crates/l/append-if)

A CLI utility for appending text to files based on simple conditional logic
about the contents of those files. Particularly useful for appending text to
.bashrc and other places without accidentally appending the same thing multiple
times.

## Installation

```
cargo install append-if
```

## Usage

```bash
append-if --mode missing --path ~/.bashrc --pattern '$HOME/.rvm/bin' --append '
# Add RVM to PATH for scripting. Make sure this is the last PATH variable change.
export PATH="$PATH:$HOME/.rvm/bin"

'
```

## CLI Options
```
$ append-if --help
append-if 0.1.0
A utility for appending text to files based on simple conditional logic. Designed to be highly
useful in shell scripts

USAGE:
    append-if [OPTIONS] --path <PATH> --append <APPEND>

OPTIONS:
    -a, --append <APPEND>            Text to append to the end of the file if conditions are met
    -c, --check-path <CHECK_PATH>    Path of the file to check. Defaults to path
    -h, --help                       Print help information
    -m, --mode <MODE>                Mode used for conditional logic. Choices are 'missing',
                                     'present', or 'always' [default: Missing]
    -p, --path <PATH>                Path of the file you want to append to
        --pattern <PATTERN>          Pattern to match against. Defaults to what we are appending.
                                     Can be a string or regular expression. If an invalid regular
                                     expression is provided, the input will be treated as a string
                                     we are searching for in the target file
    -v, --verbose                    Flag to enable verbose output
    -V, --version                    Print version information
```
