A calculator parser/interpreter made in rust.

Can be executed through the terminal, starting a loop allowing the user to enter an expression, and printing the evaluated result. The loop will exit if the user enters 'exit'.
 
### TODO:

  - ~~Report error reasons during parsing.~~ (Done)
    -  ~~ This is surprisingly difficult, because the way I am handling my parsing makes it difficult to determine where the error in the expression actually is. ~~
    -  ~~ Add *sync* tokens? i.e. a specific token which identifies during a specific step that the input is *supposed* to match this step, so return an actual error instead of trying next possibility in parent routine. ~~
  - Implement GUI?