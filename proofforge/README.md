# 🔐 ProofForge

> Forged in Rust. Proven on Chain.

ProofForge is a decentralized credential verification system for construction and industrial certifications.

## Components

### 🧱 Blockchain Node (`chain/`)
- Rust + Substrate implementation
- Custom pallet for Soulbound Token (SBT) certificates
- Store and verify training certifications on-chain

### 🔐 CLI Credential Issuer (`issuer-cli/`)
- Admin tool for credential management
- Issue and revoke certificates via Substrate RPC
- Generate verification QR codes

### 🌐 Credential Viewer Web App (`frontend/`)
- Next.js + Tailwind application
- Verify credentials by scanning QR codes or entering wallet addresses
- Display certification status and metadata

### 🌐 QR Generator Tool (`tools/qr-gen/`)
- Generate QR codes for credential verification
- Print badges for workers to display on-site

## Development

Each component has its own README with specific setup instructions.

## License

Apache 2.0 