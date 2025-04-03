# New York Time X-Word Downloader

CLI utility for bulk downloading NYT X-Word PDFs

## Usage 

```console
Usage: nyt_xword_downloader [OPTIONS] --token <TOKEN> [START] [END]

Arguments:
  [START]  
  [END]    

Options:
  -t, --token <TOKEN>  
  -d, --dest <DEST>    
  -s, --skip-sunday    
  -h, --help           Print help
```

### Arguments

#### `START`/`END`

These positional arguments should be formatted `<full-year>-<2digit-month>-<2digit-day>` and
represent an inclusive range. That means `2024-01-01 2024-12-31` would be all of the days in
the year 2024.

#### `TOKEN`

If not provided with the instructions below, it will try and use the
[`rookie`](https://crates.io/crates/rookie) crate to look it up from your browser.

This is the `NYT-S` cookie from a valid login to the new york times puzzle archive. You can find
your token by logging into [The NYT website](https://www.nytimes.com/crosswords/archive) in any
browser and then opening the developer console. Instructions for Firefox are provided below but you
should be able to follow a similar flow in any major browser.

##### Firefox

Navigate to the `Storage` tab and then the `Cookies` expand on the left, under the
`https://www.nytimes.com` entry you should see a value `NYT-S`, click that then on the right panel
that opens, right click on the top line and select `Copy`

#### `--skip-sunday`/`-s`

The Sunday puzzle is a much larger PDF and can be a pain to do by hand, this will skip all sundays
through the time period provided.

#### `DEST`

The destination to store the PDFs (defaulting to the current working directory). The directory
structure will be

```console
<root>
└── <year>
    └── <2digit-month>
        └── <2digit-day>.pdf
```
