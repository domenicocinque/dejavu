# idar

> Image deduplication and removal 

A simple command-line tool for identifying and handling duplicate images within a directory based on image hashing.

This project is mostly a learning project. Every contribution is welcome.

## Installation

```
$ cargo install idar
```

## Usage

Find duplicates within a directory:

```
$ idar deduplicate /path/to/directory
```

Create a new directory with deduplicated images (must be used after `deduplicate`):

```
$ idar remove /path/to/directory
```

Show help message:

```
$ idar --help
```
