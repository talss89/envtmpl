# envtmpl

Compile Golang-style text/templates, exposing environment variables à la dockerize.

Built so we can compile our WordPress runtime images for arm64, without having to maintain the whole dockerize project.

Written as a quick hack in rust, using [gtmpl](https://docs.rs/gtmpl/latest/gtmpl/).

## Usage

```
envtmpl --target ./input-dir:./output-dir

envtmpl --help
```

Binaries will be available shortly via GitHub releases.

## Builtins

Builtin coverage is as follows:

### and
Returns the boolean AND of its arguments by returning the first empty argument or the last argument, that is, “and x y” behaves as “if x then y else x”. All the arguments are evaluated.

### call
Returns the result of calling the first argument, which must be a function, with the remaining arguments as parameters.

### eq
Returns the boolean truth of arg1 == arg2 [== arg3 …]

### ge
Returns the boolean truth of arg1 >= arg2

### gt
Returns the boolean truth of arg1 > arg2

### index
Returns the result of indexing its first argument by the following arguments. Thus “index x 1 2 3” is, in Go syntax, x[1][2][3]. Each indexed item must be a map, slice or array.

### le
Returns the boolean truth of arg1 <= arg2

### len
Returns the integer length of its argument.

### lt
Returns the boolean truth of arg1 < arg2

### ne
Returns the boolean truth of arg1 != arg2

### not
Returns the boolean negation of its single argument.

### or
Returns the boolean OR of its arguments by returning the first non-empty argument or the last argument, that is, “or x y” behaves as “if x then x else y”. All the arguments are evaluated.

### print
An implementation of golang’s fmt.Sprint

### printf
An implementation of golang’s fmt.Sprintf Limitations:

### println
An implementation of golang’s fmt.Sprintln

### urlquery
Returns the escaped value of the textual representation of its arguments in a form suitable for embedding in a URL query.

### semverCompare
Return boolean if the arg1 constraint matches the arg2 semantic version string

### atoi
Convert arg1 from string to integer

### default
If arg2 does not exist, then return arg1
