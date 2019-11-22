use async_std::{
    net::{TcpListener, ToSocketAddrs}, // 3
    prelude::*, // 1
    task, // 2
};
use async_std::{
    io::BufReader,
    net::TcpStream,
};
use futures::{AsyncReadExt, FutureExt, StreamExt, Future};

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
        //我们使用task :: spawn函数来生成一个独立的任务以与每个客户端一起工作。也就是说，接受客户端后，accept_loop立即开始等待下一个客户端。跟java接收到客户端后新建线程处理类似, 只不过这里使用的是异步任务.
        //这是事件驱动的体系结构的核心优势：我们同时为许多客户端提供服务，而无需花费很多硬件线程。
        let _handle = task::spawn(connection_loop(stream));
    }

    Ok(())
}

async fn connection_loop(stream: TcpStream) -> Result<()> {
    //幸运的是，“将字节流分成几行”功能已经实现。 .lines（）调用返回String的流。
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines();

    //处理首行-->登陆信息
    let name = match lines.next().await {
        None => Err("peer disconnected immediately")?,
        Some(line) => line?,
    };
    println!("name={}", name);

    //而且，我们再次实现了手动异步for循环。
    while let Some(line) = lines.next().await {
        let line = line?;
        //最后，我们将每一行解析为目标登录名和消息本身的列表。
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

/// 处理错误
/// 上述解决方案中的一个严重问题是，尽管我们在connection_loop中正确传播了错误，但随后将错误放到了 floor 上！
/// 也就是说，task :: spawn不会立即返回错误（它不能，它需要先运行Future才能完成），只有在它加入后才返回。
/// 我们可以通过等待任务加入来“修复”它，如下所示：
/// ```
/// let handle = task::spawn(connection_loop(stream));
/// handle.await
/// ```
/// .await等待客户端完成，然后？传播结果。
/// 但是，此解决方案有两个问题！首先，因为我们立即等待客户端，所以我们一次只能处理一个客户端，这完全违反了异步的目的！
/// 其次，如果客户端遇到IO错误，则整个服务器将立即退出。也就是说，一个人的不稳定连接使整个聊天室瘫痪了！
/// 在这种情况下，处理客户端错误的正确方法是记录它们，并继续为其他客户端提供服务。
/// 因此，我们为此使用一个辅助函数：
fn spawn_and_log_error<F>(fut: F) -> task::JoinHandle<()>
    where
        F: Future<Output=Result<()>> + Send + 'static,
{
    task::spawn(async move {
        if let Err(e) = fut.await {
            eprintln!("{}", e)
        }
    })
}
