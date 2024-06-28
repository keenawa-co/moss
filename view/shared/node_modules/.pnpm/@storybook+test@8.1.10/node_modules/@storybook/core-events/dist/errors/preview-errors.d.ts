import { S as StorybookError } from '../storybook-error-32ac504c.js';

/**
 * If you can't find a suitable category for your error, create one
 * based on the package name/file path of which the error is thrown.
 * For instance:
 * If it's from @storybook/client-logger, then CLIENT-LOGGER
 *
 * Categories are prefixed by a logical grouping, e.g. PREVIEW_ or FRAMEWORK_
 * to prevent manager and preview errors from having the same category and error code.
 */
declare enum Category {
    DOCS_TOOLS = "DOCS-TOOLS",
    PREVIEW_CLIENT_LOGGER = "PREVIEW_CLIENT-LOGGER",
    PREVIEW_CHANNELS = "PREVIEW_CHANNELS",
    PREVIEW_CORE_EVENTS = "PREVIEW_CORE-EVENTS",
    PREVIEW_INSTRUMENTER = "PREVIEW_INSTRUMENTER",
    PREVIEW_API = "PREVIEW_API",
    PREVIEW_REACT_DOM_SHIM = "PREVIEW_REACT-DOM-SHIM",
    PREVIEW_ROUTER = "PREVIEW_ROUTER",
    PREVIEW_THEMING = "PREVIEW_THEMING",
    RENDERER_HTML = "RENDERER_HTML",
    RENDERER_PREACT = "RENDERER_PREACT",
    RENDERER_REACT = "RENDERER_REACT",
    RENDERER_SERVER = "RENDERER_SERVER",
    RENDERER_SVELTE = "RENDERER_SVELTE",
    RENDERER_VUE = "RENDERER_VUE",
    RENDERER_VUE3 = "RENDERER_VUE3",
    RENDERER_WEB_COMPONENTS = "RENDERER_WEB-COMPONENTS",
    FRAMEWORK_NEXTJS = "FRAMEWORK_NEXTJS"
}
declare class MissingStoryAfterHmrError extends StorybookError {
    data: {
        storyId: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 1;
    constructor(data: {
        storyId: string;
    });
    template(): string;
}
declare class ImplicitActionsDuringRendering extends StorybookError {
    data: {
        phase: string;
        name: string;
        deprecated: boolean;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 2;
    readonly documentation = "https://github.com/storybookjs/storybook/blob/next/MIGRATION.md#using-implicit-actions-during-rendering-is-deprecated-for-example-in-the-play-function";
    constructor(data: {
        phase: string;
        name: string;
        deprecated: boolean;
    });
    template(): string;
}
declare class CalledExtractOnStoreError extends StorybookError {
    readonly category = Category.PREVIEW_API;
    readonly code = 3;
    template(): string;
}
declare class MissingRenderToCanvasError extends StorybookError {
    readonly category = Category.PREVIEW_API;
    readonly code = 4;
    readonly documentation = "https://github.com/storybookjs/storybook/blob/next/MIGRATION.md#mainjs-framework-field";
    template(): string;
}
declare class CalledPreviewMethodBeforeInitializationError extends StorybookError {
    data: {
        methodName: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 5;
    constructor(data: {
        methodName: string;
    });
    template(): string;
}
declare class StoryIndexFetchError extends StorybookError {
    data: {
        text: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 6;
    constructor(data: {
        text: string;
    });
    template(): string;
}
declare class MdxFileWithNoCsfReferencesError extends StorybookError {
    data: {
        storyId: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 7;
    constructor(data: {
        storyId: string;
    });
    template(): string;
}
declare class EmptyIndexError extends StorybookError {
    readonly category = Category.PREVIEW_API;
    readonly code = 8;
    template(): string;
}
declare class NoStoryMatchError extends StorybookError {
    data: {
        storySpecifier: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 9;
    constructor(data: {
        storySpecifier: string;
    });
    template(): string;
}
declare class MissingStoryFromCsfFileError extends StorybookError {
    data: {
        storyId: string;
    };
    readonly category = Category.PREVIEW_API;
    readonly code = 10;
    constructor(data: {
        storyId: string;
    });
    template(): string;
}
declare class StoryStoreAccessedBeforeInitializationError extends StorybookError {
    readonly category = Category.PREVIEW_API;
    readonly code = 11;
    template(): string;
}
declare class NextJsSharpError extends StorybookError {
    readonly category = Category.FRAMEWORK_NEXTJS;
    readonly code = 1;
    readonly documentation = "https://storybook.js.org/docs/get-started/nextjs#faq";
    template(): string;
}
declare class NextjsRouterMocksNotAvailable extends StorybookError {
    data: {
        importType: string;
    };
    readonly category = Category.FRAMEWORK_NEXTJS;
    readonly code = 2;
    constructor(data: {
        importType: string;
    });
    template(): string;
}
declare class UnknownArgTypesError extends StorybookError {
    data: {
        type: object;
        language: string;
    };
    readonly category = Category.DOCS_TOOLS;
    readonly code = 1;
    readonly documentation = "https://github.com/storybookjs/storybook/issues/26606";
    constructor(data: {
        type: object;
        language: string;
    });
    template(): string;
}

export { CalledExtractOnStoreError, CalledPreviewMethodBeforeInitializationError, Category, EmptyIndexError, ImplicitActionsDuringRendering, MdxFileWithNoCsfReferencesError, MissingRenderToCanvasError, MissingStoryAfterHmrError, MissingStoryFromCsfFileError, NextJsSharpError, NextjsRouterMocksNotAvailable, NoStoryMatchError, StoryIndexFetchError, StoryStoreAccessedBeforeInitializationError, UnknownArgTypesError };
