use std::env;


fn main() {
    print!("请输入你的姓名：");
    let args: Vec<String> = env::args().collect();
    println!("传入了{:?}个参数", args.len());
    println!("你好！{:?}!", args[1]);
}
