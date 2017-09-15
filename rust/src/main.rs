extern crate getopts;

use getopts::Options;
use std::env;
mod unko;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();
  opts.optflag("k", "kuso", "set title \"KUSO\"");
  opts.optflag("h", "help", "print kuso help menu");

  let matches = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!(f.to_string()) }
  };

  if matches.opt_present("h") {
      print_usage(&program, opts);
      return;
  }
  else if matches.opt_present("k") {
      unko::kuso();
      return;
  }

  unko::print(0);
}
