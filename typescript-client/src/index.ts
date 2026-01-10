/**
 * Axiom TypeScript/JavaScript Client
 *
 * Official client library for Axiom - semantic knowledge system.
 *
 * @packageDocumentation
 */

export { AxiomClient, type ClientConfig } from './client';

// Models
export type {
  Token,
  TokenCreate,
  TokenUpdate,
  QueryResult,
  APIKey,
  APIKeyCreate,
  User,
  HealthCheck,
  SystemStatus,
  JWTToken,
  ErrorResponse,
} from './models';

// Errors
export {
  AxiomError,
  AuthenticationError,
  AuthorizationError,
  NotFoundError,
  ValidationError,
  RateLimitError,
  ConflictError,
  ServerError,
  NetworkError,
} from './errors';

// Retry utilities
export {
  retryWithBackoff,
  withRetry,
  calculateDelay,
  shouldRetry,
  type RetryConfig,
} from './utils/retry';

// Version
export const VERSION = '0.59.2';
