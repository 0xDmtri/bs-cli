## BS-CLI

*Very simple and minimal Black Scholes CLI implemented in Rust*
---


### Requirements:
1. Linux or macOS
2. Rustup: ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```


### Installation:
1. ```git clone https://github.com/0xDmtri/bs-cli.git```
2. ```sh ~/bs-cli/install.sh```


### Usage:
```bs -u <Underlying> -s <Strike> -d <Days> -r <Rate> -v <Volatility> ```


### Example:
``` bs -u 100.0 -s 100.0 -d 0.25 -v 0.8```


### Contributors:
1. ```git clone --branch develop https://github.com/0xDmtri/bs-cli.git```
2. ```cd bs-cli && cargo build```

---
### Note:
Currently, we are using hardcoded 360 days annualization convention for simplicity, feel free to submit a PR request to customize it!