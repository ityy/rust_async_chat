//! 连接读与写
//! 那么，如何确保在connection_loop中读取的消息流入相关的connection_writer_loop？
//! 我们应该以某种方式维护一个对等体：HashMap <String，Sender <String >>映射，它允许客户端找到目标通道。
//! 但是，此映射将具有一些共享的可变状态，因此我们必须在其上包装一个RwLock，并回答一些棘手的问题，即如果客户端在收到消息的同时加入，会发生什么情况。
//! 使状态的推理更简单的一个技巧来自参与者模型。我们可以创建一个专门的代理任务，该任务拥有对等映射，并使用渠道与其他任务进行通信。
//! 通过将对等方隐藏在这样的“角色”任务中，我们消除了对互斥量的需要，并明确了序列化点。
//! 事件“鲍勃将消息发送给爱丽丝”的顺序和“爱丽丝加入”的顺序由代理的事件队列中相应事件的顺序确定。

use std::collections::hash_map::{Entry, HashMap};
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Debug)]
enum Event {
    NewPeer {
        name: String,
        stream: Arc<TcpStream>,
    },
    Message {
        from: String,
        to: Vec<String>,
        msg: String,
    },
}


