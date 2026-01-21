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
COMMENT : '#' ~[\r\n]* -> skip ;
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
    | arc
    | circle
    | line
    | rectangle
    | setPosition
    | setRemove
    | setWidth
    | setFill ;

// Figures.
arc : 'a' position FLOAT FLOAT FLOAT ;
circle : 'e' position FLOAT ;
line : ('l' | 'lf') position+ ;
rectangle : 'r' position position ;

// Icon name.
name : IDENTIFIER ;

// Set context.
setPosition : 'm' position ;
setWidth : 'w' FLOAT ;
setRemove : 'subtract' ;
setFill : 'fill' ;
