"use client"

import { Certificate } from '@/app/lib/types'
import { CertCard } from './CertCard'
import { Shield } from 'lucide-react'

interface CertListProps {
  certificates: Certificate[]
  isLoading?: boolean
  error?: string
}

export function CertList({ certificates, isLoading, error }: CertListProps) {
  if (isLoading) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <div className="animate-pulse flex flex-col items-center gap-4">
          <Shield className="h-12 w-12 text-muted" />
          <p className="text-muted-foreground">Loading certificates...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <Shield className="h-12 w-12 text-destructive mb-2" />
        <h3 className="text-xl font-semibold mb-2">Error Loading Certificates</h3>
        <p className="text-muted-foreground">{error}</p>
      </div>
    )
  }

  if (certificates.length === 0) {
    return (
      <div className="flex flex-col items-center justify-center py-12">
        <Shield className="h-12 w-12 text-muted mb-2" />
        <h3 className="text-xl font-semibold mb-2">No Certificates Found</h3>
        <p className="text-muted-foreground">This account has no certificates issued to it.</p>
      </div>
    )
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {certificates.map((cert) => (
        <CertCard key={cert.id} certificate={cert} />
      ))}
    </div>
  )
} 