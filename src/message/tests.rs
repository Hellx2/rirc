use command::{Channel, Command};
use message::Message;

#[test]
fn test_nick() {
    let msg = Message::new("NICK yolo").unwrap();
    assert_eq!(msg.cmd, Command::Nick("yolo", None));

    let msg = Message::new("NICK yolo 4").unwrap();
    assert_eq!(msg.cmd, Command::Nick("yolo", Some(4)));
}

#[test]
fn test_join() {
    let msg = Message::new("JOIN #yolo").unwrap();
    assert_eq!(
        msg.cmd,
        Command::Join(vec!(Channel {
            name: "#yolo",
            key: None
        }))
    );

    let msg = Message::new("JOIN #yolo key1").unwrap();
    assert_eq!(
        msg.cmd,
        Command::Join(vec!(Channel {
            name: "#yolo",
            key: Some("key1")
        }))
    );

    let msg = Message::new("JOIN #yolo,#yolo2 key1").unwrap();
    assert_eq!(
        msg.cmd,
        Command::Join(vec!(
            Channel {
                name: "#yolo",
                key: Some("key1")
            },
            Channel {
                name: "#yolo2",
                key: None
            }
        ))
    );
}

#[test]
fn test_names() {
    let m = Message::new("NAMES #channel1").unwrap();
    assert_eq!(m.cmd, Command::Names(vec!["#channel1"], None));

    let m = Message::new("NAMES #channel1 servername").unwrap();
    assert_eq!(m.cmd, Command::Names(vec!["#channel1"], Some("servername")));

    let m = Message::new("NAMES #channel1,#channel2 servername").unwrap();
    assert_eq!(
        m.cmd,
        Command::Names(vec!["#channel1", "#channel2"], Some("servername"))
    );
}
