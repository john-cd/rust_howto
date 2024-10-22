// use std::path::PathBuf;

// use clap::arg;
// use clap::value_parser;
// use clap::Arg;
// use clap::Command;

// // TODO fix *  The terminal process "cargo 'test', '--package',
// 'deps', '--test', 'clap-basic', '--', 'test', '--exact',
// '--show-output'" terminated with exit code: 2.

// fn cli() -> Command {
//     clap::Command::new("My Test Program")
//         .bin_name("test_app")
//         .version("0.1.0")
//         .author("Hackerman Jones <hckrmnjones@hack.gov>")
//         .about("Teaches argument parsing")
//         .arg(
//             Arg::new("num")
//                 .short('n')     // -n argument
//                 .long("number") // --number long-form argument
//                 .value_name("NUMBER") // placeholder for the
// argument's value in the help message / usage.
// .required(false)                 .help("Enter your favorite
// number")         )
//         .arg(arg!(--value <VALUE>)) // Use the arg! macro
//         .arg(
//             Arg::new("file")
//                 .short('f')
//                 .long("file")
//                 .value_parser(value_parser!(PathBuf))
//                 .help("Enter the path of a file")
//         )
// }

// #[test]
// fn test() {
//     let matches = cli().get_matches();

//     if let Some(num) = matches.get_one::<i64>("num") {
//         println!("Value for num: {num}");
//     }

//     if let Some(file_path) = matches.get_one::<PathBuf>("file") {
//         println!("Value for file: {}", file_path.display());
//     }
// }
