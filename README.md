## Description

md-strip is a command-line tool that strips links from Markdown, replacing each link with its title.

This can be useful as a pre-processing step before conversion to other printable formats such as PDF or DOCX by tools such as [pandoc](https://pandoc.org/).

## Installation

```shell
cargo install md-strip
```

## Usage

```shell
md-strip -i in.md -o out.md
```
