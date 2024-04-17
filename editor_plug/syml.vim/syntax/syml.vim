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

syn match symlValue /\%([!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~\\]\|-\%( \)\@!\)[!#$%&()\*+./0-9<=>?@A-Z^\_\`a-z|~'\\\-]*/
syn match symlValue /'[^']*'/
syn region symlValue start=/"/ end=/"/ contains=symlEscape,symlInStringComment
syn match symlEscape /\\[nrt"' \t\\]/ contained
syn match symlEscape /\\;\@=/ contained
syn match symlEscape /\\\r\=\n/ contained
syn match symlArray /- /
syn region symlInlineArray start=/\[/ end=/]/ contains=TOP,symlArray,symlKeyval
syn region symlInlineTable start=/{/ end=/}/ contains=TOP,symlArray
syn match symlKeyval /:/
syn match symlComment /;.*/
syn match symlInStringComment /\\\@<=;.*/ contained

" link color {{{1
hi def link symlValue String
hi def link symlEscape SpecialChar
hi def link symlArray Repeat
hi def link symlInlineArray NONE
hi def link symlInlineTable NONE
hi def link symlKeyval Identifier
hi def link symlComment Comment
hi def link symlInStringComment Comment
" }}}1

let b:current_syntax = 'syml'
