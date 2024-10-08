# Vera

#### An interpreted esoteric programming language with syntax based on Pink Floyd song lyrics and titles

> [!WARNING]
> The language is still under development and will probably undergo several changes before release. Keywords are most likely to change in the future

## Language Overview

Vera is a dynamically typed interpreted Object Oriented programming language. Each statement has to end with a semicolone ( ; ). Vera does not allow nested procedures

### Comments

Comments in Vera start with the double forward slash ( // ).

```c
// this is a comment
```

### Declarations

Variables in Vera are declared using the **pink** keyword.

```ruby 
pink x = 5;
```

Variables can also take the null value. The root of all evil, null, in xanadu is called **money**

```ruby 
pink empty = money;
```

Procedures are declared using the **brick** keyword

```ruby 
brick add(a, b) {
    goodbye a + b;
}
```

The **goodbye** keyword in the above example functions just as a return keyword in other languages

The procedure can later be called like this

```ruby 
pink x = add(1, 2); // Assigns value of 3 to variable x
```

### Writting to the console

To write to the console use the **shine** keyword

```ruby 
shine "Hey you";
```

### Control flow

if statements are declared using the **outThere?** keyword

```ruby 
outThere?(1 == 1) {
    shine "Hey you";
}
```

else statements are declared using the **anybody** keyword

```ruby 
outThere?(1 == 2) {
    shine "Hey you";
} anybody {
    shine "Not you";
}
```

else if statements are declared using the **anybody outThere?** keyword

```ruby 
outThere?(1 == 2) {
    shine "Hey you";
} anybody {
    shine "??";
} anybody outThere?(1 == 1) {
    shine "Not you";
}
```

while loops are declared using the **echoes** keyword

```ruby 
echoes(1 == 1) {
    shine "Hey you";
}
```

for loops are declared using the **time** keyword

```ruby 
time(pink i = 0; i < 10; i++) {
    shine i;
}
```

### Operators

#### Arithmetic Operators 

 | Symbol   | Operator  | Syntax |
 | :---:    |  :---:    | :---:  |
 |   +      |  Plus     | a + b  |
 |   -      |  Minus    | a - b  |
 |   *      | Multiply  | a * b  |
 |   /      |  Divide    | a / b  |
 |   ^      |  POWER    | a ^ b  |
 |   ++      |  increment    | a++  |
 |   -      |  decrement    | a--  |

#### Relational Operators

 | Symbol  |  Operator  | Syntax |
 | :---:   |   :---:    | :---:  |
 |   <     |   Less then    |  a < b  |
 |   >     |   Greater then   |  a > b  |
 |   <=    | Less then or equal to |  a <= b  |
 |   >=    |   Greater then or equal to    | a >= b  |
 |   ==    |   Equal to   |  a == b  |
 |   !=    |   Not equal to    | a != b  |

#### Logical Operators

 | Symbol  |  Operator  | Syntax |
 | :---:   |   :---:    | :---:  |
 |   and      |  Logical AND     | a and b  |
 |   or      |  Logical OR   |  a or b |
 |   !      | Logical NOT |  !a  |

#### Bitwise Operators

 | Symbol  |  Operator  | Syntax |
 | :---:   |   :---:    | :---:  |
 |   <<      |  Left bit shift     | a << b  |
 |   >>      |  Right bit shift     | a >> b  |

#### Assignment Operators

 | Symbol   | Operator |  Syntax |
 | :---:    |  :---:   |  :---:  |
 |   =      |  Simple Assignment  | a = b  |

## Tooling

- Code editor with syntax highlighting: [Heaven's Door](https://github.com/Turtel216/Heavens-Door)
