use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    str,
};
// fn main() {
//     for a in 0..5 {
//         println!("{}", a); // 0 1 2 3 4
//     }
// }
fn main() -> std::io::Result<()> {
    // 创建可变对象 stream 连接服务器
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    for _ in 0..10 {
        let mut input = String::new();
        // 读取数据并存到input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        // 将input转为bytes
        // 写入到stream
        stream
            .write(input.as_bytes())
            .expect("Failed to write to stream");
        // 通过 stream 创建 BufReader
        let mut reader = BufReader::new(&stream);
        // 创建一个Vector
        let mut buffer: Vec<u8> = Vec::new();
        // reader中数据转行为止，存入buffer
        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");
        // 打印buffer
        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
        println!("");
    }
    Ok(())
}
