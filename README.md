<h1 align="center">gif_server</h1>
<div align="center">
    Simple server to search GIFs
</div>

<h2>Run server</h2>

1. Change [config.toml](./config.toml) if necessary.
> If you change the port in [config.toml](./config.toml), do not forget to also
> change `run` command in the [justfile](./justfile):
> ```
> run: 
>    docker run --rm -p 8888:<your_port> --name gif_server gif_server
> ```

2. Run docker container:
```
just run
```
*or if you dont want run docker container, just use:*
```
cargo run
```