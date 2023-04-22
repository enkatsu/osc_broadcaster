OSC (Open Sound Control) broadcast server that can be started from the command line.

This is a Rust implementation of [oscP5broadcaster](https://sojamo.de/libraries/oscP5/examples/oscP5broadcaster/oscP5broadcaster.pde).
oscP5broadcaster is a sample application of [oscP5](https://sojamo.de/libraries/oscP5/).

# Usage

```
USAGE:
    osc_broadcaster [OPTIONS]

OPTIONS:
    -h, --help                         Print help information
    -l, --listen-port <LISTEN_PORT>    [default: 32000]
    -s, --send-port <SEND_PORT>        [default: 12000]
    -V, --version                      Print version information
```

## Connect

You can register as a target client from the distribution by sending an OSC message to osc_broadcast with the address pattern "/server/connect" from your client application.

```
(osc_broadcaster) <-{ port: 32000, OSC: /server/connect }- (client)
```

## Broadcast

When the destination clients are registered with osc_broadcast, sending an OSC message to osc_broadcast will distribute the message to all destination clients.

```
# Send from client
(osc_broadcaster) <-{ port: 32000, OSC: /your/osc/addr "hello" }- (client)
```

```
# Send to client
(osc_broadcaster) -{ port: 12000, OSC: /your/osc/addr "hello" }-> (client)
```

## Disconnect

You can exclude a target client from the distribution by sending an OSC message to "osc_broadcast" with the address pattern "/server/disconnect" from your client application.

```
(osc_broadcaster) <-{ port: 32000, OSC: /server/disconnect }- (client)
```
