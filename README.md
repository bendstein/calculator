A calculator parser/interpreter made in rust.

Can be executed through a terminal with the following arguments:

- help
    - Description: Display application help.
    - Usage: /help
    - Example: If used as a key-value argument, rather than a flag argument, must be either true or false.

- exp
    - Description: The expression to perform the action on.
    - Usage: /exp:{VALUE}

- action
    - Description: The action to perform on the expression. (parse: Parse the expression, and print the formatted parse tree; evaluate: Evaluate the expression and print the result; both: Parse the expression, print the formatted parse tree, evaluate the expression, and print the result.)
    - Usage: /action:{VALUE}
    - Restrictions: [parse, evaluate, both]
    - Default Value: evaluate

Examples:

    > ./calculator_parser "/exp: (6 * 7 + 2)^4"
    3748096
 
    > ./calculator_parser "/exp: 5 + 2 * 3 ^ 2 - 1" /action:parse
    Parsed: 5 + [2 * [3 ^ 2]] - 1

    > ./calculator_parser "/exp: -2 * 4! + max(rrand(0, 25), 2)" /action:both
    Parsed: [[-2] * [4!]] + max(rrand(0, 25), 2)
    -33.538227
 
