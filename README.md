# mygrep

A simple grep command line tool! 

Functions by first converting the regex into RPN using a shunting yard parser. Then uses a simple stack based method to construct the NFA from the postfix regular expression. Finally, builds the DFA from the NFA using the powerset construction algorithm.

Example Input: (poem.txt)

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```


```cargo run -- "(you)|(us)" poem.txt```

Example Output:

```
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
To tell your name the livelong day
```

Improvements:

1. The binary operator '-' for expanding over a range could be implemented as such:
   - Give it a higher precedence than the others in the initial parser so it immediately grabs the left and right operands.
   - Change the labels in the automata to have an upper and lower bound so as to save having to add a label for each character over the range.
2. The square brackets ']' and '[' just alternate on everything inside instead of concatenate.
   - This could be done by keeping a brackets stack in the formatting step and changing the already inserted concat operator to an alternation when inside square brackets.
