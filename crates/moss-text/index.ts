/**
 * Represents a localized string as a tuple.
 *
 * - `key`: The unique localization key.
 * - `origin`: The original or fallback text.
 * - `description`: Optional description providing additional context or `null` if absent.
 *
 * @example
 * ["greeting.hello", "Hello, World!", "A friendly greeting"]
 * ["greeting.hello", "Hello, World!", null]
 */
export type LocalizedString = [key: string, origin: string, description: string | null];
