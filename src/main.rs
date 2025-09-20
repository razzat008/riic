use std::{env, process, sync::Arc};

use riic::Client;
fn main() {
    let arguments: Arc<[String]> = env::args().collect();
    let build = Client::build(&arguments).unwrap_or_else(|err| {
        eprint!("noob");
        process::exit(0);
    });
}
