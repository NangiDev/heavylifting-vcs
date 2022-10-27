use commit::commit;
use init::init;
use log::log;
use std::collections::VecDeque;
use std::env;
use std::process::exit;
use tag::tag;

fn main() {
    let snapshot_counter: i32 = 1;
    let mut args: VecDeque<String> = env::args().collect();

    let _cmd_name = args.pop_front();

    if args.is_empty() {
        println!("Run help document");
        panic!("Not sufficent arguments applied");
    }

    while !args.is_empty() {
        match args.pop_front().as_deref() {
            Some("init") => {
                init(args.pop_front());
                exit(0);
            }
            Some("commit") => {
                commit(snapshot_counter, args.pop_front());
                exit(0);
            }
            Some("log") => {
                log();
                exit(0);
            }
            Some("tag") => {
                tag(args.pop_front(), args.pop_front());
                exit(0);
            }
            None => {
                panic!("Not sufficent arguments applied")
            }
            arg => {
                println!("Run help document");
                panic!("Invalid argument: {}", arg.unwrap());
            }
        };
    }
}
