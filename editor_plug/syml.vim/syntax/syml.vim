" Vim syntax file
" Language:		syml
" Maintainer:		A4-Tacks <wdsjxhno1001@163.com>
" Last Change:		2024-04-17
" URL:			https://github.com/A4-Tacks/syml-rs

if exists("b:current_syntax")
    finish
endif

" syntax {{{1
syn case match

setlocal iskeyword=@,39,45,48-57,a-z,A-Z,92,95,96,124,126,!,#,$,%,&,(,),*,+,.,/,<,=,>,?,@,^
setlocal shiftwidth=2

syn match symlKey /:/
syn match symlValue /\%([!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~\\]\|-\%( \)\@!\)[!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~'\\\-]*/
syn match symlKey /\%([!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~\\]\|-\%( \)\@!\)[!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~'\\\-]*[ \t]*:/
syn match symlValue /'[^']*'/
syn match symlKey /'[^']*'[ \t]*:/
syn region symlValue start=/"/ end=/"/ contains=symlEscape,symlEscapeErr
syn match symlEscapeErr /\\./ contained
syn match symlEscape /\\[nrt"' \t\\]/ contained
syn match symlEscape /\\;\@=/ nextgroup=symlComment contained
syn match symlEscape /\\\ze\r\=\n/ contained
syn match symlEscape /\\x\x\{2}/ contained
syn match symlEscape /\\u\x\{4}/ contained
syn match symlEscape /\\u{\x\+}/ contained
syn match symlEscape /\\U\x\{8}/ contained
syn match symlArray /- /
syn region symlInlineArray start=/\[/ end=/]/ contains=TOP,symlArray,symlKey
syn region symlInlineTable start=/{/ end=/}/ contains=TOP,symlArray
syn match symlComment /;.*/

" link color {{{1
hi def link symlValue String
hi def link symlEscape SpecialChar
hi def link symlEscapeErr Error
hi def link symlArray Repeat
hi def link symlInlineArray NONE
hi def link symlInlineTable NONE
hi def link symlKey Keyword
hi def link symlComment Comment
" }}}1

let b:current_syntax = 'syml'
