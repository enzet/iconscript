/**
 * Grammar for iconscript language.
 *
 * @author Sergey Vartanov
 * @since 2 September 2022
 */
grammar IconScript;

VARIABLE : '@' IDENTIFIER ;
FLOAT : '-'? [0-9]+ ( '.' * )? ;
IDENTIFIER : [a-zA-Z_][a-zA-Z0-9_]* ;
WS : [ \t\r\n] -> skip ;

position : relative='+'? x=FLOAT ',' y=FLOAT ;
script : expression* ;
expression : assignment | icon ;
assignment : left=IDENTIFIER '=' right=commands ;
commands : command+ ;
icon : '{' command* '}' ;
command
    : name 
    | VARIABLE
    | 'a'
    | 'ar' position FLOAT FLOAT FLOAT
    | 'c' position FLOAT
    | line
    | 'r'
    | 'p' FLOAT
    | 's' position position
    | 'w' FLOAT ;

// Commands.
line : ('l' | 'lf') position+ ;
name : '%' IDENTIFIER ;