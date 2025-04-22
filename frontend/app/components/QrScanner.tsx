"use client";

import { useEffect, useRef, useState } from 'react';
import { Html5Qrcode } from 'html5-qrcode';

interface QrScannerProps {
  onScan: (result: string) => void;
  fps?: number;
  qrbox?: number;
}

export function QrScanner({ onScan, fps = 10, qrbox = 250 }: QrScannerProps) {
  const [error, setError] = useState<string | null>(null);
  const [hasPermission, setHasPermission] = useState<boolean | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const scannerRef = useRef<Html5Qrcode | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Initialize scanner when component mounts
    if (containerRef.current) {
      scannerRef.current = new Html5Qrcode('qr-reader');
    }

    // Clean up on unmount
    return () => {
      if (scannerRef.current && isScanning) {
        scannerRef.current.stop().catch(error => console.error("Failed to stop scanner:", error));
      }
    };
  }, []);

  useEffect(() => {
    // Request camera permission and start scanning
    const startScanner = async () => {
      if (!scannerRef.current) return;

      try {
        setIsScanning(true);
        
        await scannerRef.current.start(
          { facingMode: "environment" }, // Prefer back camera
          {
            fps,
            qrbox: { width: qrbox, height: qrbox },
          },
          (decodedText) => {
            onScan(decodedText);
            // Don't stop scanner after successful scan to allow multiple scans
          },
          (errorMessage) => {
            // QR code parsing errors are handled here
            console.log(errorMessage);
          }
        );
        
        setHasPermission(true);
      } catch (err: any) {
        console.error("Scanner error:", err);
        
        if (err.toString().includes("permission")) {
          setError("Camera permission denied. Please allow camera access.");
          setHasPermission(false);
        } else {
          setError(`Failed to start scanner: ${err.toString()}`);
        }
        
        setIsScanning(false);
      }
    };

    if (scannerRef.current && !isScanning && hasPermission !== false) {
      startScanner();
    }

    return () => {
      if (scannerRef.current && isScanning) {
        scannerRef.current.stop().catch(error => console.error("Failed to stop scanner:", error));
        setIsScanning(false);
      }
    };
  }, [fps, qrbox, onScan, isScanning, hasPermission]);

  return (
    <div className="w-full flex flex-col items-center">
      <div 
        id="qr-reader" 
        ref={containerRef}
        className="w-full max-w-md overflow-hidden rounded-lg"
      />
      
      {error && (
        <div className="mt-4 p-3 bg-destructive/10 text-destructive rounded-md">
          <p>{error}</p>
          {hasPermission === false && (
            <button 
              onClick={() => setHasPermission(null)}
              className="mt-2 text-sm underline"
            >
              Try again
            </button>
          )}
        </div>
      )}
      
      <div className="mt-4 text-sm text-muted-foreground text-center">
        <p>Point your camera at a VeriSite QR code</p>
      </div>
    </div>
  );
} 