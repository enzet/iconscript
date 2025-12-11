/**
 * Type declarations for ANTLR-generated grammar files.
 */

declare module "./grammar/IconScriptLexer.js" {
    export default class IconScriptLexer {
        constructor(input: any);
    }
}

declare module "./grammar/IconScriptParser.js" {
    export default class IconScriptParser {
        constructor(stream: any);
        script(): any;
        removeErrorListeners(): void;
        addErrorListener(listener: any): void;
    }
}

declare module "./grammar/IconScriptListener.js" {
    export default class GeneratedIconScriptListener {
        enterAssignment?(ctx: any): void;
        exitIcon?(ctx: any): void;
        enterName?(ctx: any): void;
        exitLine?(ctx: any): void;
        exitCircle?(ctx: any): void;
        exitArc?(ctx: any): void;
        exitRectangle?(ctx: any): void;
        exitSetPosition?(ctx: any): void;
        exitSetWidth?(ctx: any): void;
        exitSetRemove?(ctx: any): void;
        enterCommand?(ctx: any): void;
        enterIcon?(ctx: any): void;
        enterScope?(ctx: any): void;
        exitScope?(ctx: any): void;
    }
}
