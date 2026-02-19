use std::path::PathBuf;

use anyhow::Result;
use shared::{
    database::get_database,
    mnt_protocols::{
        AsyncZigZagVarint, ClientRequest, ClientRequestContent, MNT_PATH, ServerResponse,
        ServerResponseContent,
    },
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};
use tracing::event;

use crate::{auth::get_totp_code, database::auth::Authentication};

pub struct AutoCleanUnixListener(PathBuf);

impl Drop for AutoCleanUnixListener {
    fn drop(&mut self) {
        std::fs::remove_file(&self.0).ok();
    }
}

pub async fn init() -> anyhow::Result<()> {
    // unix
    let path = PathBuf::from(MNT_PATH);
    let _ = AutoCleanUnixListener(path.clone());
    event!(tracing::Level::INFO, "starting unix listener");
    let listener = UnixListener::bind(&path)?;
    event!(tracing::Level::INFO, "unix listener started");
    loop {
        let (stream, addr) = listener.accept().await?;
        event!(tracing::Level::INFO, "new unix connection: {addr:?}");
        tokio::spawn(async move {
            let res = handle(stream).await;
            if let Err(e) = res {
                event!(tracing::Level::DEBUG, "handle error: {e:?}");
            }
        });
    }
}

async fn handle(mut stream: UnixStream) -> Result<()> {
    loop {
        let size = stream.read_zigzag_varint::<usize>().await?;
        let mut buf = vec![0; size];
        stream.read_exact(&mut buf).await?;
        let data = serde_json::from_slice::<ClientRequest>(&buf)?;
        // TODO: handle request
        let res = handle_req(data.content).await;
        let response = match res {
            Ok(res) => ServerResponse {
                id: data.id,
                content: Some(res),
                error: None,
            },
            Err(e) => ServerResponse {
                id: data.id,
                content: None,
                error: Some(e.to_string()),
            },
        };
        let buf = serde_json::to_vec(&response)?;
        stream.write_zigzag_varint::<usize>(buf.len()).await?;
        stream.write_all(&buf).await?;
    }
}

async fn handle_req(req: ClientRequestContent) -> Result<ServerResponseContent> {
    match req {
        ClientRequestContent::AdminTOTP => {
            let user = get_database().get_first_user().await?;
            let code = get_totp_code(&user.username, user.totp_secret)?;
            Ok(ServerResponseContent::AdminTOTP {
                user: user.username.to_string(),
                totp: code,
            })
        }
    }
}
