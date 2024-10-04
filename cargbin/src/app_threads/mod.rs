use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use tokio_tungstenite::tungstenite::http::Uri;
use tokio_tungstenite::tungstenite::ClientRequestBuilder;
use tracing::{info, warn};
use url::Url;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[cfg(unix)]
const SOCKET_URL: &'static str = env!("SOCKET_URL");
pub fn socket_reader(
    // socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>>,
    msg_mem:Arc<Mutex<Vec<String>>>
){  
    thread::spawn(move|| {
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            info!("SOCKET READER THREAD OPEN");
            let uri: Uri = SOCKET_URL.parse().unwrap();
            let builder = ClientRequestBuilder::new(uri);
            let (mut ws_stream, _) = connect_async(builder).await.expect("Failed to connect");
            let (mut write, mut read) = ws_stream.split();
            loop{
                if let Some(msg) = read.next().await {
                    if let Ok(msg)=msg{
                        match msg {
                            Message::Text(text)=>{
                                info!("SOCKET READ MASSAGE : {}",text);
                                msg_mem.lock().unwrap().push(text);
                            }
                            _=>{
    
                            }
                        }
                    }
                }
                write.send(Message::Ping(vec![1_u8])).await.unwrap();
                thread::sleep(Duration::from_millis(1));
            }
            
        });
    });
}
