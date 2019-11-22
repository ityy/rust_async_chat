use std::io::{BufRead, BufReader};

use async_std::{
    net::{TcpListener, ToSocketAddrs}, // 3
    prelude::*, // 1
    task, // 2
};
use futures::{AsyncReadExt, FutureExt, StreamExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// 让我们实现协议的接收部分。我们要：
/// 1 用\n拆分传入的TcpStream并将字节解码为utf-8
/// 2 将第一行解析为登陆信息
/// 3 将其余行解析为login：message
async fn accept_loop(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;//此处会阻塞task直到功能完成获取返回值, 执行task得线程则不会阻塞, 会继续向下轮询其它task执行.
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from:{}", stream.peer_addr()?);
        let _handle = task::spwan(cannection_loop(stream));
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name={}", name);

    while let Some(line) = lines.next().await {
        let line = line?;
        let (dest, msg) = match line.find(':') {
            None => continue,
            Some(idx) => (&line[..idx], line[idx + 1..].trim()),
        };
        let dest: Vec<String> = dest.split(',').map(|name| name.trim().to_string()).collect();
        let msg: String = msg.trim().to_string();
    }
    Ok(())
}


#[test]
fn run() -> Result<()> {
    let fut = accept_loop("127.0.0.1:8080");
    task::block_on(fut) //
}