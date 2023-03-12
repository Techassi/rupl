// use std::net::Ipv4Addr;

// use rupl::args::{Arg, Args};

// #[test]
// fn test_args_simple() {
//     let input = String::from("--arg value");

//     let args = match Args::new(input, vec![Arg::new("arg")]) {
//         Ok(p) => p,
//         Err(err) => panic!("{}", err),
//     };

//     let arg: String = match args.get("arg") {
//         Ok(p) => p,
//         Err(err) => panic!("{}", err),
//     };

//     assert_eq!(arg, String::from("value"))
// }

// #[test]
// fn test_args_ipaddr() {
//     let input = String::from("--ip 10.10.10.10");

//     let args = match Args::new(input, vec![Arg::new("ip")]) {
//         Ok(p) => p,
//         Err(err) => panic!("{}", err),
//     };

//     let ip: Ipv4Addr = match args.get("ip") {
//         Ok(p) => p,
//         Err(err) => panic!("{}", err),
//     };

//     assert_eq!(ip, Ipv4Addr::new(10, 10, 10, 10))
// }
