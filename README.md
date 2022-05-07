# Rodalies Timetables CLI

Timetables of the trains of Rodalies de la Generalitat de Catalunya on the terminal!

With this CLI written in Rust you can get timetables faster, no need to open an app nor a browser anymore.

## Installation

There will be more installation methods in the future, but for now you can:

### Cargo install

1. You will need to have rust on your system, if not having it yet go to [installation page](https://www.rust-lang.org/tools/install)

2. Now use cargo to install it on your rust bin folder

```bash
$ cargo install rodalies-cli
```

### Manual build

1. You will need to have rust on your system, if not having it yet go to [installation page](https://www.rust-lang.org/tools/install)

2. Clone this repository

```bash
$ git clone https://github.com/gerardcl/rodalies-cli.git
```

3. Enter to the new cloned repo's folder and:

   * Use cargo to run it:

   ```bash
   $ cargo run -- --help
   ```

   * Or build it and move the binary to your bin's folder:

   ```bash
   $ cargo build --release
   $ cp target/release/rodalies-cli <to your preferred bin folder loctation>
   ```

## Usage

Once you have `rodalies-cli` installed just run the help command to understand what can you do:

```bash
$ rodalies-cli --help
rodalies-cli 0.1.0
Gerard C.L. <gerardcl@gmail.com>
CLI for searching train timetables of the trains of Rodalies de la Generalitat de Catalunya

USAGE:
    rodalies-cli [OPTIONS]

OPTIONS:
    -d, --day <DAY>          The day value of the date to search for [default: 7]
    -f, --from <FROM>        The origin's station ID [default: ]
    -h, --help               Print help information
    -m, --month <MONTH>      The month value of the date to search for [default: 5]
    -s, --search <SEARCH>    Search the ID of a given station's name pattern, to later use it on
                             your origin or destination [default: ]
    -t, --to <TO>            The destinations's station ID [default: ]
    -V, --version            Print version information
    -y, --year <YEAR>        The year value of the date to search for [default: 2022]
```

Long story short: you will need to use the stations' IDs to define your origins and destinations. And, to know such IDs you just need to search for them by searching text patterns.

### Example

1. First search the IDs of your origin and destination stations:

```bash
$ rodalies-cli -s gir
🚂 Rodalies CLI configuration: Args { search: "gir", from: "", to: "", day: 7, month: 5, year: 2022 }
📅 Today's date is 07/05/2022
🔍 Listing the stations' IDs of the stations' names containing: 'gir'
+--------------+------------+
| Station name | Station ID |
+--------------+------------+
| Girona       |   79300    |
+--------------+------------+

$ rodalies-cli -s si
🚂 Rodalies CLI configuration: Args { search: "si", from: "", to: "", day: 7, month: 5, year: 2022 }
📅 Today's date is 07/05/2022
🔍 Listing the stations' IDs of the stations' names containing: 'si'
+------------------------+------------+
| Station name           | Station ID |
+------------------------+------------+
| Cerdanyola-Universitat |   72503    |
| Sils                   |   79202    |
| Sitges                 |   71701    |
+------------------------+------------+
```

2. Search for today's timetable:

```bash
$ rodalies-cli -f 79300 -t 79202
🚂 Rodalies CLI configuration: Args { search: "", from: "79300", to: "79202", day: 7, month: 5, year: 2022 }
📅 Today's date is 07/05/2022
📆 Searching timetable for date 07/05/2022
📖 Timetable with 0 transfers found:
+----------+-------+---------+-------+-------+---------+
| Duration | Train | Station | Start | End   | Station |
+----------+-------+---------+-------+-------+---------+
|  00:19   |  R11  | Girona  | 06:19 | 06:38 |  Sils   |
|  00:18   |  R11  | Girona  | 07:09 | 07:27 |  Sils   |
|  00:19   |  R11  | Girona  | 08:09 | 08:28 |  Sils   |
|  00:16   |  R11  | Girona  | 08:49 | 09:05 |  Sils   |
|  00:19   |  R11  | Girona  | 09:39 | 09:58 |  Sils   |
|  00:16   |  R11  | Girona  | 10:19 | 10:35 |  Sils   |
|  00:19   |  R11  | Girona  | 11:39 | 11:58 |  Sils   |
|  00:16   |  R11  | Girona  | 12:19 | 12:35 |  Sils   |
|  00:19   |  R11  | Girona  | 13:39 | 13:58 |  Sils   |
|  00:16   |  R11  | Girona  | 14:19 | 14:35 |  Sils   |
|  00:19   |  R11  | Girona  | 15:39 | 15:58 |  Sils   |
|  00:16   |  R11  | Girona  | 16:19 | 16:35 |  Sils   |
|  00:19   |  R11  | Girona  | 17:39 | 17:58 |  Sils   |
|  00:16   |  R11  | Girona  | 18:19 | 18:35 |  Sils   |
|  00:16   |  R11  | Girona  | 19:19 | 19:35 |  Sils   |
|  00:19   |  R11  | Girona  | 20:49 | 21:08 |  Sils   |
|  00:18   |  R11  | Girona  | 21:23 | 21:41 |  Sils   |
+----------+-------+---------+-------+-------+---------+
```

## Issues

Please, open an issue if you find any problem or you want to add a new feature. Happy to get contributions too!
