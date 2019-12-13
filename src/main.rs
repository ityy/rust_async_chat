//! 规格与说明
//! 聊天使用基于TCP的简单文本协议。该协议包含utf-8消息，以\n分隔。
//! 客户端连接到服务器，并作为第一行发送登录信息。之后，客户端可以使用以下语法将消息发送给其他客户端：
//! login1, login2, ... loginN: message
//! 然后，每个指定的客户端都会收到一条 from login: message 消息。
//! 可能的会话如下所示:
//! On Alice's computer:   |   On Bob's computer:
//!
//! > alice                |   > bob
//! > bob: hello               < from alice: hello
//!                        |   > alice, bob: hi!
//!                            < from bob: hi!
//! < from bob: hi!        |
//!
//! 聊天服务器的主要挑战是跟踪许多并发连接。聊天客户端的主要挑战是管理并发传出消息，传入消息和用户键入。


mod t01_accept_loop;
mod t02_receiving_messages;
//mod t03_sending_messages;
mod t04_connecting_readers_and_writers;


