use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub struct TestHttpServer {
    pub url: String,
    request_line: Arc<Mutex<Option<String>>>,
}

impl TestHttpServer {
    pub async fn request_line(&self) -> Option<String> {
        self.request_line.lock().await.clone()
    }
}

pub async fn spawn_json_server(body: &'static str) -> TestHttpServer {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let addr = listener.local_addr().expect("server address");
    let request_line = Arc::new(Mutex::new(None));
    let request_line_writer = Arc::clone(&request_line);

    tokio::spawn(async move {
        if let Ok((mut socket, _)) = listener.accept().await {
            let mut buffer = [0u8; 4096];
            if let Ok(bytes_read) = socket.read(&mut buffer).await {
                let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                let first_line = request.lines().next().unwrap_or_default().to_string();
                *request_line_writer.lock().await = Some(first_line);

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = socket.write_all(response.as_bytes()).await;
            }
        }
    });

    TestHttpServer {
        url: format!("http://{}", addr),
        request_line,
    }
}
