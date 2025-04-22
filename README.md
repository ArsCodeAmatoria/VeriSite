# VeriSite

> Forged in Rust. Proven on Chain.

VeriSite is a decentralized credential verification system for construction and industrial certifications. It uses blockchain technology to securely store and verify credentials, ensuring authenticity and preventing forgery.

## Overview

VeriSite helps construction companies, training providers, and safety inspectors verify that workers on-site have valid training credentials. The system:

- Issues tamper-proof digital credentials as Soulbound Tokens (SBTs)
- Provides instant verification via QR codes
- Maintains an immutable record of credential history
- Works offline with progressive web app capabilities

## Components

### Blockchain Node (`chain/`)
- Rust + Substrate implementation
- Custom pallet for Soulbound Token (SBT) certificates
- Store and verify training certifications on-chain
- Provides secure, immutable credential storage

### CLI Credential Issuer (`issuer-cli/`)
- Admin tool for credential management
- Issue and revoke certificates via Substrate RPC
- Generate verification QR codes
- Batch operations for credential management

### Credential Viewer Web App (`frontend/`)
- Next.js + Tailwind application
- Progressive Web App (PWA) for mobile support
- Verify credentials by scanning QR codes or entering wallet addresses
- Display certification status and metadata
- Mobile-friendly interface for on-site verification

### QR Generator Tool (`tools/qr-gen/`)
- Generate QR codes for credential verification
- Print badges for workers to display on-site
- Create printable credentials with embedded verification links

### Proof Verification Library (`proofforge/`)
- Core verification logic
- Used by other components to validate credential proofs
- Compatible with standard cryptographic libraries

## Getting Started

### Prerequisites
- Rust toolchain (for blockchain and CLI components)
- Node.js and npm (for frontend)
- Docker and Docker Compose (for containerized deployment)

### Quick Start
1. Clone the repository
   ```
   git clone https://github.com/yourusername/VeriSite.git
   cd VeriSite
   ```

2. Start the development environment
   ```
   docker-compose up
   ```

3. Access the components:
   - Web interface: http://localhost:3000
   - Blockchain explorer: http://localhost:8000
   - RPC endpoint: http://localhost:9944

## Development

Each component has its own README with specific setup instructions.

### Running Individual Components

Check the README in each component directory for detailed setup and development instructions.

## Deployment

Deployment guides for production environments are available in the deployment documentation.

## Contributing

We welcome contributions to VeriSite! Please see our contributing guidelines for more details.

## License

Apache 2.0 