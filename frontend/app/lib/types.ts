export interface Certificate {
  id: string;
  name: string;
  certType: string;
  owner: string;
  issuer: string;
  issuedAt: Date;
  expiresAt: Date | null;
  revoked: boolean;
  isExpired: boolean;
  metadata: Record<string, any>;
} 