use anyhow::{Context, Result};
use clap::Parser;
use qrcode::QrCode;
use qrcode::render::unicode;
use sp_core::crypto::Ss58Codec;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about = "QR code generator for ProofForge credentials")]
struct Cli {
    /// Account address to generate QR code for
    #[arg(short, long)]
    account: String,

    /// Output file path
    #[arg(short, long)]
    out: PathBuf,

    /// Base URL for the verification page (default: https://verisite.io/verify/)
    #[arg(short, long, default_value = "https://verisite.io/verify/")]
    base_url: String,

    /// QR code size (default: 8)
    #[arg(short, long, default_value_t = 8)]
    size: u16,

    /// Print QR code to terminal
    #[arg(short, long)]
    print: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // Validate account address
    let account = match args.account.parse::<sp_core::sr25519::Public>() {
        Ok(pub_key) => pub_key.to_ss58check(),
        Err(_) => {
            // If parsing fails, assume it's already an SS58 address
            args.account.clone()
        }
    };

    // Create the verification URL
    let verify_url = format!("{}{}", args.base_url, account);
    println!("Creating QR code for URL: {}", verify_url);

    // Generate QR code
    let code = QrCode::new(verify_url.as_bytes())
        .context("Failed to generate QR code")?;

    // Print QR code to terminal if requested
    if args.print {
        let image = code.render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("\n{}", image);
    }

    // Save QR code to file
    let image = code.render::<image::Luma<u8>>()
        .min_dimensions(args.size * 32, args.size * 32)
        .build();
    
    image.save(&args.out)
        .context(format!("Failed to save QR code to {:?}", args.out))?;
    
    println!("QR code saved to {:?}", args.out);
    
    Ok(())
} 