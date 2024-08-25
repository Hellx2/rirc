# Motivation

* learning rust
* learn to implment a protocol
* experiment with typed irc commands

## typed irc commands

When looking at irc libraries all of them keep the irc command more or less ‘untyped’.

````python
def handle_command(cmd):
    if cmd.starts_with(“PING”):
        send_pong()
````

or a bit more typed

````csharp
public void HandleJoin(IrcMsg msg)
{
    switch (msg.Cmd)
    {
        case IrcCommand.JOIN:
            assert(msg.Params.GetType() == typeof(List<string>));
            // do something with params.
    }
}
````

What if this could be super typed:

````rust
// struct ChannelName(String);

struct Channel {
    name: String, // or: name: ChannelName
    key: Option<String>
}

enum IrcCommand {
    JOIN(Vec<Channel>),
    ...
}

fn handle_join(msg: IrcMsg) {
    match IrcMsg.cmd {
        IrcCommand::JOIN(channels) => {
            // do something with all the typed channels.
        },
        _ => panic!(“why was I called with something other than IrcCommand::JOIN)”)
    }
}
````
