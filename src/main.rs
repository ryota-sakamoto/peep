extern crate getopts;
extern crate termion;

use getopts::Options;
use std::env;
use std::io;
use std::process;

extern crate peep;
use peep::app::App;

fn print_usage(prog: &str, version: &str, opts: &Options) {
    let brief = format!(
        "{p} {v}\n\nUsage: {p} [OPTION]... [FILE]",
        p = prog,
        v = version
    );
    println!("{}", opts.usage(&brief));
    println!(
        "Commands on Normal Mode:
    (num)j         Scroll down
    (num)k         Scroll up
    (num)d         Scroll down half page
    (num)u         Scroll up half page
    (num)f         Scroll down a page
    (num)b         Scroll up a page
    (num)l         Scroll horizontally right
    (num)h         Scroll horizontally left
    (num)L         Scroll horizontally right half page
    (num)H         Scroll horizontally left half page
    0              Go to the beggining of line
    $              Go to the end of line
    g              Go to the beggining of file
    G              Go to the end of file
    [num]g [num]G  Go to line [num]
    /pattern       Search forward in the file for the regex pattern
    n              Search next
    N              Search previous
    q Ctrl-c       Quit
    (num)+         Increment screen height
    (num)-         Decrement screen height
    [num]=         Set screen height to [num]
    #              Toggle line number printing
    !              Toggle text wrap
    ESC            Cancel
    F              Toggle to follow mode

Commands on Following Mode:
    /pattern       Highlight the regex pattern
    q Ctrl-c       Quit
    (num)+         Increment screen height
    (num)-         Decrement screen height
    [num]=         Set screen height to [num]
    #              Toggle line number printing
    !              Toggle text wrap
    ESC            Cancel
    F              Toggle to normal mode"
    );
}

fn print_version(prog: &str, version: &str) {
    println!("{} {}", prog, version);
}

fn run() -> io::Result<()> {
    let prog = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let args: Vec<String> = env::args().skip(1).collect();

    let mut opts = Options::new();
    opts.optopt("n", "lines", "set height of pane", "LINES")
        .optopt("t", "tab-width", "set tab width", "WIDTH")
        .optflag("N", "print-line-number", "print line numbers")
        .optflag("f", "follow", "output appended data as the file grows")
        .optflag("w", "wrap", "wrap text line")
        .optflag("h", "help", "show this usage")
        .optflag("v", "version", "show version");

    let matches = opts
        .parse(args)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

    if matches.opt_present("h") {
        print_usage(prog, version, &opts);
        return Ok(());
    }

    if matches.opt_present("v") {
        print_version(prog, version);
        return Ok(());
    }

    let file_path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        if termion::is_tty(&io::stdin()) {
            // not find file name and pipe input
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Error. Missing filename (\"{} --help\" for help)", prog),
            ));
        }
        "-".to_owned()
    };

    let mut app: App = Default::default();
    app.show_linenumber = matches.opt_present("N");
    app.follow_mode = matches.opt_present("f");
    app.wraps_line = matches.opt_present("w");
    if let Ok(Some(nlines)) = matches.opt_get::<u16>("n") {
        app.nlines = nlines;
    }
    if let Ok(Some(tab_width)) = matches.opt_get::<u16>("t") {
        app.tab_width = tab_width;
    }

    app.run(&file_path)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
