use age::secrecy::Secret;
use clap::Parser;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(name = "badage")]
#[command(about = "Simple age decryption tool with passphrase as CLI flag")]
struct Args {
    #[arg(short, long, help = "Passphrase for decryption")]
    passphrase: String,

    #[arg(short, long, help = "Input file path or '-' for stdin")]
    input: String,

    #[arg(short, long, help = "Output file path or '-' for stdout")]
    output: String,
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

    let decryptor = match age::Decryptor::new(&input_data[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => return Err("Invalid encrypted data".into()),
    };

    let mut output_data = Vec::new();
    let mut reader = decryptor.decrypt(&passphrase, None)?;
    reader.read_to_end(&mut output_data)?;

    if args.output == "-" {
        io::stdout().write_all(&output_data)?;
    } else {
        std::fs::write(&args.output, &output_data)?;
    }

    Ok(())
}
