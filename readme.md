incredible how I don't know how to properly write rust
**I will probably rewrite this entire thing 5 or 6 times because I'm learning rust as I'm learning how to write a program like this**
and yes I'm also learning regex while making this


currently supported operations<br>

{ and } create and close capture groups ( { -> "(?:", } -> ")" )
[digit-digit] defines a range of capture DOES NOT work for alphabetic characters atm
[digit] defines a count for the following capture (e.x. [3] "hello" -> (hello){3})
"[word]" captures a word


IN PROGRESS:
[letter-letter] (e.x. [a-z] / [a-zA-z] / etc.)
special operators - <word> -> \w, <digit> -> \d, <space> -> \s, etc.
or/and translating into logical ors and ands
negative lookaheads (possibly denoted by <neglook>?)  
