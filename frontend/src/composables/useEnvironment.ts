import type { EnvVarKey } from '../../env'

/**
 * Represents the expected environment configuration.
 */
type EnvironmentConfig = {
  /**
   * The base connection URL of the @liftoff/api.
   */
  apiUrl: string
}

/**
 * Parses a required environment variable value from the provided environment variables for the provided key.
 * @param key - The key of the required environment variable.
 * @throws {Error} - When the provided key doesn't exist in the provided environment variables.
 */
function parseRequiredEnvConfigEntry<T>(key: EnvVarKey): T {
  if (!import.meta.env[key]) {
    throw new Error(
      `Failed to find an environment configuration key for the required entry key '${key}'!`
    )
  }

  return import.meta.env[key]
}

/**
 * Parses the expected environment variables.
 */
export default function useEnv(): EnvironmentConfig {
  return {
    apiUrl: parseRequiredEnvConfigEntry('VITE_BASE_API_URL')
  }
}
