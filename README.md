Markdown to HTML Converter
====================

[![Build Status](https://travis-ci.org/magiclen/markdown2html-converter.svg?branch=master)](https://travis-ci.org/magiclen/markdown2html-converter)

Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.

## Help

```
EXAMPLES:
  markdown2html-converter /path/to/file.md                          # Convert /path/to/file.md to /path/to/file.html,
titled "file"
  markdown2html-converter /path/to/file.md -o /path/to/output.html  # Convert /path/to/file.md to /path/to/output.html,
titled "output"
  markdown2html-converter /path/to/file.md -t "Hello World!"        # Convert /path/to/file.md to /path/to/file.html,
titled "Hello World!"

USAGE:
    markdown2html-converter [FLAGS] [OPTIONS] <MARKDOWN_PATH>

FLAGS:
        --no-safe         Allows raw HTML and dangerous URLs.
        --no-highlight    Not allow to use highlight.js.
        --no-mathjax      Not allow to use mathjax.js.
        --no-cjk-fonts    Not allow to use CJK fonts.
    -h, --help            Prints help information
    -V, --version         Prints version information

OPTIONS:
    -t, --title <TITLE>                                Specifies the title of your HTML file.
    -o, --html-path <HTML_PATH>                        Specifies the path of your HTML file.
        --css-path <CSS_PATH>                          Specifies the path of your custom CSS file.
        --highlight-js-path <HIGHLIGHT_JS_PATH>        Specifies the path of your custom highlight.js file.
        --highlight-css-path <HIGHLIGHT_CSS_PATH>
            Specifies the path of your custom CSS file for highlight.js code blocks.

        --mathjax-path-path <MATHJAX_JS_PATH>          Specifies the path of your custom single MathJax.js file.
        --mathjax-config-path <MATHJAX_CONFIG_PATH>    Specifies the path of your custom config JS file for MathJax.js.

ARGS:
    <MARKDOWN_PATH>    Specifies the path of your Markdown file.
```

## Dependency

Markdown is converted to HTML by the [comrak](https://crates.io/crates/comrak) crate. The default stylesheet (the CSS file) is from [sindresorhus/github-markdown-css](https://github.com/sindresorhus/github-markdown-css). 

If ` ``` ` is used in the input Markdown file, the [highlight.js](https://highlightjs.org/) will be automatically embedded in the output HTML file. The preset supported languages are listed below.

|Common|Other|
|---|---|
|Apache|Go|
|Bash|Rust|
|C#|
|C++|
|CSS|
|CoffeeScript|
|Diff|
||HTML, XML|
|HTTP|
|Ini, TOML|
|JSON|
|Java|
|JavaScript|
|Makefile|
|Markdown|
|Nginx|
|Objective-C|
|PHP|
|Perl|
|Properties|
|Python|
|Ruby|
|SQL|
|Shell Session|

If `#{{` - `}}#` or `#{{{` - `}}}#` is used in the input Markdown file, the [mathjax.js](https://www.mathjax.org/) will be automatically embedded in the output HTML file. `#{{` and `}}#` are `inlineMath` delimiters. `#{{{` and `}}}#` are `displayMath` delimiters. **mathjax.js** can be generated by [pkra/MathJax-single-file](https://github.com/pkra/MathJax-single-file). The default **mathjax.js** are using the [TeX-MML-AM_CHTML](http://docs.mathjax.org/en/latest/config-files.html#the-tex-mml-am-chtml-configuration-file) configuration file.

## A Markdown Example

[The Markdown File](https://github.com/github/magiclen/blob/master/example.md)

[The HTML File](https://jsfiddle.net/magiclen/jgs324w0/)

## License

[MIT](LICENSE)