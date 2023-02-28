# lili

local helper

## exec

lili reads `cfg.yaml` from default location (~/.lili/executor/cfg.yaml) or specified `lili exec -c your_path`, and execute user-defined applications. You can also enable `debug_output` to check on logs under `~/.lili/debug`.

sample config:

```yaml
processes:
  sync:
    cmd: syncthing
  naive:
    cmd: ~/.bin/naive
    args: ~/.bin/naive.json
debug_output: true
```
