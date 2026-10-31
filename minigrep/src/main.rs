use std::env;
use std::error::Error;
use std::fs;
use std::panic;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Vec is Rust's growable, ordered list type. Its elements are stored on the heap.
    // <String> specifies the type of each element.

    // dbg!(&args);

    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {query}");
    // println!("In file {file_path}");

    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    // let (query, file_path) = parse_config(&args);
    // let config = parse_config(&args);
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        // Question: what is unwrap_or_else
        // Answer: do not panic like unwrap(), deal with err with closure.
        // Question: what does |err| mean? |err| captures the var named err in frame of
        // unwrap_or_else?
        // Answer: no |err|, err is a parameter, unwrap_or_else will use call this no name function
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // let contents =
    //     fs::read_to_string(config.file_path).expect("Should have been to read the files");
    // run(config);
    if let Err(e) = run(config) {
        // Question: what is if let?
        println!("Application error: {e}");
        process::exit(1);
    }

    // println!("With text:\n {contents}");
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Question: what is Box? what is dyn Error?

    // let contents =
    //     fs::read_to_string(config.file_path).expect("should have been able to read the file");
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Question: new is an associated function?
    // Answer: yes
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();     Config { query, file_path }
    // }
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];
//
//     (query, file_path)
// }

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    // let query = args[1];  // 编译错误
    // 这相当于试图从借用的 slice 中移走一个非 Copy 类型的值。Rust 不允许这样做。
    // clone() 会创建一个新的 String，让 Config 独立拥有它。
    //
    // also can borrow rather than clone
    // struct Config<'a> {
    //    query: &'a str,
    //    file_path: &'a str,
    // }
    //
    // Question: what is lifetime?
    //
    //
    // Question: what is Copy type?
    // i32 实现了 Copy trait
    //
    // let a = 10;
    // let b = a;
    // println!("{a}"); // 正常
    // println!("{b}"); // 正常
    // let file_path = args[2].clone();
    //
    // Rust 的策略是：对于非 Copy 类型，赋值默认执行 move，让原变量失效。
    // Copy 是 Clone 的子 trait，能实现 Copy 的类型必须也实现 Clone。
    //
    // Question: what is trait?

    Config { query, file_path }
}
