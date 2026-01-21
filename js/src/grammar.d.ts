/**
 * Type declarations for ANTLR-generated grammar files.
 */

import type {CharStream, TokenStream, ErrorListener} from "antlr4";

// Prevent TypeScript from trying to compile the actual grammar files.
declare module "../grammar/IconScriptLexer.js" {
    export default class IconScriptLexer {
        constructor(input: CharStream);
    }
}

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

declare module "../grammar/IconScriptListener.js" {
    export default class IconScriptListener {
        // Type placeholder for the generated listener.
    }
}
