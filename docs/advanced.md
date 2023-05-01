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

## Load initial setting file

osc_broadcaster can be changed from options, some settings can be changed.
However, there are times when you may want to share these settings for development or distribute the settings along with the source code.
So I will show you how to set up the initial settings using a configuration file.
You can read the configuration file and start it with the `-f` option. The configuration file supports JSON, YAML, TOML, and CSV (CSV supports only limited configuration items).
Examples of specific configuration files are located in the repository.

```shell
osc_broadcaster -f settings.json
osc_broadcaster -f settings.yaml
osc_broadcaster -f settings.toml
osc_broadcaster -f settings.csv
```

- [JSON](https://github.com/enkatsu/osc_broadcaster/blob/main/docs/sample.json)
- [YAML](https://github.com/enkatsu/osc_broadcaster/blob/main/docs/sample.yaml)
- [TOML](https://github.com/enkatsu/osc_broadcaster/blob/main/docs/sample.toml)
- [CSV](https://github.com/enkatsu/osc_broadcaster/blob/main/docs/sample.csv)
