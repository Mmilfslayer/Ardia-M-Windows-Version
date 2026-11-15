export async function getRandomValues(array: Uint8Array | Uint16Array | Uint32Array | Int8Array | Int16Array | Int32Array): Promise<void> {
  if (typeof window !== 'undefined' && window.crypto && window.crypto.getRandomValues) {
    window.crypto.getRandomValues(array);
  } else {
    // Fallback for non-browser environments (shouldn't happen in Tauri)
    for (let i = 0; i < array.length; i++) {
      array[i] = Math.floor(Math.random() * 256);
    }
  }
}

export function randomUUID(): string {
  if (typeof window !== 'undefined' && window.crypto && window.crypto.randomUUID) {
    return window.crypto.randomUUID();
  }
  // RFC 4122 v4 compliant UUID generation
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0;
    const v = c === 'x' ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

export function generateToken(length: number = 32): string {
  const array = new Uint8Array(length);
  getRandomValues(array);
  return Array.from(array, (byte) => byte.toString(16).padStart(2, '0')).join('');
}

export function generateBase64Token(length: number = 32): string {
  const array = new Uint8Array(length);
  getRandomValues(array);
  return btoa(String.fromCharCode(...array)).replace(/\+/g, '-').replace(/\//g, '_').replace(/=/g, '');
}