// use eframe::{egui::{self, Pos2, Rounding, Sense, Vec2, ViewportBuilder}};
mod app_conf;
mod content;
mod app_threads;
use app_conf::setup_custom_fonts;

use app_threads::{socket_reader,mqtt_reader,socket_ping};
// use app_threads::{socket_ping, socket_reader};
// use catppuccin_egui::{FRAPPE, LATTE, MACCHIATO, MOCHA};
use eframe::egui::{self, menu, vec2, Color32, RichText, Vec2, ViewportBuilder,Ui};
use content::{graph::graph_view, main::main_view, remote::remote_view, view::view_view,chat::chat_view};
use egui_tracing::EventCollector;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, Level,warn};
use tungstenite::{connect, http::Uri, stream::{MaybeTlsStream, NoDelay}, ClientRequestBuilder, Message, WebSocket};
// use tungstenite::{connect, http::Uri, stream::MaybeTlsStream, ClientRequestBuilder, WebSocket};
use std::{iter::repeat, net::TcpStream, sync::{Arc, Mutex}};
use crossbeam_channel::{unbounded, Receiver, Sender};

#[cfg(unix)]
const SOCKET_URL: &'static str = env!("SOCKET_URL");

#[derive(Clone,Default)]
enum Menu{
    REMOTE,
    VIEW,
    GRAPH,
    CHAT,
    LOGVIEW,
    #[default]
    None
}


fn main() {
    let collector = egui_tracing::EventCollector::default().with_level(Level::INFO);
    // collector.with_level(Level::INFO);
    tracing_subscriber::registry()
        .with(collector.clone())
        .init();
    let windows = ViewportBuilder{
        title: Some(String::from("yum car app")),
        app_id: Some(String::from("yum car app")),
        // fullsize_content_view: Some(true),
        titlebar_shown: Some(false),
        min_inner_size: Some(Vec2::new(380., 800.)),
        resizable: Some(false),
        fullscreen:Some(true),
        ..Default::default()
    };
    let options = eframe::NativeOptions {
        viewport:windows,
        // default_theme:Theme::Dark,
        ..Default::default()
    };
    
    // let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
         options,
          Box::new(|cc|{
            let mut app = MyEguiApp::new(cc,collector);
            let msg_mem=app.message.clone();
            let msg_reader_mem=app.msg_reader.clone();
            let msg_sender_mem=app.msg_sender.clone();
            let socket_mem = app.socket.clone();
            socket_reader(msg_mem,socket_mem);
            let socket_mem = app.socket.clone();
            socket_ping(msg_sender_mem,socket_mem);
            // mqtt_reader();
            // let socket_mem=app.socket_mem.clone();
            // socket_reader(socket_mem,msg_mem); 
            // // let socket_mem=app.socket_mem.clone();
            // socket_ping(socket_mem);
            Ok(Box::new(app))
          }
        ));
}
    // eframe::run_native("My egui App", options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc,collector))));

// #[derive(Default)]
struct MyEguiApp {
    menu : Menu,
    collector: EventCollector,
    message:Arc<Mutex<Vec<String>>>,
    msg_sender:Arc<Mutex<Sender<Message>>>,
    msg_reader:Arc<Mutex<Receiver<Message>>>,
    socket:Arc<Mutex<Option<WebSocket<MaybeTlsStream<TcpStream>>>>> ,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>,collector: EventCollector) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        let socket = Arc::new(Mutex::new(None));
        let uri: Uri = SOCKET_URL.parse().unwrap();
        let builder = ClientRequestBuilder::new(uri);
        if let Ok((mut sockets,res))=connect(builder){
            match sockets.get_mut() {
                tungstenite::stream::MaybeTlsStream::Plain(stream) => {stream.set_read_timeout(None).unwrap()},
                tungstenite::stream::MaybeTlsStream::NativeTls(stream) => {
                    stream.get_mut().set_read_timeout(None).unwrap()
                }
                _ => unimplemented!(),
            }
            *socket.lock().unwrap()=Some(sockets);
        }
        let (s, r) = unbounded::<Message>();
        let msg_sender = Arc::new(Mutex::new(s));
        let msg_reader = Arc::new(Mutex::new(r));
        let message = Arc::new(Mutex::new(repeat("".to_string()).take(5).collect()));
        
        Self{
            menu:Menu::default(),
            msg_sender,
            msg_reader,
            collector,
            message,
            socket,
            // ..Default::default()
        }
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    ctx.request_repaint();
    
    // catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA);
    ctx.set_pixels_per_point(1.25);
       egui::CentralPanel::default().show(ctx, |ui| {
        
        menu::bar(ui, |ui| {
            // ui.label(format!("{}", egui_nerdfonts::regular::REMOTE));
            ui.menu_button(
                RichText::new(
                    "🏠Main"
                    // format!("{} Main", egui_nerdfonts::regular::HOME)
                )
                    .strong()
                    .size(22.0)
                    .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Home").clicked() {
                    self.menu=Menu::None;
                    info!("TEST HOME CLICKED");
                    // …
                }
                if ui.button("My Chatting").clicked() {
                    self.menu=Menu::CHAT;
                    warn!("TEST");
                    // …
                }
                if ui.button("APP LOG").clicked() {
                    self.menu=Menu::LOGVIEW;
                    // …
                }
            });
            ui.menu_button(
                RichText::new(
                    "📱Remote"
                    // format!("{} Remote", egui_nerdfonts::regular::REMOTE_1)
                )
                    .strong()
                    .size(22.0)
                    .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    self.menu=Menu::REMOTE;
                    // …
                }
            });
            ui.menu_button(RichText::new(
                "🗖View"
                // format!("{} View", egui_nerdfonts::regular::MONITOR)
            )
                .strong()
                .size(22.0)
                .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    self.menu=Menu::VIEW;
                    // …
                }
            });
            ui.menu_button(RichText::new(
                "🗠Graph"
                // format!("{} Graph", egui_nerdfonts::regular::GRAPH_1)
            )
                .strong()
                .size(22.0)
                .color(Color32::from_rgb(38, 150, 255)), |ui| {
                if ui.button("Open").clicked() {
                    self.menu =Menu::GRAPH;
                }
            });
        });
           ui.add_space(50.);
           match self.menu {
            Menu::REMOTE=>{
                remote_view(ui,ctx);
            },
            Menu::VIEW=>{
                view_view(ui,ctx);
            },
            Menu::GRAPH=>{
                graph_view(ui,ctx);
            }
            Menu::LOGVIEW=>{
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add(egui_tracing::Logs::new(self.collector.clone()));
                });
            }
            Menu::CHAT=>{
                let msg_mem = self.message.clone();
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.label(RichText::new(
                        "HI THERE!!!"
                        // format!("{} View", egui_nerdfonts::regular::MONITOR)
                    )
                        .strong()
                        .size(22.0)
                        .color(Color32::from_rgb(235, 64, 52)));
                    for i in self.message.lock().unwrap().iter(){
                        ui.label(RichText::new(
                            i
                            // format!("{} View", egui_nerdfonts::regular::MONITOR)
                        )
                            .strong()
                            .size(15.0)
                            .color(Color32::from_rgb(235, 64, 52)));
                    }
                    // Add a lot of widgets here.
                });
            }
            _=>{
                main_view(ui,ctx);
            }
           }
        //    ui.heading("START CAR APP");
       });
   }
}