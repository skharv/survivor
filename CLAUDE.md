# Survivor

## Rules

- Never use `.unwrap()`. Use early returns with `let Ok(...) = ... else { return; }` or propagate errors with `?` instead.
