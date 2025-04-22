"use client"

import { useState, useEffect } from 'react'
import { useParams } from 'next/navigation'
import { CertList } from '@/app/components/CertList'
import { getAccountCertificates } from '@/app/lib/api'
import { Certificate } from '@/app/lib/types'
import { Shield, User, ArrowLeft } from 'lucide-react'
import Link from 'next/link'

export default function VerifyAccount() {
  const params = useParams()
  const account = params.account as string
  
  const [certificates, setCertificates] = useState<Certificate[]>([])
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function loadCertificates() {
      try {
        setIsLoading(true)
        setError(null)
        
        const certs = await getAccountCertificates(account)
        setCertificates(certs)
      } catch (err) {
        console.error('Error loading certificates:', err)
        setError('Failed to load certificates from the blockchain')
      } finally {
        setIsLoading(false)
      }
    }
    
    if (account) {
      loadCertificates()
    }
  }, [account])

  // Format the account address for display (truncate middle)
  const formatAddress = (address: string) => {
    if (address.length <= 16) return address
    return `${address.slice(0, 8)}...${address.slice(-8)}`
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
      
      <div className="container py-8">
        <div className="mb-8 flex flex-col items-center text-center">
          <div className="flex justify-center mb-4">
            <div className="inline-flex h-12 w-12 items-center justify-center rounded-full bg-muted">
              <User className="h-6 w-6 text-primary" />
            </div>
          </div>
          <h1 className="text-3xl font-bold tracking-tight mb-2">Verifying Credentials</h1>
          <p className="text-lg text-muted-foreground mb-2">
            Account: <span className="font-mono">{formatAddress(account)}</span>
          </p>
          <div className="h-1 w-24 bg-primary my-4"></div>
        </div>
        
        <CertList 
          certificates={certificates}
          isLoading={isLoading}
          error={error || undefined}
        />
      </div>
    </div>
  )
} 