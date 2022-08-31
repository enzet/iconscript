grammar iconscript;
script: command* ;
command: ( 'l' | 'lf' ) ;
NEWLINE : [\r\n]+ ;
INT : [0-9]+ ;
POSITION : '+'? FLOAT ',' FLOAT ;
FLOAT : INT+ ( '.' INT* )? ;
