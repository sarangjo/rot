# `rot`

Convert a word into a lovely rotated all-caps square. `rot` prints to the command line, `rotclip` copies the output to the system clipboard.

## Usage

```
$ cargo build
$ cargo install --path .
```

Add `$HOME/.cargo/bin` to your system PATH.

## Example

```
$ rot hello
H E L L O
E L L O H
L L O H E
L O H E L
O H E L L
$ rotclip goodbye       # copies to clipboard 
```

## See Also

[`bash` implementation](https://github.com/sarangjo/dotfiles/blob/main/bash/.bash_aliases#L70)
