// To implement * as a wildcard, I did these steps:
// SELECT * FROM
// 1. Added a new variant `AllColumns` to the Expression enum in statement.rs
//    This lets the parser distinguish * as selecting all columns, not multiplication.

// 2. Modified the column parsing loop in the parse_select() method in sql_parser.rs
//    Before parsing each column, I check if the next token is Star (`*`).
//    If yes, I push Expression::AllColumns to the columns list.

// 3. I already had the tokenizer producing the Star token for `*` (used for multiplication),
//    so no additional tokenizer changes were needed for this.
