use std::process::ExitCode;

fn usage(args: Vec<String>) -> ExitCode {
    println!("Usage: {} stack|heap <index>", args[0]);
    println!("  index is 0..=3 for successes else out-of-bounds error");

    ExitCode::from(1)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Missing arguments");
        return usage(args);
    }

    let index: isize = match args[2].parse::<isize>() {
        Ok(i) => i,
        Err(e) => {
            println!("Error parsing second argument: {e}");
            return usage(args);
        }
    };

    // Create array of 4 items, indexes 0..=3
    // either on the "stack" or "heap"
    let val = match args[1].as_str() {
        "stack" => {
            let xs = [0, 1, 2, 3];
            unsafe { *xs.as_ptr().offset(index) }
        }
        "heap" => {
            let xs = Box::new([0, 1, 2, 3]);
            unsafe { *xs.as_ptr().offset(index) }
        }
        _ => {
            println!(
                "First argument, \"{}\", should be \"stack\" or \"heap\" without quotes",
                args[1]
            );
            return usage(args);
        }
    };

    println!("main:- xs[{index}]=0x{val:0x}");

    ExitCode::from(0)
}
