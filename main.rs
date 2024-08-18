use core::num;
#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::path::StripPrefixError;
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;
#[allow(dead_code)]
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads:  u16){
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}  
        }
        if ( MAX-port ) <= num_threads{
            break;
        }
        port += num_threads; 
    }
}
impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        }else if args.len() > 4 {
            return Err("Too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return Ok(Arguments  {flag: String::from(""), ipaddr, threads:4 });
        }else{
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage : -j to select how many threads you want
                \r\n      -h or --help for help menu");
                return Err("help");
            }else if flag.contains("-h") || flag.contains("-help"){
                return Err("Too many arguments");
            }else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid ip address") 
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s) => s,
                    Err(_) =>  return Err("falied to parse thread no.")
                };
                return Ok(Arguments { flag, ipaddr, threads });
            }else{
                return Err("Invalid Syntax");
            }

        }

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // for i in &args{
    //     println!("{}",i);
    // }
    let programe = args[0].clone(); 
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help"){
                process::exit(0);
            }else{
                println!("{} is having problem parsing arguments:  {}",programe,err);
                process::exit(0);
            }
        } 
    );
    let num_threds = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threds  {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, arguments.ipaddr, num_threds)
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }
    println!("");
    out.sort();

    for v in out{
        println!("{} port is open",v);
    }
    // println!("{:?}",args);
}
