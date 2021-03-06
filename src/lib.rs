//! # Markdown to HTML Converter
//! Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.

extern crate clap;
extern crate html_minifier;
extern crate comrak;
extern crate htmlescape;

#[macro_use]
extern crate lazy_static_include;

#[macro_use]
extern crate lazy_static;

use std::env;
use std::path::{Path, PathBuf};
use std::fs;

use clap::{App, Arg};

use comrak::{markdown_to_html, ComrakOptions};

use html_minifier::HTMLMinifier;

lazy_static_include_str!(MarkdownCSS, "resources/github-markdown.css");
lazy_static_include_str!(FontCJK, "resources/font-cjk.css");
lazy_static_include_str!(FontCJKMono, "resources/font-cjk-mono.css");
lazy_static_include_str!(Github, "resources/github.css");

lazy_static_include_str!(Webfont, "resources/webfont.js");
lazy_static_include_str!(JQuerySlim, "resources/jquery-slim.min.js");
lazy_static_include_str!(WebfontLoader, "resources/webfontloader.min.js");
lazy_static_include_str!(HighlightCode, "resources/highlight-code.js");
lazy_static_include_str!(MathJax, "resources/mathjax.min.js");
lazy_static_include_str!(MathJaxConfig, "resources/mathjax-config.js");

lazy_static_include_str!(Highlight, "resources/highlight.min.js.html");

// TODO -----Config START-----

const APP_NAME: &str = "Markdown to HTML Converter";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");


#[derive(Debug)]
pub struct Config {
    pub markdown_path: String,
    pub html_path: Option<String>,
    pub title: Option<String>,
    pub no_safe: bool,
    pub no_highlight: bool,
    pub no_mathjax: bool,
    pub no_cjk_fonts: bool,
    pub css_path: Option<String>,
    pub highlight_js_path: Option<String>,
    pub highlight_css_path: Option<String>,
    pub mathjax_js_path: Option<String>,
}

impl Config {
    pub fn from_cli() -> Result<Config, String> {
        let arg0 = env::args().next().unwrap();
        let arg0 = Path::new(&arg0).file_stem().unwrap().to_str().unwrap();

        let examples = vec![
            "/path/to/file.md                          # Convert /path/to/file.md to /path/to/file.html, titled \"file\"",
            "/path/to/file.md -o /path/to/output.html  # Convert /path/to/file.md to /path/to/output.html, titled \"output\"",
            "/path/to/file.md -t \"Hello World!\"        # Convert /path/to/file.md to /path/to/file.html, titled \"Hello World!\"",
        ];

        let matches = App::new(APP_NAME)
            .version(CARGO_PKG_VERSION)
            .author(CARGO_PKG_AUTHORS)
            .about(format!("Markdown to HTML Converter is a free tool for converting a Markdown file to a single HTML file with built-in CSS and JS.\n\nEXAMPLES:\n{}", examples.iter()
                .map(|e| format!("  {} {}\n", arg0, e))
                .collect::<Vec<String>>()
                .concat()
            ).as_str()
            )
            .arg(Arg::with_name("TITLE")
                .required(false)
                .long("title")
                .short("t")
                .help("Specifies the title of your HTML file.")
                .takes_value(true)
                .display_order(1)
            )
            .arg(Arg::with_name("MARKDOWN_PATH")
                .required(true)
                .help("Specifies the path of your Markdown file.")
                .takes_value(true)
            )
            .arg(Arg::with_name("HTML_PATH")
                .required(false)
                .long("html-path")
                .short("o")
                .help("Specifies the path of your HTML file.")
                .takes_value(true)
                .display_order(2)
            )
            .arg(Arg::with_name("NO_SAFE")
                .required(false)
                .long("no-safe")
                .help("Allows raw HTML and dangerous URLs.")
                .display_order(3)
            )
            .arg(Arg::with_name("NO_HIGHLIGHT")
                .required(false)
                .long("no-highlight")
                .help("Not allow to use highlight.js.")
                .display_order(4)
            )
            .arg(Arg::with_name("NO_MATHJAX")
                .required(false)
                .long("no-mathjax")
                .help("Not allow to use mathjax.js.")
                .display_order(5)
            )
            .arg(Arg::with_name("NO_CJK_FONTS")
                .required(false)
                .long("no-cjk-fonts")
                .help("Not allow to use CJK fonts.")
                .display_order(6)
            )
            .arg(Arg::with_name("CSS_PATH")
                .required(false)
                .long("css-path")
                .help("Specifies the path of your custom CSS file.")
                .takes_value(true)
                .display_order(100)
            )
            .arg(Arg::with_name("HIGHLIGHT_JS_PATH")
                .required(false)
                .long("highlight-js-path")
                .help("Specifies the path of your custom highlight.js file.")
                .takes_value(true)
                .display_order(101)
            )
            .arg(Arg::with_name("HIGHLIGHT_CSS_PATH")
                .required(false)
                .long("highlight-css-path")
                .help("Specifies the path of your custom CSS file for highlight.js code blocks.")
                .takes_value(true)
                .display_order(102)
            )
            .arg(Arg::with_name("MATHJAX_JS_PATH")
                .required(false)
                .long("mathjax-path-path")
                .help("Specifies the path of your custom single MathJax.js file.")
                .takes_value(true)
                .display_order(103)
            )
            .after_help("Enjoy it! https://magiclen.org")
            .get_matches();

        let title = matches.value_of("TITLE").map(|s| s.to_string());

        let markdown_path = matches.value_of("MARKDOWN_PATH").unwrap().to_string();

        let html_path = matches.value_of("HTML_PATH").map(|s| s.to_string());

        let no_safe = matches.is_present("NO_SAFE");

        let no_highlight = matches.is_present("NO_HIGHLIGHT");

        let no_mathjax = matches.is_present("NO_MATHJAX");

        let no_cjk_fonts = matches.is_present("NO_CJK_FONTS");

        let css_path = matches.value_of("CSS_PATH").map(|s| s.to_string());

        let highlight_js_path = matches.value_of("HIGHLIGHT_JS_PATH").map(|s| s.to_string());

        let highlight_css_path = matches.value_of("HIGHLIGHT_CSS_PATH").map(|s| s.to_string());

        let mathjax_js_path = matches.value_of("MATHJAX_JS_PATH").map(|s| s.to_string());

        Ok(Config {
            title,
            markdown_path,
            html_path,
            no_safe,
            no_highlight,
            no_mathjax,
            no_cjk_fonts,
            css_path: css_path,
            highlight_js_path: highlight_js_path,
            highlight_css_path: highlight_css_path,
            mathjax_js_path: mathjax_js_path,
        })
    }
}

// TODO -----Config END-----

pub fn run(config: Config) -> Result<i32, String> {
    let (markdown_path, html_path, title) = {
        let markdown_path = Path::new(&config.markdown_path);

        if !markdown_path.exists() {
            return Err(format!("`{}` does not exist.", config.markdown_path));
        }

        if !markdown_path.is_file() {
            return Err(format!("`{}` is not a file.", config.markdown_path));
        }

        let file_name = markdown_path.file_name().unwrap().to_str().unwrap();
        let file_name_len = file_name.len();


        let file_stem = markdown_path.file_stem().unwrap().to_str().unwrap();
        let file_stem_len = file_stem.len();

        let file_ext = if file_name_len > file_stem_len {
            &file_name[(file_stem_len + 1)..]
        } else {
            ""
        };

        if file_ext.ne("md") && file_ext.ne("markdown") {
            return Err(format!("`{}` is not a Markdown file.", config.markdown_path));
        }

        let html_path = match &config.html_path {
            Some(html_path) => {
                let html_path = PathBuf::from(html_path);

                html_path
            }
            None => {
                let folder_path = markdown_path.parent().unwrap();

                let html_path = Path::join(folder_path, format!("{}.html", file_stem));

                html_path
            }
        };

        if html_path.exists() {
            if !html_path.is_file() {
                return Err(format!("`{}` exists and it is not a file.", html_path.to_str().unwrap()));
            }
        }

        let title = match &config.title {
            Some(title) => title,
            None => file_stem
        };

        (markdown_path, html_path, title)
    };

    let markdown = fs::read_to_string(markdown_path).map_err(|err| err.to_string())?;

    let markdown_html = {
        let mut options = ComrakOptions::default();

        if !config.no_safe {
            options.safe = true;
        }

        options.ext_autolink = true;
        options.ext_description_lists = true;
        options.ext_footnotes = true;
        options.ext_strikethrough = true;
        options.ext_superscript = true;
        options.ext_table = true;
        options.ext_tagfilter = true;
        options.ext_tasklist = true;
        options.hardbreaks = true;

        markdown_to_html(&markdown, &options)
    };

    let mut minifier = HTMLMinifier::new();

    minifier.digest("<!DOCTYPE html>").map_err(|err| err.to_string())?;
    minifier.digest("<html>").map_err(|err| err.to_string())?;

    minifier.digest("<head>").map_err(|err| err.to_string())?;
    minifier.digest("<meta charset=UTF-8>").map_err(|err| err.to_string())?;
    minifier.digest("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\">").map_err(|err| err.to_string())?;
    minifier.digest(&format!("<meta name=\"generator\" content=\"{} {} by magiclen.org\"/>", APP_NAME, CARGO_PKG_VERSION, )).map_err(|err| err.to_string())?;

    minifier.digest("<title>").map_err(|err| err.to_string())?;
    minifier.digest(&htmlescape::encode_minimal(title)).map_err(|err| err.to_string())?;
    minifier.digest("</title>").map_err(|err| err.to_string())?;

    minifier.digest("<style>").map_err(|err| err.to_string())?;
    match config.css_path {
        Some(with_css_path) => {
            let with_css = fs::read_to_string(with_css_path).map_err(|err| err.to_string())?;
            minifier.digest(&htmlescape::encode_minimal(&with_css)).map_err(|err| err.to_string())?;
        }
        None => {
            minifier.digest(&MarkdownCSS).map_err(|err| err.to_string())?;
        }
    }
    minifier.digest("</style>").map_err(|err| err.to_string())?;

    let has_code = {
        if config.no_highlight {
            false
        } else {
            markdown_html.find("</code></pre>").is_some()
        }
    };

    let has_mathjax = {
        if config.no_mathjax {
            false
        } else {
            markdown_html.find("#{{").is_some()
        }
    };

    if !config.no_cjk_fonts || !has_code {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&JQuerySlim).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    if !config.no_cjk_fonts {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&WebfontLoader).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;

        minifier.digest("<style>").map_err(|err| err.to_string())?;
        minifier.digest(&FontCJK).map_err(|err| err.to_string())?;
        minifier.digest("</style>").map_err(|err| err.to_string())?;

        minifier.digest("<style>").map_err(|err| err.to_string())?;
        minifier.digest(&FontCJKMono).map_err(|err| err.to_string())?;
        minifier.digest("</style>").map_err(|err| err.to_string())?;
    }

    if has_code {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        match config.highlight_js_path {
            Some(with_highlight_js_path) => {
                let with_highlight_js = fs::read_to_string(with_highlight_js_path).map_err(|err| err.to_string())?;
                minifier.digest(&htmlescape::encode_minimal(&with_highlight_js)).map_err(|err| err.to_string())?;
            }
            None => {
                minifier.digest(&Highlight).map_err(|err| err.to_string())?;
            }
        }
        minifier.digest("</script>").map_err(|err| err.to_string())?;

        minifier.digest("<style>").map_err(|err| err.to_string())?;
        match config.highlight_css_path {
            Some(with_highlight_css_path) => {
                let with_highlight_css = fs::read_to_string(with_highlight_css_path).map_err(|err| err.to_string())?;
                minifier.digest(&htmlescape::encode_minimal(&with_highlight_css)).map_err(|err| err.to_string())?;
            }
            None => {
                minifier.digest(&Github).map_err(|err| err.to_string())?;
            }
        }
        minifier.digest("</style>").map_err(|err| err.to_string())?;
    }

    if has_mathjax {
        minifier.digest("<script type=\"text/x-mathjax-config\">").map_err(|err| err.to_string())?;
        minifier.digest(&MathJaxConfig).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;

        minifier.digest("<script>").map_err(|err| err.to_string())?;
        match config.mathjax_js_path {
            Some(with_mathjax_js_path) => {
                let with_mathjax_js = fs::read_to_string(with_mathjax_js_path).map_err(|err| err.to_string())?;
                minifier.digest(&htmlescape::encode_minimal(&with_mathjax_js)).map_err(|err| err.to_string())?;
            }
            None => {
                minifier.digest(&MathJax).map_err(|err| err.to_string())?;
            }
        }
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    minifier.digest("</head>").map_err(|err| err.to_string())?;

    minifier.digest("<body>").map_err(|err| err.to_string())?;

    minifier.digest("<article class=\"markdown-body\">").map_err(|err| err.to_string())?;
    minifier.digest(&markdown_html).map_err(|err| err.to_string())?;
    minifier.digest("</article>").map_err(|err| err.to_string())?;

    if !config.no_cjk_fonts {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&Webfont).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    if has_code {
        minifier.digest("<script>").map_err(|err| err.to_string())?;
        minifier.digest(&HighlightCode).map_err(|err| err.to_string())?;
        minifier.digest("</script>").map_err(|err| err.to_string())?;
    }

    minifier.digest("</body>").map_err(|err| err.to_string())?;

    minifier.digest("</html>").map_err(|err| err.to_string())?;

    let minified_html = minifier.get_html();

    fs::write(html_path, minified_html).map_err(|err| err.to_string())?;

    Ok(0)
}