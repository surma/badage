use age::secrecy::Secret;
use clap::{Parser, Subcommand};
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(name = "badage")]
#[command(about = "Simple age encryption/decryption tool with passphrase as CLI flag")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "Passphrase for encryption")]
    passphrase: String,

    #[arg(short, long, help = "Input file path or '-' for stdin")]
    input: String,

    #[arg(short, long, help = "Output file path or '-' for stdout")]
    output: String,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Encrypt a file or stream")]
    Encrypt(Args),
    #[command(about = "Decrypt a file or stream")]
    Decrypt(Args),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt(Args {
            passphrase,
            input,
            output,
        }) => {
            encrypt(passphrase, input, output)?;
        }
        Commands::Decrypt(Args {
            passphrase,
            input,
            output,
        }) => {
            decrypt(passphrase, input, output)?;
        }
    }

    Ok(())
}

fn encrypt(
    passphrase: String,
    input: String,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_data = if input == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        std::fs::read(&input)?
    };

    let passphrase = Secret::new(passphrase);
    let encryptor = age::Encryptor::with_user_passphrase(passphrase);

    let mut encrypted = vec![];
    let mut writer = encryptor.wrap_output(&mut encrypted)?;
    writer.write_all(&input_data)?;
    writer.finish()?;

    if output == "-" {
        io::stdout().write_all(&encrypted)?;
    } else {
        std::fs::write(&output, &encrypted)?;
    }

    Ok(())
}

fn decrypt(
    passphrase: String,
    input: String,
    output: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_data = if input == "-" {
        let mut buffer = Vec::new();
        io::stdin().read_to_end(&mut buffer)?;
        buffer
    } else {
        std::fs::read(&input)?
    };

    let passphrase = Secret::new(passphrase);

    let decryptor = match age::Decryptor::new(&input_data[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => return Err("Invalid encrypted data".into()),
    };

    let mut output_data = Vec::new();
    let mut reader = decryptor.decrypt(&passphrase, None)?;
    reader.read_to_end(&mut output_data)?;

    if output == "-" {
        io::stdout().write_all(&output_data)?;
    } else {
        std::fs::write(&output, &output_data)?;
    }

    Ok(())
}
