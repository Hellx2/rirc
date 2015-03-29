
fn handle_ping(&self, msg: Command) {
    match msg {
        Command::PING => self.pong(),
        _ => ()
    }
}

fn handle_names(&self, msg: Command) {
    match msg {
        Command::NAMES(channels, Some(server)) => channels.map(|c| self.send_names(c, server)),
        Command::NAMES(channels, None) => channels.map(|c| self.send_names(c)),
        _ => ()
    }
}


fn pong(&self) {
    let msg = create_msg(Command::PONG);
    self.socket.send(msg);
}

fn ask_for_names(&self, channel: &str) {
    let msg = create_msg(Command::NAMES(vec![channel.to_string()], None))
}
