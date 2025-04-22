use anyhow::{Context, Result};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};
use codec::{Decode, Encode};
use qrcode::QrCode;
use qrcode::render::unicode;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::Ss58Codec, sr25519::Pair};
use std::path::PathBuf;
use substrate_api_client::{
    rpc::WsRpcClient,
    Api, XtStatus,
};
use tracing::{info, error};

const DEFAULT_NODE_URL: &str = "ws://127.0.0.1:9944";

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// URL of the Substrate node to connect to
    #[arg(short, long, default_value = DEFAULT_NODE_URL)]
    node_url: String,

    /// Path to the keyfile
    #[arg(short, long)]
    keyfile: Option<PathBuf>,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Issue a new certificate
    Issue {
        /// Address of the certificate recipient
        #[arg(short, long)]
        to: String,

        /// Name of the certificate
        #[arg(short, long)]
        cert: String,

        /// Type of the certificate (e.g., WHMIS, Rigging, Tower Crane)
        #[arg(short, long)]
        r#type: String,

        /// Expiry date in YYYY-MM-DD format
        #[arg(short, long)]
        expiry: String,

        /// Generate QR code for verification
        #[arg(short, long)]
        generate_qr: bool,

        /// Path to save the QR code image (default: credential_<id>.png)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Revoke an existing certificate
    Revoke {
        /// ID of the certificate to revoke
        #[arg(short, long)]
        id: u32,
    },
}

#[derive(Debug, Serialize, Deserialize, Encode, Decode)]
struct CertificateMetadata {
    name: String,
    cert_type: String,
    issuer: String,
    issued_at: u64,
}

async fn issue_certificate(
    api: &Api<Pair, WsRpcClient>,
    to: &str,
    cert_name: &str,
    cert_type: &str,
    expiry: &str,
    generate_qr: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    // Parse address
    let to_account = to.parse()
        .context("Failed to parse recipient address")?;

    // Parse expiry date
    let expiry_date = NaiveDate::parse_from_str(expiry, "%Y-%m-%d")
        .context("Failed to parse expiry date. Use YYYY-MM-DD format")?;
    
    // Convert to block number (approximation: 1 day = 7200 blocks at 12-second block time)
    let now = chrono::Local::now().date_naive();
    let days_until_expiry = expiry_date.signed_duration_since(now).num_days();
    if days_until_expiry < 0 {
        anyhow::bail!("Expiry date cannot be in the past");
    }
    
    let blocks_until_expiry = (days_until_expiry as u32) * 7200;
    let current_block = api.get_block_number(None)
        .context("Failed to get current block number")?;
    let expiry_block = current_block + blocks_until_expiry;

    // Create metadata
    let metadata = CertificateMetadata {
        name: cert_name.to_string(),
        cert_type: cert_type.to_string(),
        issuer: api.signer().unwrap().public().to_ss58check(),
        issued_at: chrono::Utc::now().timestamp() as u64,
    };
    
    let metadata_json = serde_json::to_string(&metadata)
        .context("Failed to serialize metadata")?;

    // Prepare and send the transaction
    info!("Issuing certificate to {}", to);
    info!("Certificate: {} ({})", cert_name, cert_type);
    info!("Expires at block: {}", expiry_block);
    
    let xt = api.create_signed(
        subxt::tx::Payload::new(
            "Certificates",
            "issue_cert",
            (to_account, metadata_json.as_bytes().to_vec(), expiry_block),
        ),
        subxt::tx::PairSigner::new(api.signer().unwrap().clone()),
    );
    
    let tx_hash = api.send_extrinsic(xt.hex(), XtStatus::InBlock)
        .context("Failed to send transaction")?;
    
    info!("Certificate issued! Transaction hash: {}", tx_hash);
    
    // TODO: Parse events to get the certificate ID
    let cert_id = 1; // Placeholder until we properly extract from events
    
    // Generate QR code if requested
    if generate_qr {
        generate_qr_code(to, cert_id, output)?;
    }
    
    Ok(())
}

async fn revoke_certificate(
    api: &Api<Pair, WsRpcClient>,
    cert_id: u32,
) -> Result<()> {
    info!("Revoking certificate with ID: {}", cert_id);
    
    let xt = api.create_signed(
        subxt::tx::Payload::new(
            "Certificates",
            "revoke_cert",
            (cert_id,),
        ),
        subxt::tx::PairSigner::new(api.signer().unwrap().clone()),
    );
    
    let tx_hash = api.send_extrinsic(xt.hex(), XtStatus::InBlock)
        .context("Failed to send transaction")?;
    
    info!("Certificate revoked! Transaction hash: {}", tx_hash);
    
    Ok(())
}

fn generate_qr_code(account: &str, cert_id: u32, output_path: Option<PathBuf>) -> Result<()> {
    // Create URL for verification
    let verify_url = format!("https://verisite.io/verify/{}", account);
    
    // Generate QR code
    let code = QrCode::new(verify_url.as_bytes())
        .context("Failed to generate QR code")?;
    
    // Output QR code to terminal
    let image = code.render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    
    println!("\nScan this QR code to verify the credential:\n");
    println!("{}", image);
    
    // Save QR code image if requested
    if let Some(path) = output_path {
        let output_path = path;
        let img = code.render::<image::Luma<u8>>().build();
        img.save(&output_path)
            .context(format!("Failed to save QR code to {:?}", output_path))?;
        info!("QR code saved to {:?}", output_path);
    } else {
        let default_path = format!("credential_{}.png", cert_id);
        let img = code.render::<image::Luma<u8>>().build();
        img.save(&default_path)
            .context(format!("Failed to save QR code to {}", default_path))?;
        info!("QR code saved to {}", default_path);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Connect to node
    info!("Connecting to node at {}", cli.node_url);
    let client = WsRpcClient::new(&cli.node_url)
        .context("Failed to create WebSocket RPC client")?;
    
    // TODO: Load key from file if provided
    let api = Api::<Pair, _>::new(client)
        .context("Failed to create API")?;
    
    // Execute subcommand
    match cli.command {
        Commands::Issue {
            to,
            cert,
            r#type,
            expiry,
            generate_qr,
            output,
        } => {
            issue_certificate(&api, &to, &cert, &r#type, &expiry, generate_qr, output).await?;
        }
        Commands::Revoke { id } => {
            revoke_certificate(&api, id).await?;
        }
    }
    
    Ok(())
} 