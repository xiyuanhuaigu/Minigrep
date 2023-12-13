use std::env;
use std::fs;
use std::process;

struct Config{
    query: String,
    file_path: String,
}

fn main(){
    // 首先通过 use 引入标准库中的 env 包，然后 env::args 方法会读取并分析传入的命令行参数，最终通过 collect 方法输出一个集合类型 Vector。
    let args: Vec<String> = env::args().collect();

    // 对build返回的Result进行处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("文件名路径为{}",config.query);
    println!("文件名为{}",config.file_path);

    run(config);
}

fn run(config: Config){
    let contents = fs::read_to_string(config.file_path).expect("请输入文件");

    println!("With text:\n{contents}");
}

impl Config {
    fn build(args:&[String]) -> Result<Config, &'static str> {

        if args.len() < 3{
            return Err("not enough arguments");
        } 

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}