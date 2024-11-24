![demo](./docs/demo/screenshot.png)

```toml
# $HOME/.config/rrss/config.toml

max_concurrency = 20

[theme]
date_format = "%a %H:%M %d-%m-%Y"
borders = false
column_spacing = 2
unread_marker = '•'
# ...

# [keybinds]
# cancel = ["todo"]
```

```toml
# $HOME/.config/rrss/sources.toml

[[sources]]
url = "..."
filter = { pattern = "cool regex", invert = false, case_insensitive = false }
max_items = 100

[[sources]]
# ...
```

todos

- hard update feeds (replace existing items)
- notify when specific feeds (feed source flag) gets new items after an update
- sort by columns
- global search in all items and feeds
- export as opml
- bookmarking/item ratings --> filter by / sort by
- define a grammar for queries so that it can be used in the tui too
    - target:feed_id if:unread if:(posted < date)...