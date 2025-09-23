use std::{env, thread, process, sync::Arc};

use riic::Client;
fn main() {
    let arguments: Arc<[String]> = env::args().collect();
    let build = Client::build(&arguments).unwrap_or_else(|err| {
        eprint!("noob:{}", err);
        process::exit(0);
    });

    let handle = build.initialize_connection();
    handle.join().unwrap();
}
