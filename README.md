# Reners

A super simple Rust utility to massively rename your files

### Usage

```
$ reners <starting_folder> <string_to_search> [<string_to_replace> = ""]
```

For example:

```
$ reners ./foo bar baz
```

Renames all the files inside the `foo` folder (and recursively in all its subfolders) the files  that contains `bar` in the name with `baz` (it replace `bar` with `baz`).


```
$ reners ./foo bar
```

Removes the string `bar` from all the files inside the `foo` folder.
