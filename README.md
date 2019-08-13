Battle.net OAuth token
===

Installation
---

```
battle-net-oauth = "0.1"
```

Get a token 
---

```rust
extern crate battle_net_oauth;

let token = battle_net_oauth::get_oauth_token();
```