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
    while num > 0 && s:filter_line(getline(num)) =~# '^;\=$'
        let num -= 1
    endwhile
    return num
endfunction
function! s:nextnonblank(num) " {{{1
    let num = a:num
    let eof = line('$')
    while num <= eof && s:filter_line(getline(num)) =~# '^;\=$'
        let num += 1
    endwhile
    return num
endfunction
function! GetSymlIndent() " {{{1
    if v:lnum <= 1 | return indent(v:lnum) | endif
    let lnum = v:lnum
    let line = s:filter_line(getline(lnum))
    if line !~# '^-\=$' | return indent(lnum) | endif

    let pnum = s:prevnonblank(lnum - 1)
    let raw_pline = getline(pnum)
    let pline = s:filter_line(raw_pline)

    let diff = 0

    if pline =~# ':$'
        let diff += 1
    endif
    if line =~# '^-'
        let diff -= 1
    endif

    let res_indent = match(raw_pline, '^ *\%(- \)*\zs') + diff * &shiftwidth

    if res_indent > indent(lnum) && line =~# '^-'
        return indent(lnum)
    endif

    return res_indent
endfunction
function! GetSymlFold() "{{{1
    let line = getline(v:lnum)
    if line =~# '^\s*$'
        return '='
    endif
    let indent = match(line, '^ *\%(- \)\=\zs\S')
    if s:filter_line(getline(v:lnum)) =~# ':$'
        let nnum = s:nextnonblank(v:lnum+1)
        return '>'.(match(getline(nnum), '^ *\%(- \)\=\zs\S') / 2)
    endif
    return max([0, indent]) / 2
endfunction
" }}}1

setlocal indentexpr=GetSymlIndent()
setlocal indentkeys=0-,<->,o,O,e
setlocal foldmethod=expr
setlocal foldexpr=GetSymlFold()
