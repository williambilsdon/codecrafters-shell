This is my approach to the
["Build Your Own Shell" Challenge](https://app.codecrafters.io/courses/shell/overview).

I've stuck to using only the std crate in order to give myself a chance to learn a little more about it and explore what abilities rust has built in.

## Areas to work on

- Implement Result<> across the board and handle consistently.
- Gracefully handling all errors.
- Input validation.
- Tab completion could be interesting.
- Improve how external command works/how an invalid command is detected.
  (Validate command exists in creation and return an Err if it can't be found).
- Update success paths to make each command module testable.
