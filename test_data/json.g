Goal = json
json =  object
    |   array

object = '{' pair (',' pair)* '}'
    |   '{' '}' 
  
    
pair =  STRING ':' value 

array =  '[' value (',' value)* ']'
    |   '[' ']' 
   

value = STRING
    |   NUMBER
    |   object  
    |   array  
    |   'true' 
    |   'false'
    |   'null'

NUMBER = '-'? INT '.' NUM EXP?
    |   '-'? INT EXP
    |   '-'? INT

EXP =  ('E'|'e') ('+'|'-')? INT 
INT =  NUM