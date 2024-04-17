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
    / comment? `\r`? `\n` \[ \\t]\*\
  )
- **simple-val**:\
  ([!#$%&()\*+./0-9<=>?@A-Z\\\\^\_\`a-z|\~\p{XID\_Start}] / `-` !` `)\
  [!#$%&()\*+./0-9<=>?@A-Z\\\\^\_\`a-z|\~\\\-'\p{XID\_Continue}]\*\
  / `'` \[^'\r\n] `'`\
  / `"` (escape\* / \[^\\r\\n\\\\]) `"`\
- **inline-value**:\
  `[` _ (simple-val (_ `,` _ simple-val)* _ `,`?)? _ `]`\
  / `{` _ (simple-val _ `:` _ inline-value (_ `,` simple-val _ `:` _ inline-value)* _ `,`?)? _ `}`\
  / simple-val
- **list**:\
  `- ` value `\n` (cnl indent(+2) `- ` value(indent+2))\*
- **value**:\
  list\
  / simple-val _ `:` (_ simple-val _ `:`)\* (\
    cnl() (indent list(indent) / indent(+2) value(indent+2))\
    / inline-value\
  )\
  / inline-value

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
