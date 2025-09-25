use std::{env, process, sync::Arc, thread};

use riic::Client;
fn main() {
    let arguments: Arc<[String]> = env::args().collect();
    let build = Client::build(&arguments).unwrap_or_else(|err| {
        eprint!("noob:{}", err);
        process::exit(0);
    });

    let handler = thread::spawn(move || build.initialize_connection());
    let _ = handler.join();
}
