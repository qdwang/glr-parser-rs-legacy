program = file_input

single_input = NEWLINE
 | simple_stmt
 | compound_stmt NEWLINE

file_input = (NEWLINE | stmt)+

eval_input = testlist NEWLINE*

decorator = '@' dotted_name ( '(' arglist? ')' )? NEWLINE

decorators = decorator+

decorated = decorators ( classdef | funcdef )

funcdef = DEF NAME parameters ( '->' test )? ':' suite

parameters = '(' typedargslist? ')'

typedargslist = tfpdef ( '=' test )? ( ',' tfpdef ( '=' test )? )* ( ',' ( '*' tfpdef? ( ',' tfpdef ( '=' test )? )* ( ',' '**' tfpdef )?  | '**' tfpdef  )? )?
 | '*' tfpdef? ( ',' tfpdef ( '=' test )? )* ( ',' '**' tfpdef )? 
 | '**' tfpdef

tfpdef = NAME ( ':' test )?

varargslist = vfpdef ( '=' test )? ( ',' vfpdef ( '=' test )? )* ( ',' ( '*' vfpdef? ( ',' vfpdef ( '=' test )? )* ( ',' '**' vfpdef )? | '**' vfpdef )? )?
 | '*' vfpdef? ( ',' vfpdef ( '=' test )? )* ( ',' '**' vfpdef )?
 | '**' vfpdef

vfpdef = NAME

stmt = simple_stmt 
 | compound_stmt

simple_stmt = small_stmt ( ';' small_stmt )* ';'? NEWLINE

small_stmt = expr_stmt 
 | del_stmt 
 | pass_stmt 
 | flow_stmt 
 | import_stmt 
 | global_stmt 
 | nonlocal_stmt 
 | assert_stmt

expr_stmt = testlist_star_expr ( augassign ( yield_expr | testlist) | ( '=' ( yield_expr| testlist_star_expr ) )*)           

testlist_star_expr = ( test | star_expr ) ( ',' ( test |  star_expr ) )* ','?

augassign = '+=' 
 | '-=' 
 | '*=' 
 | '@='
 | '/=' 
 | '%=' 
 | '&=' 
 | '|=' 
 | '^=' 
 | '<<=' 
 | '>>=' 
 | '**=' 
 | '//='

del_stmt = DEL exprlist

pass_stmt = PASS

flow_stmt = break_stmt 
 | continue_stmt 
 | return_stmt 
 | raise_stmt 
 | yield_stmt

break_stmt = BREAK

continue_stmt = CONTINUE

return_stmt = RETURN testlist?

yield_stmt = yield_expr

raise_stmt = RAISE ( test ( FROM test )? )?

import_stmt = import_name 
 | import_from

import_name = IMPORT dotted_as_names

import_from = FROM ( ( '.' | '...' )* dotted_name | ('.' | '...')+ ) IMPORT ( '*' | '(' import_as_names ')' | import_as_names)         

import_as_name = NAME ( AS NAME )?

dotted_as_name = dotted_name ( AS NAME )?

import_as_names = import_as_name ( ',' import_as_name )* ','?

dotted_as_names = dotted_as_name ( ',' dotted_as_name )*

dotted_name = NAME ( '.' NAME )*

global_stmt = GLOBAL NAME ( ',' NAME )*

nonlocal_stmt = NONLOCAL NAME ( ',' NAME )*

assert_stmt = ASSERT test ( ',' test )?

compound_stmt = if_stmt 
 | while_stmt 
 | for_stmt 
 | try_stmt 
 | with_stmt 
 | funcdef 
 | classdef 
 | decorated

if_stmt = IF test ':' suite ( ELIF test ':' suite )* ( ELSE ':' suite )?

while_stmt = WHILE test ':' suite ( ELSE ':' suite )?

for_stmt = FOR exprlist IN testlist ':' suite ( ELSE ':' suite )?

try_stmt = TRY ':' suite ( ( except_clause ':' suite )+ ( ELSE ':' suite )? ( FINALLY ':' suite )? | FINALLY ':' suite )

with_stmt = WITH with_item ( ',' with_item )* ':' suite

with_item = test ( AS expr )?

except_clause = EXCEPT ( test ( AS NAME )? )?

suite = simple_stmt 
 | NEWLINE 'INDENT' stmt+ 'DEDENT'

test = or_test ( IF or_test ELSE test )?
 | lambdef

test_nocond = or_test 
 | lambdef_nocond

lambdef = LAMBDA varargslist? ':' test

lambdef_nocond = LAMBDA varargslist? ':' test_nocond

or_test = and_test ( OR and_test )*

and_test = not_test ( AND not_test )*

not_test = NOT not_test 
 | comparison

comparison = star_expr ( comp_op star_expr )*

comp_op = '<'
 | '>'
 | '=='
 | '>='
 | '<='
 | '<>'
 | '!='
 | IN
 | NOT IN
 | IS
 | IS NOT

star_expr = '*'? expr

expr = xor_expr ( '|' xor_expr )*

xor_expr = and_expr ( '^' and_expr )*

and_expr = shift_expr ( '&' shift_expr )*

shift_expr = arith_expr ( '<<' arith_expr | '>>' arith_expr )*

arith_expr = term ( '+' term | '-' term )*

term = factor ( '*' factor | '/' factor | '%' factor | '//' factor | '@' factor)*

factor = '+' factor 
 | '-' factor 
 | '~' factor 
 | power

power = atom trailer* ( '**' factor )?

atom = '(' ( yield_expr | testlist_comp )? ')' 
 | '[' testlist_comp? ']'  
 | '{' dictorsetmaker? '}' 
 | NAME 
 | number 
 | string+ 
 | '...' 
 | NONE
 | TRUE
 | FALSE

testlist_comp = test ( comp_for | ( ',' test )* ','? )

trailer = '(' arglist? ')' 
 | '[' subscriptlist ']' 
 | '.' NAME

subscriptlist = subscript ( ',' subscript )* ','?

subscript = test 
 | test? ':' test? sliceop?

sliceop = ':' test?

exprlist = star_expr ( ',' star_expr )* ','?

testlist = test ( ',' test )* ','?

dictorsetmaker = test ':' test ( comp_for | ( ',' test ':' test )* ','? ) | test ( comp_for | ( ',' test )* ','? )

classdef = CLASS NAME ( '(' arglist? ')' )? ':' suite

arglist = ( argument ',' )* ( argument ','? | '*' test ( ',' argument )* ( ',' '**' test )? | '**' test )

argument = test comp_for? 
 | test '=' test

comp_iter = comp_for 
 | comp_if

comp_for = FOR exprlist IN or_test comp_iter?

comp_if = IF test_nocond comp_iter?

yield_expr = YIELD yield_arg?

yield_arg = FROM test 
 | testlist

string = STRING_LITERAL
 | BYTES_LITERAL

number = integer
 | FLOAT_NUMBER
 | IMAG_NUMBER

integer = DECIMAL_INTEGER
 | OCT_INTEGER
 | HEX_INTEGER
 | BIN_INTEGER


DEF = 'def'
RETURN = 'return'
RAISE = 'raise'
FROM = 'from'
IMPORT = 'import'
AS = 'as'
GLOBAL = 'global'
NONLOCAL = 'nonlocal'
ASSERT = 'assert'
IF = 'if'
ELIF = 'elif'
ELSE = 'else'
WHILE = 'while'
IN = 'in'
TRY = 'try'
FINALLY = 'finally'
WITH = 'with'
EXCEPT = 'except'
LAMBDA = 'lambda'
OR = 'or'
AND = 'and'
NOT = 'not'
IS = 'is'
NONE = 'None'
TRUE = 'True'
FALSE = 'False'
CLASS = 'class'
YIELD = 'yield'
DEL = 'del'
PASS = 'pass'
CONTINUE = 'continue'
BREAK = 'break'

STRING_LITERAL = ('u'|'U')? ('r'|'R')? ( SHORT_STRING | LONG_STRING )

BYTES_LITERAL = ('b'|'B') ('r'|'R')? ( SHORT_BYTES | LONG_BYTES )

FLOAT_NUMBER = POINT_FLOAT
 | EXPONENT_FLOAT

IMAG_NUMBER = ( FLOAT_NUMBER | INT_PART ) ('j'|'J')

DOT = '.'
ELLIPSIS = '...'
STAR = '*'
OPEN_PAREN = '(' 
CLOSE_PAREN = ')' 
COMMA = ','
COLON = '='
SEMI_COLON = ';'
POWER = '**'
ASSIGN = '='
OPEN_BRACK = '[' 
CLOSE_BRACK = ']' 
OR_OP = '|'
XOR = '^'
AND_OP = '&'
LEFT_SHIFT = '<<'
RIGHT_SHIFT = '>>'
ADD = '+'
MINUS = '-'
DIV = '/'
MOD = '%'
IDIV = '//'
NOT_OP = '~'
OPEN_BRACE = '{' 
CLOSE_BRACE = '}' 
LESS_THAN = '<'
GREATER_THAN = '>'
EQUALS = '=='
GT_EQ = '>='
LT_EQ = '<='
NOT_EQ_1 = '<>'
NOT_EQ_2 = '!='
AT = '@'
ARROW = '->'
ADD_ASSIGN = '+='
SUB_ASSIGN = '-='
MULT_ASSIGN = '*='
AT_ASSIGN = '@='
DIV_ASSIGN = '/='
MOD_ASSIGN = '%='
AND_ASSIGN = '&='
OR_ASSIGN = '|='
XOR_ASSIGN = '^='
LEFT_SHIFT_ASSIGN = '<<='
RIGHT_SHIFT_ASSIGN = '>>='
POWER_ASSIGN = '**='
IDIV_ASSIGN = '//='

EXPONENT_FLOAT = ( INT_PART | POINT_FLOAT ) EXPONENT

SHORT_BYTES = 'SHORT_BYTES Literal'
    
LONG_BYTES = 'LONG_BYTES Literal'
