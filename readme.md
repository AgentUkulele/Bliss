incredible how I don't know how to properly write rust <br>
**I will probably rewrite this entire thing 5 or 6 times because I'm learning rust as I'm learning how to write a program like this** <br>
and yes I'm also learning regex while making this <br>


currently supported operations<br>
<br>
{ and } create and close capture groups ( { -> "(?:", } -> ")" ) <br>
[digit-digit] defines a range of capture DOES NOT work for alphabetic characters atm <br>
[digit] defines a count for the following capture (e.x. [3] "hello" -> (hello){3}) <br>
"[word]" captures a word <br>


IN PROGRESS: <br>
<ul>
<li>[letter-letter] (e.x. [a-z] / [a-zA-z] / etc.) </li>
<li>special operators - <word> -> \w, <digit> -> \d, <space> -> \s, etc. </li>
<li>or/and translating into logical ors and ands </li>
<li>negative lookaheads (possibly denoted by \<neglook>?)</li>  
</ul>
