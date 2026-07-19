## Hint 3: Algorithm Outline

```
function format_scoreboard_line(player_name, score, rank):
    Step 1: Format rank as a 2-digit, zero-padded number
    Step 2: Format score as a 4-digit, zero-padded number with a leading + or -
    Step 3: Combine rank | name | score into a single string using a literal separator
    Step 4: Return the formatted string
```

**Note:** This is pseudocode. You need to call `format!()` or a similar Rust macro to produce the output. The `format!()` macro is your friend here; learn its syntax and you'll use it everywhere.

**Spoiler threshold:** High—this tells you the steps, but not the Rust syntax.
