// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
//
// The necessary import statements have been automatically added by a Python script.
// This ensures that all required dependencies are correctly referenced and available
// within this module.
//
// If you need to add or modify imports, please update the import_map in the script and
// re-run `make gen-models` it to regenerate the file accordingly.

import type { LocalizedString } from "@repo/moss-text";

/**
 * Represents an HTML link (`<a>`) with attributes commonly used in web development.
 */
export type HtmlLink = { href: string; target?: Target; rel?: Rel; text?: LocalizedString };

/**
 * Represents possible values for the `rel` attribute in an HTML link.
 *
 * The `rel` attribute specifies the relationship between the current document
 * and the linked document.
 */
export type Rel = "noopener" | "noreferrer" | "nofollow";

/**
 * Represents possible values for the `target` attribute in an HTML link.
 *
 * The `target` attribute specifies where to open the linked document.
 */
export type Target = "_self" | "_blank" | "_parent" | "_top";
