
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Args for Crob9000", about = "Currently just version")]
struct Opt {
    #[structopt(name = "version")]
    version: u32,
}

fn main() -> Result<(), ()> {
    let opt = Opt::from_args();
    if opt.version < 84 {
        Err(())
    } else {
        Ok(())
    }
}
