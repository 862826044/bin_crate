use std::env;

///老铁，你是谁？
fn main() {
    print!("请输入你的姓名：");
    let args: Vec<String> = env::args().collect();
    println!("传入了{:?}个参数", args.len()-1);
    println!("你好！{:?}!", args[1]);
}
