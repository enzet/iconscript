/**
 * Type declarations for ANTLR-generated grammar files.
 */

import type {CharStream, TokenStream, ErrorListener} from "antlr4";

// Prevent TypeScript from trying to compile the actual grammar files.
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error - Grammar files are generated and not part of the TypeScript project.
declare module "../grammar/IconScriptLexer.js" {
    export default class IconScriptLexer {
        constructor(input: CharStream);
    }
}

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error - Grammar files are generated and not part of the TypeScript project.
declare module "../grammar/IconScriptParser.js" {
    // eslint-disable-next-line @typescript-eslint/no-empty-object-type
    export interface ScriptContext {
        // Type placeholder for the actual context type.
    }

    export default class IconScriptParser {
        constructor(stream: TokenStream);
        script(): ScriptContext;
        removeErrorListeners(): void;
        addErrorListener(listener: ErrorListener): void;
    }
}

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-expect-error - Grammar files are generated and not part of the TypeScript project.
declare module "../grammar/IconScriptListener.js" {
    export default class IconScriptListener {
        // Type placeholder for the generated listener.
    }
}
