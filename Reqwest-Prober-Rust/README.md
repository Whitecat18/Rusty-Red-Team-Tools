# Request Prober Using Rust 

## What is this ?!

This program is to checks active sites on the list and returns HTTP response status code. Mainly i have written to test my firewall project and Load balancing in firewall .

## Features

- **Protocol Agnostic**: Automatically detects and checks both HTTP and HTTPS versions of URLs if the protocol is not specified.
- **Concurrent Processing**: Employs asynchronous tasks to concurrently check multiple URLs, enhancing speed and efficiency.
- **Color-Coded Output**: Utilizes color-coding to represent different response statuses for easy visualization.
- **Timeout Handling**: Manages timeouts effectively, indicating URLs that take too long to respond.

## How to Use

### Installation

1. **Clone the Repository**

```
git clone https://github.com/Whitecat18/Reqwest-Prober-Rust.git
cd Reqwest-Prober-Rust
```

2 . **To run the Program**

```
cargo run -- url_file.txt 
```
## Demo Video 

[reqwest_prober-demo.webm](https://github.com/Whitecat18/Reqwest-Prober-Rust/assets/96696929/ca777b66-94e5-43f7-a1f3-858aad563f9d)



## Bug Fixes and Errors 

### For Debian Based Users Ubuntu/Kali 

<details>
  <summary><b>Detailed Error Info</b></summary>
  The Following Error Occures while testing on Kali Linux .<br>
  
  ```
  run pkg_config fail: 
  pkg-config exited with status code 1
  > PKG_CONFIG_ALLOW_SYSTEM_CFLAGS=1 pkg-config --libs --cflags openssl
  ```

</details>

If you face an openssl error while building, install the following dependics and try again ! 

```
sudo apt install libssl-dev pkg-config libudev-dev
```

### For Arch Users

```
cargo run -- url_file.txt 
```

## Advantages

- **Efficiency**: Concurrent checking of URLs speeds up the process, especially when dealing with numerous URLs.
- **Ease of Use**: Simple command-line interface, requiring just a file path containing URLs.
- **Visual Clarity**: Color-coded output aids in quickly identifying the status of each URL.
