use std::io::{Error, BufReader, Write, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;
use std::str;

fn handle_client(mut stream: TcpStream) -> Result<(), Error>{

    loop {
        //创建缓冲区
        let mut buffer: Vec<u8> = Vec::new();
        //读取客户端发送的内容
        let mut reader = BufReader::new(&stream);
        let size = match reader
            .read_until(b'\n', &mut buffer){
            Ok(r) => r,
            Err(e) => {
                return Err(e);
            }
        };
        //客户端断开连接则返回
        if size == 0 {
            println!("cli disconnect");
            return Ok(());
        }
        //打印客户端发送信息
        println!("receive msg from cli : {}", str::from_utf8(&buffer).unwrap());
        //将信息再返回给客户端
        stream.write(&buffer)?;
        //现成等待1s
        thread::sleep(time::Duration::from_secs(1 as u64));
    }

}

fn main() -> std::io::Result<()> {
    //监听本地8080端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //创建线程动态数组
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        //建立tcp连接
        let stream = stream.expect("failed!");
        //起一个子线程处理tcp连接
        let handle = thread::spawn(move || {
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //将tcp连接加入到动态数组
        thread_vec.push(handle);
    }

    //等待所有子线程结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}
