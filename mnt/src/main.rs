use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

use anyhow::Result;
use clap::Parser;
use shared::mnt_protocols::{
    ClientRequest, ClientRequestContent, ServerResponse, SyncZigZagVarint, MNT_PATH,
};
use shared::objectid::ObjectId;

/// 简单的命令行客户端，用于与 WebGateway 的管理 Unix 套接字通信
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 可选的 Unix 套接字路径（默认使用协议中定义的 MNT_PATH）
    #[arg(short, long, default_value = MNT_PATH)]
    socket: String,

    /// 要发送的请求类型（目前仅支持 admin-totp）
    #[arg(value_enum, default_value_t = RequestType::AdminTotp)]
    request: RequestType,
}

#[derive(clap::ValueEnum, Debug, Clone)]
enum RequestType {
    AdminTotp,
    // 将来可以扩展其他请求类型
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. 连接到 Unix 套接字
    let mut stream = UnixStream::connect(&args.socket)?;

    // 2. 根据命令行参数构造请求
    let request_content = match args.request {
        RequestType::AdminTotp => ClientRequestContent::AdminTOTP,
    };

    // 生成一个请求 ID（实际应用中应使用唯一 ID，这里简单使用固定值 1）
    // 假设 ObjectId 实现了 From<u64> 或者有一个 new 方法
    let id = ObjectId::new(); // 根据您的 ObjectId 定义调整

    let request = ClientRequest {
        id,
        content: request_content,
    };

    // 3. 序列化并发送
    let buf = serde_json::to_vec(&request)?;
    stream.write_zigzag_varint::<usize>(buf.len())?;
    stream.write_all(&buf)?;

    // 4. 读取响应长度和数据
    let size = stream.read_zigzag_varint::<usize>()?;
    let mut buf = vec![0; size];
    stream.read_exact(&mut buf)?;

    // 5. 解析响应
    let response: ServerResponse = serde_json::from_slice(&buf)?;

    // 如果响应中包含错误，以非零退出码退出
    if response.error.is_some() {
        eprintln!("服务端返回错误: {:?}", response.error);
        std::process::exit(1);
    }

    let response_content = response.content.unwrap();

    match response_content {
        shared::mnt_protocols::ServerResponseContent::AdminTOTP { user, totp } => {
            println!("User: {user}");
            println!("TOTP: {totp}");
        }
    }

    Ok(())
}
