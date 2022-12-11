A calculator parser/interpreter made in rust.

Can be executed through a terminal with the following arguments:

- help
    - Description: Display application help.
    - Usage: /help

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

<br />
	
    > ./calculator_parser "/exp: 5 + 2 * 3 ^ 2 - 1" /action:parse
    Parsed: 5 + [2 * [3 ^ 2]] - 1

<br />
	
    > ./calculator_parser "/exp: -2 * 4! + max(rrand(0, 25), 2)" /action:both
    Parsed: [[-2] * [4!]] + max(rrand(0, 25), 2)
    -33.538227
 
### TODO:

  - Report error reasons during parsing.
    - This is surprisingly difficult, because the way I am handling my parsing makes it difficult to determine where the error in the expression actually is.
    - Add *sync* tokens? i.e. a specific token which identifies during a specific step that the input is *supposed* to match this step, so return an actual error instead of trying next possibility in parent routine.
  - Implement GUI?