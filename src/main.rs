extern crate rl_sys;

use rl_sys::readline;
use rl_sys::history::listmgmt;

fn evaluate(input: String) -> String {
    String::from("hello there")
}

fn main() {

    loop {
        let output: String = match readline::readline("") {
            Ok(Some(s)) => evaluate(s),
            Ok(None) => break,  // user entered ctrl-d
            Err(e) => {
                println!("{}", e);
                continue;

            }
        };
        println!("=> {}", output);

        // Enables up/down arrow scrolling through history
        listmgmt::add(&output).unwrap();

    }
}
