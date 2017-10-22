# verticalize-text-rs

> Change horizontal texts into vertical

##### inspired by [hor2vec](https://github.com/M157q/hor2vec) and [tate](https://github.com/mattn/tate)

### Usage

```
$ verticalize -h

USAGE:

  verticalize [-h] [-s SEP] [--ld {l2r,r2l}] [--wd {t2b,b2t}] [--nr] <FILEPATH>

FLAGS:
        --no-rotate    With this option given, verticalize won't rotate the input
    -h, --help         Prints help information. Use --help for more details.
    -V, --version      Prints version information

OPTIONS:
        --line-direction <LINE_DIRECTION>    The reading direction of each line. [default: l2r]  [values: l2r, r2l]
    -s, --seprator <SEPARATOR>               The seperator between lines. Default is '', you can use ' ', '|' or any other strings. [default: '']
        --word-direction <WORD_DIRECTION>    The reading direction of each word/character. [default: t2b]  [values: t2b, b2t]

ARGS:
    <FILEPATH>    The file you'd like to change the words into vertical
```