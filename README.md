<div align="center">
<h1>clyp</h1>
  <a href="https://travis-ci.com/gabrielbarker/clyp">
    <img src="https://travis-ci.com/gabrielbarker/clyp.svg?branch=main"/>
  </a>
  <a href="https://www.rust-lang.org/">
    <img src="https://img.shields.io/badge/language-rust-%23dea584"/>
  </a>
  <a href="https://github.com/gabrielbarker/clyp/releases/latest">
    <img src="https://img.shields.io/github/v/release/gabrielbarker/clyp?color=ff69b4"/>
  </a>
  <a href="https://github.com/gabrielbarker/clyp/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/gabrielbarker/clyp" />
  </a>

<b>A cli tool that increases the power of the clipboard</b>

<a href="#installation">Installation</a> •
<a href="#using-clyp">Using Clyp</a> •
<a href="#license">License</a>

</div>
<br>

_Clyp is a tool that allows you to save snippets of text that you will use frequently, or that you briefly need to keep track of. Clyp helps to alleviate the problem of needing to keep multiple things in your clipboard at once. Clyp has a minimal API that makes it lightweight and easy to use. Save as many clyps as you like, under different alias', or deposit things temporarily into the default clyp for an even quicker interaction._

# Installation

The latest release contains an executable suitable for macOS. Simply download the latest release, extract it, and place the binary in a directory in \$PATH.

# Using Clyp

To use clyp, you either need to have something in your clipboard that you want to _save_, or a saved clyp that you want to _read_.

## Saving

To save the current contents of the clipboard to a clyp, simply run

```
clyp -s test-clyp
```

where `test-clyp` is the name of the clyp. The name can be omitted for ease of use, e.g.

```
clyp -s
```

wherby the contents of the clipboard will be saved to the `.default` clyp.

## Reading

To read a clyp's contents and write it to the clipboard, simply run

```
clyp -r test-clyp
```

where `test-clyp` is the name of the clyp that you want to read. The name can be omitted, e.g.

```
clyp -r
```

wherby the contents of the `.default` clyp will be saved to the clipboard.

## Listing

The current saved clyps can be listed by running

```
clyp -l
```

## Clearing

The current saved clyps can be cleared by running

```
clyp -c
```

## Help

This information can all be found just by running

```
clyp -h
```

# License

[MIT](https://github.com/gabrielbarker/clyp/blob/main/LICENSE)
