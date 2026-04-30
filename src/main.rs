use std::net::{TcpListener, SocketAddr};
use std::io::{BufReader, BufRead, Write};
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, mpsc};

enum Message {
    Chat(String),
    Nick(String),
    List,
    Quit,
}

fn parse_message(line: &str) -> Message {
    if line.starts_with("/nick") {
        Message::Nick(line.strip_prefix("/nick ").unwrap_or("").to_string())
    } else if line == "/quit" {
        Message::Quit
    } else if line == "/list" {
        Message::List
    } else {
        Message::Chat(line.to_string())
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("サーバー起動: 0.0.0.0:8080");

    let clients: Arc<Mutex<HashMap<SocketAddr, (mpsc::Sender<String>, Option<String>)>>> = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (stream, addr) = listener.accept().unwrap();
        let (tx, rx) = mpsc::channel::<String>();
        println!("接続あり: {}", addr);

        clients.lock().unwrap().insert(addr,(tx, None));
        let clients = clients.clone();

        thread::spawn(move || {
            let mut writer_for_rx = stream.try_clone().unwrap();
            let mut writer = stream.try_clone().unwrap();
            let reader = BufReader::new(stream);

            {
                let msg = format!("{} が入室しました", addr);
                let clients = clients.lock().unwrap();
                for (_, (tx, _)) in clients.iter() {
                    tx.send(msg.clone()).unwrap();
                }
            }

            thread::spawn(move || {
                for msg in rx {
                    writeln!(writer_for_rx, "{}", msg).unwrap();
                }
            });


            for line in reader.lines() {
                let line = line.unwrap();
                let msg = parse_message(&line);
                match msg {
                    Message::Chat(text) => {
                        let clients = clients.lock().unwrap();
                        for (_add, (tx, _)) in clients.iter() {
                            tx.send(text.clone()).unwrap();
                        }
                        println!("チャット: {}", text);
                    }
                    Message::Nick(name) => {
                        println!("名前変更: {}", name);
                        writeln!(writer, "名前を {} に設定しました", name).unwrap();
                        if let Some(entry) = clients.lock().unwrap().get_mut(&addr) {
                            entry.1 = Some(name);
                        }
                        
                    }
                    Message::Quit => {
                        println!("切断");
                        break;
                    }
                    Message::List => {
                        let clients = clients.lock().unwrap();
                        let names: Vec<String> = clients.values()
                            .map(|(_, nick)| {
                                nick.clone().unwrap_or_else(|| "名無し".to_string())
                            })
                            .collect();
                        writeln!(writer, "接続中: {}", names.join(", ")).unwrap();
                    }
                }
            }

            {
                let msg = format!("{} が退出しました", addr);
                let mut clients = clients.lock().unwrap();
                for (_, (tx, _)) in clients.iter() {
                    tx.send(msg.clone()).unwrap();
                }
                clients.remove(&addr);
            }
        });
    }
}
