use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "demo")]
struct Opt {
    #[structopt(short = "g", long = "greet")]
    pub greet: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.greet {
        println!("Hello, from Wasmer!");
    }
    else {
        println!("missing something?");
    }
}
