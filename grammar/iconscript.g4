grammar iconscript;
script : expression* ;
expression : assignment | icon ;
assignment : IDENTIFIER '=' command+ ;
icon : '{' command* '}' ;
command : 
    ( '{' 
    | '}' 
    | 'a'
    | 'ar' POSITION FLOAT FLOAT FLOAT 
    | 'c' POSITION FLOAT
    | 'l' POSITION+
    | 'lf' POSITION+ 
    | 'r' 
    | 'p' FLOAT
    | 's' POSITION POSITION
    | 'w' FLOAT ) ;
INT : [0-9]+ ;
POSITION : '+'? FLOAT ',' FLOAT ;
FLOAT : '-'? INT+ ( '.' INT* )? ;
IDENTIFIER : [a-zA-Z_][a-zA-Z0-9_]* ;
WS : (' ' | '\t' | '\r'| '\n') -> skip ;