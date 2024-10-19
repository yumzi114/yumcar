use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream};
use crossbeam_channel::{Receiver, Sender};
// use futures::stream::SplitSink;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::runtime::Runtime;
use futures::{SinkExt, StreamExt};

use tokio::time;
// use futures::{AsyncRead, AsyncReadExt};
use tracing::{info, warn};
use tungstenite::http::Uri;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;
use futures_util::{future, pin_mut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rumqttc::{AsyncClient, Client, MqttOptions, QoS};
use std::borrow::BorrowMut;

#[cfg(unix)]
const MQTT_IP: &'static str = env!("MQTT_IP");
const MQTT_TOPIC: &'static str = env!("MQTT_TOPIC");
const MQTT_ID: &'static str = env!("MQTT_ID");
const MQTT_PW: &'static str = env!("MQTT_PW");


pub fn socket_ping(
    msg_sender:Arc<Mutex<Sender<Message>>>,
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>> 
){
    thread::spawn(move|| {
        info!("PING THREAD OPEN");

        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            info!("PING THREAD OPEN");
            loop{
   
                thread::sleep(Duration::from_secs(10));
                if let Some(socket)=(*socket.lock().unwrap()).as_mut(){
                    socket.send(Message::Ping(vec![0_u8])).unwrap();
                    match socket.get_mut() {
                        tungstenite::stream::MaybeTlsStream::Plain(stream) => {stream.set_read_timeout(Some(Duration::from_secs(15))).unwrap()},
                        tungstenite::stream::MaybeTlsStream::NativeTls(stream) => {
                            stream.get_mut().set_read_timeout(Some(Duration::from_secs(15))).unwrap()
                        }
                        _ => unimplemented!(),
                    }
                }
            }
        })
    });

}
pub fn socket_reader(
    msg_mem:Arc<Mutex<Vec<String>>>,
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>> 
){  
    thread::spawn(move|| {
        let rt  = Runtime::new().unwrap();
        rt.block_on(async {
            info!("SOCKET READER THREAD OPEN");
            loop{
                if let Ok(socket_mem)=socket.lock().as_mut(){
                    let ss = socket_mem.as_mut();
                    if let Some(ss)=ss{
                        if let Ok(msg)=ss.read(){
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
                }
                thread::sleep(Duration::from_millis(1));
            }
        });
    });
}
pub fn mqtt_reader(){
    
    let rt  = Runtime::new().unwrap();
    rt.block_on(async {
        let mut mqttoptions = MqttOptions::new(
            "yumcar_app",
            MQTT_IP,
              1883);
        mqttoptions.set_credentials(MQTT_ID.to_string(),MQTT_PW.to_string());
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        
        client.subscribe(MQTT_TOPIC, QoS::AtMostOnce).await.unwrap();
        while let Ok(notification) = eventloop.poll().await {
            
            println!("Received = {:?}", notification);
        }
    });
}