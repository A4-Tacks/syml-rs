SYML is a configuration language similar to YAML, but may be more user-friendly

# Syntax (Imprecise)

- **_** (white space):\
  ` ` / `\t`
- **hex**:\
  \[0-9A-Fa-f]
- **nl**:\
  `\r`? `\n`\
  / eof
- **comment**:\
  `;` (!nl any-char)\*
- **cnl**:\
  (_ comment? `\r`? `\n`)\+\
  / eof
- **escape**:\
  `\` (
    \[\\\\nrt"' \t]\
    / `x` hex{2}\
    / `u` (hex{4} / `{` hex+ `}`)\
    / `U` hex{8}\
  )
- **str_ignore**:\
  `\` (_ comment)? `\r`? `\n` _
- **simple-val**:\
  ([!#$%&()\*+./0-9<=>?@A-Z\\\\^\_\`a-z|\~\p{XID\_Start}] / `-` !` `)\
  [!#$%&()\*+./0-9<=>?@A-Z\\\\^\_\`a-z|\~\\\-'\p{XID\_Continue}]\*\
  / `'` \[^'\r\n]\* `'`\
  / `"` str\_ignore\* ((escape / \[^\\r\\n\\\\]) str\_ignore\*)\* `"`
- **inline-value**:\
  `[` _ (simple-val (_ `,` _ simple-val)* _ `,`?)? _ `]`\
  / `{` _ (simple-val _ `:` _ inline-value (_ `,` simple-val _ `:` _ inline-value)* _ `,`?)? _ `}`\
  / simple-val
- **list**:\
  `- ` ivalue `\n` (cnl indent(+2) `- ` ivalue(+2))\*
- **ivalue**:\
  list\
  / simple-val _ `:` (_ simple-val _ `:`)\* (\
    cnl() (indent list(+0) / indent(+2) ivalue(+2))\
    / inline-value\
  )\
  / inline-value
- **value**:\
  cnl()? ivalue(0) cnl()

# Examples
```ignore
- name: jack
  age: 18
- name: jones
  age: 21
  ids:
  - - 1
    - 2
  - [3, 4] ; inline
```
like JSON5:
```ignore
[
    {
        name: "jack",
        age: "18",
    },
    {
        name: "jones",
        age: "21",
        ids: [
            [1, 2],
            [3, 4],
        ]
    }
]
```

How To Use
==========
use cli utils:
```bash
cargo install syml --features cli-utils
```

use lib:
```rust
let _value = syml::parser::value(r#"
; comment
- {a: 1, b: 2}
- [3, 4]
- 5
- x: 2
  y: 3
"#).unwrap();
```
