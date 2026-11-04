use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rpassword::prompt_password;
use std::fs;
use std::path::PathBuf;

mod crypto;

#[derive(Parser)]
#[command(name = "lockbox", version, about = "Encrypt/decrypt files with a password (AES-256-GCM + Argon2id)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Decrypt a file
    Decrypt {
        input: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn default_encrypt_out(input: &PathBuf) -> PathBuf {
    let mut p = input.clone();
    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
    p.set_file_name(format!("{name}.lockbox"));
    p
}

fn default_decrypt_out(input: &PathBuf) -> PathBuf {
    let mut p = input.clone();
    let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
    let name = match name.strip_suffix(".lockbox") {
        Some(s) => s.to_string(),
        None => format!("{name}.dec"),
    };
    p.set_file_name(name);
    p
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output } => {
            let data = fs::read(&input).with_context(|| format!("reading {}", input.display()))?;
            let pw = prompt_password("Password: ")?;
            let confirm = prompt_password("Confirm:  ")?;
            if pw != confirm {
                anyhow::bail!("passwords do not match");
            }
            let enc = crypto::encrypt(&data, pw.as_bytes())?;
            let out = output.unwrap_or_else(|| default_encrypt_out(&input));
            fs::write(&out, enc).with_context(|| format!("writing {}", out.display()))?;
            println!("🔒 Encrypted -> {}", out.display());
        }
        Commands::Decrypt { input, output } => {
            let data = fs::read(&input).with_context(|| format!("reading {}", input.display()))?;
            let pw = prompt_password("Password: ")?;
            let dec = crypto::decrypt(&data, pw.as_bytes())?;
            let out = output.unwrap_or_else(|| default_decrypt_out(&input));
            fs::write(&out, dec).with_context(|| format!("writing {}", out.display()))?;
            println!("🔓 Decrypted -> {}", out.display());
        }
    }
    Ok(())
}
