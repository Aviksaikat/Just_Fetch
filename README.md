# 🌟 Just Fetch (jfetch) 🌟

> Saikat Karmakar | 17 Jun 2024

![](media/jfetch_banner.png)

<center>

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

</center>

Just Fetch, or simply **jfetch**, is a super cool command-line system information tool written in Rust. It's fast, efficient, and gives you all the essential system information in a neat and organized way, just like neofetch but in Rust! 🚀

**Just Fetch** - because fetching system info should be Just simple and cool. ✨



## 🚀 Features

Just Fetch displays the following system information:

- ❯ **OS**
- ❯ **Kernel**
- ❯ **Architecture**
- ❯ **Hostname**
- ❯ **Shell**
- ❯ **Uptime**
- ❯ **Disk**
- ❯ **Memory**

## 🎉 Installation

To install jfetch, you need to have Rust installed. Then run:

```sh
cargo install jfetch
```

### Build from source

```sh
git clone https://github.com/Aviksaikat/Just_Fetch
cd Just_fetch
cargo build --bin jfetch
# run
cargo run --bin jfetch
```

## 🛠 Usage

## Demo
![](media/demo.gif)

Simply run:

```sh
jfetch
```

This will display all your system information in a clean and organized format. 🖥️

## 🖼 Example Output

```sh
❯ OS:            Ubuntu 20.04.2 LTS
❯ Kernel:        5.8.0-53-generic
❯ Architecture:  x86_64
❯ Hostname:      myhostname
❯ Shell:         zsh
❯ Uptime:        5 days, 3 hours, 22 minutes
❯ Disk:          50GB / 200GB
❯ Memory:        8GB / 16GB
❯ Battery        100%
```

## Help screen

```sh
jfetch -h

Simple command-line system information tool written tool like neofetch but in rust 🦀

Usage: jfetch [OPTIONS]

Options:
  -n, --no-banner
  -h, --help       Print help
  -V, --version    Print version
```


## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🚧 TODOs

Here are some features and improvements planned for future releases:

- 🔍 **Add CPU information** 
- 🌐 **Add network information**
- 💻 **Add GPU information**
- 📦 **Add package manager information**
- 📊 **Display system temperature**
- 🎨 **Add theme and color customization**
- 📅 **Show system load averages**
- 📝 **Log system information to a file**
- **Add custom configuration file**
- **Add asscii banners**

## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## 📬 Contact

For any questions, feel free to open an [issue](https://github.com/Aviksaikat/Just_Fetch/issues).

---


⭐️ Don't forget to give this project a star on [GitHub](https://github.com/Aviksaikat/Just_Fetch) if you find it useful!

---

Happy Fetching! 🥳

---

