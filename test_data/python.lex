SHORT_STRING = '(?:\\\.|[^\\\r\n'])*'|"(?:\\\.|[^\\\r\n"])*"

LONG_STRING = '''(?:[^\\]|\\\.)*'''|"""(?:[^\\]|\\\.)*"""

POINT_FLOAT = [0-9]?\.[0-9]+|[0-9]+\.

OCT_INTEGER = 0[oO][0-7]+
HEX_INTEGER = 0[xX][0-9a-fA-F]+
BIN_INTEGER = 0[bB][01]+

FRACTION = \.[0-9]+

EXPONENT = [eE][+-]?[0-9]+

DECIMAL_INTEGER = [1-9][0-9]*

NON_ZERO_DIGIT = [1-9]

INT_PART = [0-9]+

NAME = [a-zA-Z_][a-zA-Z_0-9]*

NEWLINE = \r\n|[\r\n]