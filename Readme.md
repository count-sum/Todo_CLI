## Minimal CLI Todo app in Rust

This is a simple CLI To-Do application built in Rust. It allows users to perform simple tasks such as add, mark as complete or delete items, (based on an id) using the terminal. For more details see the **commands** section below.

### Setup:
- ```cargo build```
---

### Commands:

#### Add item
- ```cargo run -- add "Item 1"```

#### Edit item
- ```cargo run -- edit 1 "Item 1 was edited"```

#### Mark item as complete
- ```cargo run -- complete 1```

#### Reset complete item
- ```cargo run -- reset 1```

#### Delete item
- ```cargo run -- delete 1```

#### List items
- ``` cargo run -- list```

----

