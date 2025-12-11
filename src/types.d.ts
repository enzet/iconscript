/**
 * Type definitions for IconScript parser.
 */

export interface Icon {
    name: string | null;
    svg: string;
}

/**
 * Type for ANTLR parser context objects.
 * ANTLR-generated parser contexts don't have type definitions.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type AntlrContext = any;

declare global {
    const IconScriptParser: {
        parseIconsFile(content: string): Icon[];
    };
}

export {};
