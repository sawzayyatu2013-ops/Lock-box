# 🔐 lockbox

Encrypt & decrypt any file with a password. Cross-platform, single binary.

**AES-256-GCM** for encryption · **Argon2id** for key derivation · random salt & nonce per file.

## 🚀 How to Install & Run

If you have Rust installed on your system, you can build and install this tool directly to your path:

git clone https://github.com/sawzayyatu2013-ops/Lock-box.git
cd Lock-box
cargo install --path .

## 🛠️ Usage

lockbox encrypt secrets.txt
lockbox decrypt secrets.txt.lockbox

## 🧪 Run the Tests

cargo test

