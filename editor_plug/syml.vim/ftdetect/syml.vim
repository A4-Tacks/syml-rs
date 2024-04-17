" vint: -ProhibitAutocmdWithNoGroup

autocmd BufRead,BufNewFile *.syml if &ft !=# 'syml' | setf syml | en
