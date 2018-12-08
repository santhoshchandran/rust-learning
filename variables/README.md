# Differences between variables and constant
1. Constants must be annotated with datatype.
2. Constants can be declared in any scope including global scope, whereas variables(let) have limited scope, at the time of learning we have explored only function scope with variables.


# Shadowing:
Declaring a new variable with the same name as a previous variable, and the new variable shadows the previous variable.

# Difference between shadowing and mut:
1. Shadowing is different than marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.
2. The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name. 
*Example:*
```
let spaces = "   ";
let spaces = spaces.len();
```
	
