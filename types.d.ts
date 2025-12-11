/**
 * Type definitions for IconScript parser.
 */

export interface Icon {
    name: string | null;
    svg: string;
}

declare global {
    const IconScriptParser: {
        parseIconsFile(content: string): Icon[];
    };
}

export {};
