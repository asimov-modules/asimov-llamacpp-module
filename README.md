# ASIMOV Llamacpp Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-llamacpp-module)](https://crates.io/crates/asimov-llamacpp-module)
[![Documentation](https://docs.rs/asimov-llamacpp-module/badge.svg)](https://docs.rs/asimov-llamacpp-module)

[ASIMOV] Llamacpp module.

## ✨ Features

- To be determined!

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code
- You must have a **running llama.cpp server** with a **loaded GGUF model**

## ⬇️ Installation

### Installation with [ASIMOV CLI]

```bash
asimov module install llamacpp -v
```

### Installation from Source Code

```bash
cargo install asimov-llamacpp-module
```

## 👉 Examples

```bash
asimov-llamacpp-prompter
```

## ⚙ Configuration

Via ASIMOV CLI

```bash
asimov module config llamacpp
```

Via environment variables

```bash
export ASIMOV_LLAMACPP_API_ENDPOINT="http://127.0.0.1:8080"
export ASIMOV_LLAMACPP_MODEL="TinyLlama-1.1B-Chat-v1.0"

```

## 📚 Reference

### Prompt

```bash
echo "Why is the sky blue?" | asimov-llamacpp-prompter
```

### 🧪 Simple GBNF example

1) Create a tiny grammar that only allows `ok`:
```bnf
# only_ok.gbnf
root ::= "ok" "\n"
```
2) Run the prompter with the grammar:
```bash
echo "Say anything." | asimov-llamacpp-prompter -g only_ok.gbnf
# => ok
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-llamacpp-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-llamacpp-module&text=asimov-llamacpp-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-llamacpp-module&title=asimov-llamacpp-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-llamacpp-module&t=asimov-llamacpp-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-llamacpp-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-llamacpp-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[Rust]: https://rust-lang.org