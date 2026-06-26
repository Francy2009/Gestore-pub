import { isTauriWebview } from './tauri-bridge'

declare const __APP_VERSION__: string

const GITHUB_LATEST_RELEASE_API = 'https://api.github.com/repos/Francy2009/The-Club/releases/latest'
// Pagina canonica delle release. Usata come fallback sicuro se la risposta
// dell'API restituisse un html_url inatteso (es. account GitHub compromesso):
// non apriamo mai nel browser di sistema un URL esterno non affidabile.
const GITHUB_RELEASES_PAGE = 'https://github.com/Francy2009/The-Club/releases/latest'
const GITHUB_RELEASE_URL_PREFIX = 'https://github.com/Francy2009/The-Club/'

type GitHubReleaseResponse = {
  tag_name?: unknown
  html_url?: unknown
  body?: unknown
  draft?: unknown
  prerelease?: unknown
}

export type AppUpdateInfo = {
  version: string
  tagName: string
  releaseUrl: string
  notes: string | null
}

export function getCurrentAppVersion() {
  return __APP_VERSION__
}

export function isTauriRuntime() {
  return isTauriWebview()
}

export async function checkForAvailableUpdate(): Promise<AppUpdateInfo | null> {
  let response: Response

  try {
    response = await fetch(GITHUB_LATEST_RELEASE_API, {
      cache: 'no-store',
      headers: {
        Accept: 'application/vnd.github+json',
        'X-GitHub-Api-Version': '2022-11-28',
      },
    })
  } catch (networkError) {
    throw new Error(`Controllo aggiornamenti non riuscito (network): ${networkError}`)
  }

  if (!response.ok) {
    throw new Error(`Controllo aggiornamenti non riuscito: ${response.status}`)
  }

  const release = (await response.json()) as GitHubReleaseResponse
  if (release.draft === true || release.prerelease === true) return null
  if (typeof release.tag_name !== 'string' || typeof release.html_url !== 'string') return null

  const version = normalizeVersion(release.tag_name)
  if (!isVersionNewer(version, getCurrentAppVersion())) return null

  // Whitelist: apriamo solo URL sotto il repo ufficiale. Se html_url non
  // corrisponde (risposta manipolata), usiamo la pagina release canonica.
  const releaseUrl = release.html_url.startsWith(GITHUB_RELEASE_URL_PREFIX)
    ? release.html_url
    : GITHUB_RELEASES_PAGE

  return {
    version,
    tagName: release.tag_name,
    releaseUrl,
    notes: typeof release.body === 'string' && release.body.trim() ? release.body.trim() : null,
  }
}

function normalizeVersion(version: string) {
  return version.trim().replace(/^v/i, '')
}

export function isVersionNewer(candidate: string, current: string) {
  const candidateParts = parseVersion(candidate)
  const currentParts = parseVersion(current)
  if (!candidateParts || !currentParts) return false

  for (let index = 0; index < candidateParts.length; index += 1) {
    const candidatePart = candidateParts[index] ?? 0
    const currentPart = currentParts[index] ?? 0
    if (candidatePart > currentPart) return true
    if (candidatePart < currentPart) return false
  }

  return false
}

function parseVersion(version: string) {
  const match = normalizeVersion(version).match(/^(\d+)\.(\d+)\.(\d+)/)
  if (!match) return null

  return [Number(match[1]), Number(match[2]), Number(match[3])]
}
