// rustc write_file.rs
// ./write_file
use std::fs;
use std::time::Duration;
use std::thread;

fn main(){
    // 睡眠
    thread::sleep(Duration::from_millis(5000));4
    // 写文件
    fs::write("output.txt", "hello, world!").unwrap();

    // 读文件
    let file_context = fs::read("output.txt").unwrap();
    // 将编码 转成字符
    let content_string = String::from_utf8_lossy(&file_context);
    println!("{:?}", content_string);
}
