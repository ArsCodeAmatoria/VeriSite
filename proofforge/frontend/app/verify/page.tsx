"use client"

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { Shield, Search, User, ArrowLeft } from 'lucide-react'
import Link from 'next/link'

export default function VerifyPage() {
  const router = useRouter()
  const [accountAddress, setAccountAddress] = useState('')
  const [error, setError] = useState<string | null>(null)

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    
    // Basic validation
    if (!accountAddress.trim()) {
      setError('Please enter an account address')
      return
    }
    
    // Clear any previous errors
    setError(null)
    
    // Navigate to the account verification page
    router.push(`/verify/${accountAddress.trim()}`)
  }

  return (
    <div className="flex flex-col min-h-screen">
      <header className="border-b">
        <div className="container flex h-16 items-center justify-between py-4">
          <div className="flex items-center gap-2">
            <Link href="/">
              <div className="flex items-center gap-2">
                <Shield className="h-6 w-6 text-primary" />
                <span className="text-xl font-bold">ProofForge</span>
              </div>
            </Link>
          </div>
          <Link 
            href="/"
            className="flex items-center gap-1 text-sm font-medium hover:underline"
          >
            <ArrowLeft className="h-4 w-4" />
            <span>Back to Home</span>
          </Link>
        </div>
      </header>
      
      <div className="container max-w-xl py-16 flex flex-col items-center">
        <div className="mb-8 flex flex-col items-center text-center">
          <div className="flex justify-center mb-4">
            <div className="inline-flex h-12 w-12 items-center justify-center rounded-full bg-muted">
              <User className="h-6 w-6 text-primary" />
            </div>
          </div>
          <h1 className="text-3xl font-bold tracking-tight mb-2">Verify Credentials</h1>
          <p className="text-lg text-muted-foreground">
            Enter an account address to view and verify their credentials
          </p>
          <div className="h-1 w-24 bg-primary my-4"></div>
        </div>
        
        <form onSubmit={handleSubmit} className="w-full space-y-4">
          <div className="space-y-2">
            <label htmlFor="account" className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
              Account Address
            </label>
            <div className="relative">
              <input
                id="account"
                type="text"
                className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                placeholder="Enter Substrate address or public key"
                value={accountAddress}
                onChange={(e) => setAccountAddress(e.target.value)}
              />
            </div>
            {error && (
              <p className="text-sm text-destructive">{error}</p>
            )}
          </div>
          
          <button
            type="submit"
            className="inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow hover:bg-primary/90 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring w-full"
          >
            <Search className="mr-2 h-4 w-4" />
            Verify Account
          </button>
        </form>
        
        <div className="mt-8 text-center text-sm text-muted-foreground">
          <p>You can also scan a QR code to verify credentials directly.</p>
        </div>
      </div>
    </div>
  )
} 