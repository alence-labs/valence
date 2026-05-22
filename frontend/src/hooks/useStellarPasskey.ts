import { useState } from 'react';

export function useStellarPasskey() {
  const [connected, setConnected] = useState(false);
  const [account, setAccount] = useState<string>('');

  async function connectWithPasskey() {
    try {
      if (!window.PublicKeyCredential) {
        return;
      }

      const credential = await navigator.credentials.create({
        publicKey: {
          challenge: Uint8Array.from('valence-challenge', (c) => c.charCodeAt(0)),
          rp: { name: 'Valence Network' },
          user: {
            id: Uint8Array.from('operator', (c) => c.charCodeAt(0)),
            name: 'operator@valence.network',
            displayName: 'Valence Operator',
          },
          pubKeyCredParams: [{ alg: -7, type: 'public-key' }],
          authenticatorSelection: { residentKey: 'preferred', userVerification: 'preferred' },
          timeout: 60000,
        },
      } as PublicKeyCredentialCreationOptions);

      if (!credential) {
        return;
      }

      const rawId = new Uint8Array(credential.rawId);
      const base64Key = btoa(String.fromCharCode(...rawId));
      setAccount(`passkey-${base64Key.slice(0, 16)}`);
      setConnected(true);
    } catch (error) {
      console.error('Passkey connect failed', error);
    }
  }

  return { connected, account, connectWithPasskey };
}
