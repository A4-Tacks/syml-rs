" Vim syntax file
" Language:		syml
" Maintainer:		A4-Tacks <wdsjxhno1001@163.com>
" Last Change:		2024-04-19
" URL:			https://github.com/A4-Tacks/syml-rs

if exists("b:did_indent")
    finish
endif
let b:did_indent = 1

function! s:filter_line(l) " {{{1
    let l = substitute(a:l, '"\%(\\["\\]\|[^"]\)*\%("\|\\\r\=\n\)|'
                \."'[^']*'", "''", 'g')

    let l = substitute(l, '\%(^ *\|[ \t]*$\)', '', 'g')
    let l = substitute(l, '^\(;\).*\|;.*', '\1', 'g')
    return l
endfunction
function! s:prevnonblank(num) " {{{1
    let num = a:num
    while num > 0 && <SID>filter_line(getline(num)) =~# '^;\=$'
        let num -= 1
    endwhile
    return num
endfunction
function! s:indent(num) " {{{1
    return match(getline(a:num), '^ *\%(- \)*\zs')
endfunction

function! GetSymlIndent() " {{{1
    if v:lnum <= 1 | return <SID>indent(v:lnum) | endif
    let lnum = v:lnum
    let pnum = <SID>prevnonblank(lnum - 1)

    let line = <SID>filter_line(getline(lnum))
    if line =~# '[^-]' | return <SID>indent(lnum) | endif

    let pline = <SID>filter_line(getline(pnum))

    let diff = 0

    if pline =~# ':$'
        let diff += 1
    endif
    if line =~# '^-'
        let diff -= 1
    endif

    let res_indent = max([0, <SID>indent(pnum)]) + diff * &shiftwidth

    if res_indent > <SID>indent(lnum) && line =~# '^-'
        return <SID>indent(lnum)
    endif
    if res_indent > <SID>indent(lnum) && empty(line) && pnum =~# '^-'
        return <SID>indent(lnum)
    endif

    return res_indent
endfunction
function! GetSymlFold() "{{{1
    let line = getline(v:lnum)
    if line =~# '^\s*$'
        return '='
    endif
    return max([0, match(line, '^ *\%(- \)\=\zs\S')]) / 2
endfunction
" }}}1

setlocal indentexpr=GetSymlIndent()
setlocal indentkeys=0-,<:>,o,O,e
setlocal foldmethod=expr
setlocal foldexpr=GetSymlFold()
