import Link from 'next/link'
import { Shield, Database, QrCode, ListChecks } from 'lucide-react'

export default function Home() {
  return (
    <div className="flex flex-col min-h-screen">
      <header className="border-b">
        <div className="container flex h-16 items-center justify-between py-4">
          <div className="flex items-center gap-2">
            <Shield className="h-6 w-6 text-primary" />
            <span className="text-xl font-bold">VeriSite</span>
          </div>
          <nav className="flex items-center gap-4">
            <Link href="/verify" className="text-sm font-medium hover:underline">
              Verify
            </Link>
            <a 
              href="https://github.com/yourusername/verisite" 
              target="_blank" 
              rel="noopener noreferrer" 
              className="text-sm font-medium hover:underline"
            >
              GitHub
            </a>
          </nav>
        </div>
      </header>
      
      <main className="flex-1">
        <section className="py-24 md:py-32 bg-muted">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <div className="flex justify-center mb-4">
                  <div className="inline-block rounded-lg bg-primary p-2 text-primary-foreground">
                    <Shield className="h-16 w-16" />
                  </div>
                </div>
                <h1 className="text-3xl font-bold tracking-tighter sm:text-5xl md:text-6xl">
                  VeriSite
                </h1>
                <p className="text-xl text-muted-foreground">
                  Forged in Rust. Proven on Chain.
                </p>
              </div>
              <div className="max-w-[700px] text-muted-foreground">
                <p className="text-lg">
                  A decentralized credential verification system for construction and industrial certifications.
                </p>
              </div>
              <div className="flex flex-col sm:flex-row gap-4 mt-8">
                <Link 
                  href="/verify" 
                  className="inline-flex h-10 items-center justify-center rounded-md bg-primary px-8 text-sm font-medium text-primary-foreground shadow"
                >
                  Verify Credentials
                </Link>
                <a 
                  href="https://github.com/yourusername/verisite" 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="inline-flex h-10 items-center justify-center rounded-md border border-input bg-background px-8 text-sm font-medium shadow-sm"
                >
                  View Source
                </a>
              </div>
            </div>
          </div>
        </section>
        
        <section className="py-16 md:py-24">
          <div className="container px-4 md:px-6">
            <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3 lg:gap-12">
              <div className="space-y-4">
                <div className="inline-flex items-center justify-center rounded-lg bg-muted p-2">
                  <Database className="h-8 w-8 text-primary" />
                </div>
                <h3 className="text-xl font-bold">Blockchain Backed</h3>
                <p className="text-muted-foreground">
                  Credentials are stored as Soulbound Tokens on a custom Substrate blockchain for tamper-proof verification.
                </p>
              </div>
              <div className="space-y-4">
                <div className="inline-flex items-center justify-center rounded-lg bg-muted p-2">
                  <ListChecks className="h-8 w-8 text-primary" />
                </div>
                <h3 className="text-xl font-bold">Industry Certifications</h3>
                <p className="text-muted-foreground">
                  Issue and verify construction certifications like WHMIS, Rigging, Tower Crane, and more.
                </p>
              </div>
              <div className="space-y-4">
                <div className="inline-flex items-center justify-center rounded-lg bg-muted p-2">
                  <QrCode className="h-8 w-8 text-primary" />
                </div>
                <h3 className="text-xl font-bold">Easy Verification</h3>
                <p className="text-muted-foreground">
                  Scan a QR code to instantly verify a worker's credentials on-site or look up by account.
                </p>
              </div>
            </div>
          </div>
        </section>
      </main>
      
      <footer className="border-t py-6 md:py-0">
        <div className="container flex flex-col items-center justify-between gap-4 md:h-16 md:flex-row">
          <p className="text-sm text-muted-foreground">
            © 2023 ProofForge. All rights reserved.
          </p>
          <p className="text-sm text-muted-foreground">
            Licensed under Apache 2.0
          </p>
        </div>
      </footer>
    </div>
  )
} 