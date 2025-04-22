"use client"

import { ApiPromise, WsProvider } from '@polkadot/api'
import { Certificate } from './types'

let api: ApiPromise | null = null
const NODE_URL = process.env.NEXT_PUBLIC_NODE_URL || 'ws://127.0.0.1:9944'

export async function getApi(): Promise<ApiPromise> {
  if (!api) {
    const provider = new WsProvider(NODE_URL)
    api = await ApiPromise.create({ provider })
  }
  return api
}

export async function getAccountCertificates(accountId: string): Promise<Certificate[]> {
  try {
    const api = await getApi()
    
    // Call the custom RPC endpoint for certificates
    const rawCertificates = await api.rpc.certificates.getAccountCertificates(accountId)
    
    if (!rawCertificates) {
      return []
    }
    
    // Transform the data to match our Certificate interface
    const currentBlock = await api.query.system.number()
    const now = new Date()
    
    return rawCertificates.map((cert: any) => {
      const metadata = JSON.parse(Buffer.from(cert.metadata).toString('utf-8'))
      const issuedAt = new Date(metadata.issued_at * 1000)
      
      // Calculate expiry based on block number difference
      let expiresAt = null
      let isExpired = false
      
      if (cert.expires_at > 0) {
        // Approximate expiry date based on 12 second blocks
        const blockDiff = cert.expires_at - currentBlock.toNumber()
        const secondsRemaining = blockDiff * 12
        
        if (blockDiff <= 0) {
          isExpired = true
        }
        
        expiresAt = new Date(now.getTime() + (secondsRemaining * 1000))
      }
      
      return {
        id: cert.id.toString(),
        name: metadata.name,
        certType: metadata.cert_type,
        owner: cert.owner.toString(),
        issuer: cert.issuer.toString(),
        issuedAt,
        expiresAt,
        revoked: cert.revoked,
        isExpired,
        metadata
      }
    })
  } catch (error) {
    console.error('Error fetching certificates:', error)
    throw new Error('Failed to fetch certificates from the blockchain')
  }
}

export async function isCertificateValid(certId: string): Promise<boolean> {
  try {
    const api = await getApi()
    return await api.rpc.certificates.isCertificateValid(certId)
  } catch (error) {
    console.error('Error checking certificate validity:', error)
    throw new Error('Failed to check certificate validity')
  }
} 