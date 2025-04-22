"use client"

import { Certificate } from '@/app/lib/types'
import { ShieldCheck, Clock, Shield, ShieldX } from 'lucide-react'
import { formatDistanceToNow } from 'date-fns'

interface CertCardProps {
  certificate: Certificate
}

export function CertCard({ certificate }: CertCardProps) {
  const getStatusColor = (cert: Certificate) => {
    if (cert.revoked) return 'text-destructive'
    if (cert.isExpired) return 'text-yellow-500'
    return 'text-green-500'
  }

  const getStatusIcon = (cert: Certificate) => {
    if (cert.revoked) return <ShieldX className="h-6 w-6" />
    if (cert.isExpired) return <Clock className="h-6 w-6" />
    return <ShieldCheck className="h-6 w-6" />
  }

  const getStatusText = (cert: Certificate) => {
    if (cert.revoked) return 'Revoked'
    if (cert.isExpired) return 'Expired'
    return 'Active'
  }

  return (
    <div className="rounded-lg border bg-card text-card-foreground shadow-sm">
      <div className="p-6 flex flex-col gap-4">
        <div className="flex items-start justify-between">
          <div>
            <h3 className="text-xl font-semibold mb-1">{certificate.name}</h3>
            <p className="text-sm text-muted-foreground">{certificate.certType}</p>
          </div>
          <div className={`${getStatusColor(certificate)} flex items-center gap-1 font-medium`}>
            {getStatusIcon(certificate)}
            <span>{getStatusText(certificate)}</span>
          </div>
        </div>
        
        <div className="grid gap-2">
          <div className="grid grid-cols-2 items-center gap-4">
            <div className="flex flex-col">
              <span className="text-sm font-medium text-muted-foreground">Issued</span>
              <span>{formatDistanceToNow(certificate.issuedAt, { addSuffix: true })}</span>
            </div>
            <div className="flex flex-col">
              <span className="text-sm font-medium text-muted-foreground">Expires</span>
              <span>
                {certificate.expiresAt ? (
                  formatDistanceToNow(certificate.expiresAt, { addSuffix: true })
                ) : (
                  "Never"
                )}
              </span>
            </div>
          </div>
        </div>
        
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <Shield className="h-4 w-4" />
          <span>ID: {certificate.id}</span>
        </div>
      </div>
    </div>
  )
} 