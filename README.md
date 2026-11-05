cat > README.md <<'EOF'
# 🔐 lockbox

Encrypt & decrypt any file with a password. Cross-platform, single binary.

**AES-256-GCM** for encryption · **Argon2id** for key derivation · random salt & nonce per file.

## 🚀 How to Install & Run

If you have Rust installed on your system, you can build and install this tool directly to your path:

git clone https://github.com/sawzayyatu2013-ops/Lock-box.git
cd Lock-box
cargo install --path .

## 🛠️ Usage

Encrypt a file (creates secrets.txt.lockbox):
lockbox encrypt secrets.txt

Decrypt the file (creates secrets.txt):
lockbox decrypt secrets.txt.lockbox

You can also specify a custom output name:
lockbox encrypt document.pdf -o vault.bin
lockbox decrypt vault.bin -o document.pdf

## 🧠 How it works (Under the hood)

This tool does not just "scramble" your files. It uses military-grade cryptography to ensure your data is safe. 

When you run `lockbox encrypt`, here is exactly what happens:

1. **Key Derivation (Argon2id):** Your password is passed through Argon2id along with a randomly generated 16-byte salt. Argon2id is memory-hard, meaning it requires a lot of RAM to compute. This makes it virtually impossible for hackers to use GPUs to brute-force your password.
2. **Encryption (AES-256-GCM):** The derived key is used with AES-256-GCM to encrypt the file. GCM provides Authenticated Encryption, meaning if a single byte of the encrypted file is modified or corrupted, decryption will fail automatically (protecting against tampering).
3. **Random Nonce:** A random 12-byte nonce is generated for every single file. This ensures that if you encrypt the exact same file twice with the same password, the output will look completely different both times.

### 📁 The File Format
Every `.lockbox` file is structured like this:
[salt (16 bytes)] + [nonce (12 bytes)] + [encrypted data + auth tag]

## 🧪 Run the Tests

cargo test
EOF

