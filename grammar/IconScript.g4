grammar IconScript;

VARIABLE : '@' IDENTIFIER ;
POSITION : '+'? FLOAT ',' FLOAT ;
FLOAT : '-'? [0-9]+ ( '.' * )? ;
IDENTIFIER : [a-zA-Z_][a-zA-Z0-9_]* ;
WS : (' ' | '\t' | '\r'| '\n') -> skip ;

script : expression* ;
expression : assignment | icon ;
assignment : left=IDENTIFIER '=' right=commands ;
commands : command+ ;
icon : '{' command* '}' ;
command :
    ( '%' IDENTIFIER
    | VARIABLE
    | 'a'
    | 'ar' POSITION FLOAT FLOAT FLOAT
    | 'c' POSITION FLOAT
    | 'l' POSITION+
    | 'lf' POSITION+
    | 'r'
    | 'p' FLOAT
    | 's' POSITION POSITION
    | 'w' FLOAT ) ;