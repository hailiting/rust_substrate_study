use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;
use std::time;

fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    // 定义 buf 长度为512， 内容为0的数组
    let mut buf = [0; 512];
    // 循环1000次处理数据之后关闭，也可以用 loop，表示一直提供服务
    for _ in 0..1000 {
        // stream 读取数据到 buf，读取到的数据长度存入 bytes_read
        let bytes_read = stream.read(&mut buf)?;
        // bytes_read 为0 说明没读到
        if bytes_read == 0 {
            return Ok(());
        }
        // 打印
        println!(
            "get from client {}",
            str::from_utf8(&buf[..bytes_read]).unwrap()
        );
        // 向 stream 写会读到的数据
        stream.write(&buf[..bytes_read])?;
        // 延时 1s
        thread::sleep(time::Duration::from_secs(1));
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    // `?` 会在返回失败时 return Err， 是返回错误的语法糖
    // let listener = TcpListener::bind("127.0.0.1:8080")?;

    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to create TcpListener");

    // 将监听器设置为非阻塞模式
    // listener
    //     .set_nonblocking(true)
    //     .expect("Cannot set non-blocking");

    // mut 创建可变变量
    // Vec 数组存储线程句柄
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    // 处理连接，每一个连接都是一个进程
    for stream in listener.incoming() {
        // stream 是 result, 可以使用 match 处理
        // let 重新定义 stream，是rust的shadow特性，区别于mut,实际是创建了一个新的变量
        let stream = stream.expect("failed!");
        // thread::spawn 创建一个新的线程
        // 使用move的闭包会将外部stream变量引入到闭包中使用
        // 引入的变量，main将不会再有使用权
        let handle = thread::spawn(move || {
            // unwrap_or_else match到Err打印到error信息
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        // 把句柄放到数组中
        thread_vec.push(handle);
    }

    for handle in thread_vec {
        // 线程需要等所有子线程结束才能结束
        handle.join().unwrap();
    }
    Ok(())
}
