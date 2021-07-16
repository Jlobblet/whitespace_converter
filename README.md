# whitespace_converter

A command line tool for converting newlines, tabs, and spaces.

```
wsc 0.1.0
[w]hite[s]pace [c]onverter

Convert whitespace in a given file and output to stdout.

USAGE:
    wsc [OPTIONS] <input>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
    -i, --indentation <indentation-style>    
            Whether to convert indentation to tabs or spaces.
            
            Pass [tabs|spaces]=number to specify how many spaces there should be per tab during conversion.
            
            e.g. wsc README.md -i tabs=4
            
            wsc README.md -i spaces=4
    -n, --newline <target-newline>           
            Whether to convert to LF or CRLF line endings.
            
            If nothing is passed, line endings are not converted. Pass LF for \n and CRLF for \r\n.
            
            e.g wsc README.md -n LF
            
            wsc README.md -n CRLF

ARGS:
    <input>    
            Path to input file to convert.
            
            Only a single file is supported.
```
