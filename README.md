# Rodalies Timetables CLI

Timetables of the trains of Rodalies de la Generalitat de Catalunya on the terminal!

`rodalies-cli` is written in [Rust](https://www.rust-lang.org/) and published to [crates.io](https://crates.io/crates/rodalies-cli), with it you can get timetables faster, no need to open an app nor a browser anymore.

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
$ rodalies-cli -f 79300 -t 71701
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

2. If the timetable requires a transfer you will also see it, for another day's timetable:

```bash
$ rodalies-cli -f 79300 -t 72400 -d 20
🚂 Rodalies CLI configuration: Args { search: "", from: "79300", to: "72400", day: 20, month: 5, year: 2022 }
📅 Today's date is 08/05/2022
📆 Searching timetable for date 20/05/2022
📖 Timetable with 1 transfers found:
+----------+-------+---------+-------+-------+-----------------+--------+-------+-------+-------+----------+
| Duration | Train | Station | Start | Stop  | Transfer        | Wait   | Train | Start | End   | Station  |
+----------+-------+---------+-------+-------+-----------------+--------+-------+-------+-------+----------+
|  02:07   |  R11  | Girona  | 06:19 | 07:55 | Barcelona-Sants | 14 min |  R2   | 08:09 | 08:26 | Aeroport |
|  02:17   |  R11  | Girona  | 07:09 | 08:40 | Barcelona-Sants | 29 min |  R2   | 09:09 | 09:26 | Aeroport |
|  02:12   |  R11  | Girona  | 07:44 | 09:10 | Barcelona-Sants | 29 min |  R2   | 09:39 | 09:56 | Aeroport |
|  02:17   |  R11  | Girona  | 08:09 | 09:40 | Barcelona-Sants | 29 min |  R2   | 10:09 | 10:26 | Aeroport |
|  02:07   |  R11  | Girona  | 08:49 | 10:10 | Barcelona-Sants | 29 min |  R2   | 10:39 | 10:56 | Aeroport |
|  02:17   |  R11  | Girona  | 09:39 | 11:10 | Barcelona-Sants | 29 min |  R2   | 11:39 | 11:56 | Aeroport |
|  02:07   |  R11  | Girona  | 10:19 | 11:40 | Barcelona-Sants | 29 min |  R2   | 12:09 | 12:26 | Aeroport |
|  02:44   |  RG1  | Girona  | 10:42 | 12:48 | Barcelona-Sants | 21 min |  R2   | 13:09 | 13:26 | Aeroport |
|  02:17   |  R11  | Girona  | 11:39 | 13:10 | Barcelona-Sants | 29 min |  R2   | 13:39 | 13:56 | Aeroport |
|  02:07   |  R11  | Girona  | 12:19 | 13:40 | Barcelona-Sants | 29 min |  R2   | 14:09 | 14:26 | Aeroport |
|  02:44   |  RG1  | Girona  | 12:42 | 14:48 | Barcelona-Sants | 21 min |  R2   | 15:09 | 15:26 | Aeroport |
|  02:17   |  R11  | Girona  | 13:39 | 15:10 | Barcelona-Sants | 29 min |  R2   | 15:39 | 15:56 | Aeroport |
|  02:07   |  R11  | Girona  | 14:19 | 15:40 | Barcelona-Sants | 29 min |  R2   | 16:09 | 16:26 | Aeroport |
|  02:07   |  R11  | Girona  | 14:49 | 16:10 | Barcelona-Sants | 29 min |  R2   | 16:39 | 16:56 | Aeroport |
|  02:17   |  R11  | Girona  | 15:39 | 17:10 | Barcelona-Sants | 29 min |  R2   | 17:39 | 17:56 | Aeroport |
|  02:07   |  R11  | Girona  | 16:19 | 17:40 | Barcelona-Sants | 29 min |  R2   | 18:09 | 18:26 | Aeroport |
|  02:07   |  R11  | Girona  | 16:49 | 18:10 | Barcelona-Sants | 29 min |  R2   | 18:39 | 18:56 | Aeroport |
|  02:17   |  R11  | Girona  | 17:09 | 18:40 | Barcelona-Sants | 29 min |  R2   | 19:09 | 19:26 | Aeroport |
|  02:07   |  R11  | Girona  | 17:49 | 19:10 | Barcelona-Sants | 29 min |  R2   | 19:39 | 19:56 | Aeroport |
|  02:07   |  R11  | Girona  | 18:19 | 19:40 | Barcelona-Sants | 29 min |  R2   | 20:09 | 20:26 | Aeroport |
|  02:17   |  R11  | Girona  | 18:39 | 20:10 | Barcelona-Sants | 29 min |  R2   | 20:39 | 20:56 | Aeroport |
|  02:07   |  R11  | Girona  | 19:19 | 20:40 | Barcelona-Sants | 29 min |  R2   | 21:09 | 21:26 | Aeroport |
|  02:17   |  R11  | Girona  | 20:09 | 21:40 | Barcelona-Sants | 29 min |  R2   | 22:09 | 22:26 | Aeroport |
|  01:58   |  R11  | Girona  | 20:59 | 22:20 | Barcelona-Sants | 19 min |  R2   | 22:39 | 22:57 | Aeroport |
|  02:12   |  R11  | Girona  | 21:19 | 22:40 | Barcelona-Sants | 33 min |  R2   | 23:13 | 23:31 | Aeroport |
+----------+-------+---------+-------+-------+-----------------+--------+-------+-------+-------+----------+
```

## Issues

Please, open an issue if you find any problem or you want to add a new feature. Happy to get contributions too!
