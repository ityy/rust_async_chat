//! 现在该实现另一半功能了-发送消息。
//! 实现发送的最明显方法是为每个connection_loop授予对其他客户端TcpStream的写的权限。
//! 这样，客户端可以直接.all_write消息给收件人。但是，这是错误的：如果Alice发送bob：foo，而Charley发送bob：bar，则Bob可能实际上收到了fobaor。
//! 通过套接字发送消息可能需要几个系统调用，因此两个并发的.write_all可能会相互干扰！
//! 根据经验，每个TcpStream只应写入一个任务。因此，让我们创建一个connection_writer_loop任务，该任务通过通道接收消息并将其写入套接字。
//! 该任务将是消息序列化的重点。如果Alice和Charley同时向Bob发送了两条消息，则Bob会以到达消息的顺序来查看消息。


// 我们将使用futures crate中的channels 。
use futures::channel::mpsc;
use futures::sink::SinkExt;
use std::sync::Arc;

// 为简单起见，我们将使用无限制的通道(unbounded channels)，并且在本教程中不会讨论背压(backpressure)。
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

async fn connection_writer_loop(mut messages: Receiver<String>,
                                //由于connection_loop和connection_writer_loop共享相同的TcpStream，
                                // 我们需要将其放入Arc。请注意，由于客户端仅从流中读取数据，而connection_writer_loop仅向流中写入数据，因此我们在这里没有任何竞争。
                                stream: Arc<TcpStream>, ) -> Result<()> {
    let mut stream = &*stream;
    while let Some(msg) = messages.next().await {
        stream.write_all(msg.as_bytes()).await?;
    }
    Ok(())
}