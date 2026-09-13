/**
 * The life span of an action's cache, in milliseconds.
 */
const CACHE_LIFE_SPAN_MS: number = 60 * 1000

/**
 * Represents the state of a store fetch action.
 */
export class FetchActionState {
  loading: boolean
  error?: unknown
  lastRunAt?: Date

  constructor() {
    this.loading = false
  }

  private get cacheExpiresAt(): Date | undefined {
    if (!this.lastRunAt) {
      return undefined
    }

    return new Date(this.lastRunAt.getTime() + CACHE_LIFE_SPAN_MS)
  }

  private isFresh(): boolean {
    if (!this.cacheExpiresAt) {
      return false
    }

    return Date.now() < this.cacheExpiresAt.getTime()
  }

  isStale(): boolean {
    return !this.isFresh()
  }
}
