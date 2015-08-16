// deflicker.rs
// Remove flicker in timelapse sequences, either for regions or globally

extern crate getopts;
extern crate image;
extern crate threadpool;

use std::env;
use std::path::Path;
mod seqproc;

fn print_help(program: &str, opts: getopts::Options) {
    print!("{}", opts.usage(&format!("Usage: {} [options]", program)));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = getopts::Options::new();
    opts.optopt("o", "outdir", "Set output directory", "DIRECTORY");
    opts.optopt("i", "indir", "Set input directory", "DIRECTORY");
    opts.optflag("h", "help", "Show this usage message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_help(&args[0], opts);
        return;
    }
    let ifpstr = matches.opt_str("i");
    let ofpstr = matches.opt_str("o");
    if ifpstr.is_none() || ofpstr.is_none() {
        print_help(&args[0], opts);
        return;
    }
    let imgseq = match seqproc::load_seq_directory(&Path::new(&ifpstr.unwrap())) {
        Ok(v) => v,
        Err(e) => panic!("{:?}", e),
    };
    let seq_luma = seqproc::get_seq_luma(imgseq);
    for n in seq_luma {
        print!("{}\n", n);
    }
}
