use std::process;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match wf2_core::Wf2::from_args(args.into_iter(), false) {
        Ok((_cli, tasks)) => {
            // dbg!("all good :)");
            // dbg!(cli);
            dbg!(tasks);
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}
