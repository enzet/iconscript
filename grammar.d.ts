/**
 * Type declarations for ANTLR-generated grammar files.
 */

import type {CharStream} from "antlr4";

declare module "./grammar/IconScriptLexer.js" {
    export default class IconScriptLexer {
        constructor(input: CharStream);
    }
}

import type {TokenStream, ErrorListener} from "antlr4";
import type {ScriptContext} from "./grammar/IconScriptParser.js";

declare module "./grammar/IconScriptParser.js" {
    export default class IconScriptParser {
        constructor(stream: TokenStream);
        script(): ScriptContext;
        removeErrorListeners(): void;
        addErrorListener(listener: ErrorListener): void;
    }
}
