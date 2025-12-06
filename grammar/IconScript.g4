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

script : expression* ;
position : relative='+'? x=FLOAT ',' y=FLOAT ;
expression : assignment | icon ;
assignment : left=IDENTIFIER '=' right=commands ;
scope : '{' commands '}' ;
commands : ( command | scope )+ ;
icon : 'icon' name '=' '{' commands '}' | 'icon' name '=' command ;
command
    : name 
    | VARIABLE
    | 'r'
    | arc
    | circle
    | line
    | rectangle
    | setPosition
    | setWidth ;

// Figures.
arc : 'a' position FLOAT FLOAT FLOAT ;
circle : 'c' position FLOAT ;
line : ('l' | 'lf') position+ ;
rectangle : 's' position position ;

/** Icon name. */
name : IDENTIFIER ;

// Set context.
setPosition : 'p' position ;
setWidth : 'w' FLOAT ;
