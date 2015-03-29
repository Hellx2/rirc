use message::Message;
use command::{Command,Channel};

macro_rules! s {
    ($x:expr) => ($x.to_string());
}

#[test]
fn test_nick() {
    let m = Message::new("NICK yolo").unwrap();
    assert_eq!(m.cmd, Command::NICK("yolo".to_string(), None));
}

#[test]
fn test_nick_1() {
    let m = Message::new("NICK yolo 4").unwrap();
    assert_eq!(m.cmd, Command::NICK("yolo".to_string(), Some(4)));
}

#[test]
fn test_join_1() {
    let m = Message::new("JOIN #yolo").unwrap();
    assert_eq!(m.cmd, Command::JOIN(vec!(Channel {name: "#yolo".to_string(), key: None})));
}
#[test]
fn test_join_2() {
    let m = Message::new("JOIN #yolo key1").unwrap();
    assert_eq!(m.cmd, Command::JOIN(vec!(Channel {name: s!("#yolo"), key: Some(s!("key1"))})));
}

#[test]
fn test_join_3() {
    let m = Message::new("JOIN #yolo,#yolo2 key1").unwrap();
    assert_eq!(m.cmd, Command::JOIN(vec!(Channel {name: s!("#yolo"), key: Some(s!("key1"))}, Channel {name: s!("#yolo2"), key: None})));
}

#[test]
fn test_names() {
    let m = Message::new("NAMES #channel1").unwrap();
    assert_eq!(m.cmd, Command::NAMES(vec!["#channel1".to_string()], None));
}

#[test]
fn test_names2() {
    let m = Message::new("NAMES #channel1 servername").unwrap();
    assert_eq!(m.cmd, Command::NAMES(vec!["#channel1".to_string()], Some("servername".to_string())));
}

#[test]
fn test_names3() {
    let m = Message::new("NAMES #channel1,#channel2 servername").unwrap();
    assert_eq!(m.cmd, Command::NAMES(vec![s!("#channel1"), s!("#channel2")], Some(s!("servername"))));
}

