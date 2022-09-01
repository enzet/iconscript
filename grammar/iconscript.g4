grammar iconscript;
script: command* ;
command: ( '{' | '}' | 'a' | 'ar' | 'c' | 'l' | 'lf' | 'r' | 's' | 'w' ) ;
NEWLINE : [\r\n]+ ;
assignment : IDENTIFIER '=' command+ ;
INT : [0-9]+ ;
POSITION : '+'? FLOAT ',' FLOAT ;
FLOAT : '-'? INT+ ( '.' INT* )? ;
IDENTIFIER : [a-zA-Z_][a-zA-Z0-9_]* ;
WS : (' ' | '\t' | '\r'| '\n') -> skip ;

// WS : (' ' | '\t' | '\r'| '\n') {$channel=HIDDEN;} ;
