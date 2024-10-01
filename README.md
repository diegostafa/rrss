```toml
# $HOME/.config/rrss/config.toml

max_concurrency = 20
dim_filtered_items = true
date_format = "%a %H:%M %d-%m-%Y"
```

```toml
# $HOME/.config/rrss/sources.toml

[[sources]]
url = "..."
filter = "cool regex"
invert_filter = false
max_items = 100

[[sources]]
...
```
![demo](./docs/demo/screenshot.png)

todos

- sort by column
- gen keymap from config
- global item search
- widget style in config (borders, colors ...)
- default view and sorting options in config
- more cli query commands ??
- save item as plain text ??
- open with custom program ??