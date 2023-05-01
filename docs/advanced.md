# Advanced

This page will introduce you to more advanced uses of this application.

## Run multiple client applications on a single computer

Here is how to run multiple client applications on one computer.
The default configuration of osc_broadcaster is to receive messages on port 32000 and send messages to port 12000.
However, this will result in all client applications using port 12000 when running multiple client applications on a single computer.
The client applications will not work properly.
Therefore, it is necessary to set the destination port number of osc_broadcaster for each client application.
The way to do this is to specify the port number as the first argument of the OSC message when connecting.
For example, /server/connect 12001, /server/connect 12002, /server/connect 12003.
This will cause osc_broadcaster to broadcast messages to different port numbers for each client application.
Examples of concrete client application implementations are presented in [client_examples](./client_examples).
