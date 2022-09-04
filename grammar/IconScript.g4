/**
 * Grammar for iconscript language.
 *
 * @author Sergey Vartanov
 * @since 2 September 2022
 */
grammar IconScript;

VARIABLE : '@' IDENTIFIER ;
FLOAT : '-'? [0-9]+ ( '.' [0-9]* )? ;
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
    | arc
    | 'c' position FLOAT
    | line
    | 'r'
    | rectangle
    | setPosition
    | 'w' FLOAT ;

// Commands.
arc : 'ar' position FLOAT FLOAT FLOAT ;
line : ('l' | 'lf') position+ ;
name : '%' IDENTIFIER ;
rectangle : 's' position position ;
setPosition : 'p' position ;