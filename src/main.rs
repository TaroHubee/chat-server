use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};

enum Message {
    Chat(String),
    Nick(String),
    Quit,
}

fn parse_message(line: &str) -> Message {
    if line.starts_with("/nick") {
        Message::Nick(line.strip_prefix("/nick ").unwrap_or("").to_string())
    } else if line == "/quit" {
        Message::Quit
    } else {
        Message::Chat(line.to_string())
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("サーバー起動: 127.0.0.1:8080");

    let (stream, addr) = listener.accept().unwrap();
    println!("接続あり: {}", addr);

    let mut writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = line.unwrap();
        let msg = parse_message(&line);
        match msg {
            Message::Chat(text) => {
                println!("チャット: {}", text);
                writeln!(writer, "Echo: {}", text).unwrap();
            }
            Message::Nick(name) => {
                println!("名前変更: {}", name);
                writeln!(writer, "名前を {} に設定しました", name);
            }
            Message::Quit => {
                println!("切断");
                break;
            }
        }
        
    }
}
