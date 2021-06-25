use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    match config {
        Ok(config) => minigrep::run(config),
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    // if let Err(e) = minigrep::run(config) {
    //     // --snip--
    // }

    // run(config);

    // let filename = &args[2];
    // let wd = &args[3];
    //   println!("{:?}", config.filename);
    // let content = fs::read_to_string(filename).expect("wrong");

  
    // println!("With text:\n{}", content);
}
