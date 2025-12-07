" Vim syntax file for iconscript
" Language: iconscript

if exists("b:current_syntax")
  finish
endif

syn case match

syn match iconscriptComment "^#.*$"
syn keyword iconscriptKeyword icon
syn keyword iconscriptCommand l lf c s a p w r
syn match iconscriptVariable "@[a-zA-Z_][a-zA-Z0-9_]*"
syn match iconscriptNumber "-\?\d\+\(\.\d*\)\?"
syn match iconscriptPosition "\+\?\d\+\(\.\d*\)\?,\s*\d\+\(\.\d*\)\?"
syn match iconscriptOperator "[={}]"

syn match iconscriptIdentifier "\<[a-zA-Z_][a-zA-Z0-9_]*\>"
syn region iconscriptString start='"' end='"' contained

hi def link iconscriptKeyword Keyword
hi def link iconscriptCommand Statement
hi def link iconscriptVariable Identifier
hi def link iconscriptIdentifier Identifier
hi def link iconscriptNumber Number
hi def link iconscriptPosition Number
hi def link iconscriptOperator Operator
hi def link iconscriptComment Comment
hi def link iconscriptString String

let b:current_syntax = "iconscript"
