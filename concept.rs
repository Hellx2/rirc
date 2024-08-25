/// Handling the `ping` command
fn handle_ping(&self, msg: Command) {
    match msg {
        Command::Ping => self.pong(),
        _ => (),
    }
}

/// Handling the `names` command.
fn handle_names(&self, msg: Command) {
    match msg {
        Command::Names(channels, Some(server)) => channels.map(|c| self.send_names(c, server)),
        Command::Names(channels, None) => channels.map(|c| self.send_names(c)),
        _ => (),
    }
}

/// Handling the return of the `pong` command.
fn pong(&self) {
    let msg = create_msg(Command::Pong);
    self.socket.send(msg);
}

/// Function to ask the user for channel names.
fn ask_for_names(&self, channel: &str) {
    let msg = create_msg(Command::Names(vec![channel], None));
}
