use age::secrecy::Secret;
use clap::Parser;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(name = "badage")]
#[command(about = "Simple age-like encryption tool with passphrase as CLI flag")]
struct Args {
    #[arg(short, long, help = "Passphrase for encryption/decryption")]
    passphrase: String,

    #[arg(short, long, help = "Input file path or '-' for stdin")]
    input: String,

    #[arg(short, long, help = "Output file path or '-' for stdout")]
    output: String,

    #[arg(short, long, help = "Decrypt instead of encrypt")]
    decrypt: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let input_data = if args.input == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        std::fs::read(&args.input)?
    };

    let passphrase = Secret::new(args.passphrase);

    let output_data = if args.decrypt {
        let decryptor = match age::Decryptor::new(&input_data[..])? {
            age::Decryptor::Passphrase(d) => d,
            _ => return Err("Invalid encrypted data".into()),
        };

        let mut decrypted = Vec::new();
        let mut reader = decryptor.decrypt(&passphrase, None)?;
        reader.read_to_end(&mut decrypted)?;
        decrypted
    } else {
        let encryptor = age::Encryptor::with_user_passphrase(passphrase);

        let mut encrypted = Vec::new();
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(&input_data)?;
        writer.finish()?;
        encrypted
    };

    if args.output == "-" {
        io::stdout().write_all(&output_data)?;
    } else {
        std::fs::write(&args.output, &output_data)?;
    }

    Ok(())
}
