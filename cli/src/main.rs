use cli::Cli;
use structopt::StructOpt;

fn main() {
    let opt: Cli = Cli::from_args();
    println!("{:?}", opt.speed);
    println!("{:?}", opt.debug);
    println!("{:?}", opt.input);
    println!("{:?}", opt.out_type);
    println!("{:?}", opt.output);
    println!("{:?}", opt.file_name);
}
