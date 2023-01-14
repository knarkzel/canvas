# canvas

```
$ cargo install --git https://github.com/knarkzel/canvas
$ canvas
Usage: canvas <COMMAND>

Commands:
  login        Login and store token
  assignments  Fetch assignments from Canvas
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Examples

```
$ canvas assignments
IKT204-G 23V Datakommunikasjon
╭─────────────────────────────────────────────────────────┬────────────┬───────────╮
│ Name                                                    ┆ Date       ┆ Days left │
╞═════════════════════════════════════════════════════════╪════════════╪═══════════╡
│ Assignment 1 - Chapter 1 Review Questions and Problems  ┆ 2023-02-03 ┆ 19        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 2 - Chapter 2 Review Questions and Problems  ┆ 2023-02-17 ┆ 33        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 3 - Wireshark HTTP                           ┆ 2023-02-17 ┆ 33        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 4 - UDP and TCP Socket Programming           ┆ 2023-04-23 ┆ 98        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 5 - Chapter 3 Review Questions and Problems  ┆ 2023-03-03 ┆ 47        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 6 - Chapter 8 Review Questions and Problems  ┆ 2023-03-10 ┆ 54        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 7 - SSL Socket Programming                   ┆ 2023-04-23 ┆ 98        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 8 - Chapter 4 Review Questions and Problems  ┆ 2023-03-24 ┆ 68        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 9 - Wireshark DHCP                           ┆ 2023-03-24 ┆ 68        │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌┤
│ Assignment 10 - Chapter 5 Review Questions and Problems ┆ 2023-04-17 ┆ 92        │
╰─────────────────────────────────────────────────────────┴────────────┴───────────╯
IKT206-G 23V DevOps
╭───────────────────────────────┬────────────┬───────────╮
│ Name                          ┆ Date       ┆ Days left │
╞═══════════════════════════════╪════════════╪═══════════╡
│ Assignment 1 - Setup/Bootcamp ┆ 2023-01-20 ┆ 5         │
╰───────────────────────────────┴────────────┴───────────╯
```
