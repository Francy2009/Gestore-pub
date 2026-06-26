// Bridge verso i comandi Rust di Tauri (modalità desktop).
//
// In Tauri v2, con `app.withGlobalTauri: false` in tauri.conf.json, l'oggetto
// `window.__TAURI__` NON viene esposto al frontend: l'unico marker sempre
// presente nel webview è `window.__TAURI_INTERNALS__` (il bridge IPC interno).
// Per chiamare i comandi Rust si deve quindi usare `invoke` dal pacchetto
// `@tauri-apps/api/core`, che a sua volta delega a `__TAURI_INTERNALS__.invoke`.
//
// Fuori dal webview Tauri (dev browser su localhost:3000, SSR, test senza mock)
// `__TAURI_INTERNALS__` non esiste: `getTauriInvoke()` ritorna null e i
// chiamanti ricadono sul fallback web (localStorage / Blob download).

export type TauriInvoke = <T>(
  command: string,
  args?: Record<string, unknown>,
) => Promise<T>

/**
 * True se stiamo girando dentro il webview Tauri desktop.
 * Sync, sicura in SSR (guarda `window`).
 */
export function isTauriWebview(): boolean {
  return (
    typeof window !== 'undefined' &&
    '__TAURI_INTERNALS__' in window &&
    typeof (window as typeof window & {
      __TAURI_INTERNALS__?: { invoke?: unknown };
    }).__TAURI_INTERNALS__?.invoke === 'function'
  )
}

// Cache del dynamic import: carichiamo `@tauri-apps/api/core` una sola volta e
// solo quando serviamo davvero invoke (mai in SSR/dev browser, dove il modulo
// non verrebbe mai richiesto). Questo mantiene il bundle server pulito.
let invokeLoader: Promise<TauriInvoke> | null = null

function loadTauriInvoke(): Promise<TauriInvoke> {
  if (!invokeLoader) {
    invokeLoader = import('@tauri-apps/api/core').then(
      (mod) => mod.invoke as unknown as TauriInvoke,
    )
  }
  return invokeLoader
}

/**
 * Ritorna la `invoke` di Tauri quando siamo nel webview desktop, altrimenti
 * null. I chiamanti devono fare `await getTauriInvoke()` e gestire il fallback
 * quando ritorna null.
 */
export async function getTauriInvoke(): Promise<TauriInvoke | null> {
  if (!isTauriWebview()) return null
  try {
    return await loadTauriInvoke()
  } catch {
    return null
  }
}