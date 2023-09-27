if exists("b:current_syntax")
		finish
endif

let b:current_syntax = "snow"

hi def link assignmentKeywords Keyword
hi def link keywords Keyword
hi def link types Type
hi def link word Macro

syn keyword assignmentKeywords let const function
syn keyword keywords use rust write push while if elif else end return
syn keyword types string i8 i16 i32
syn match word '\a\a*' contained
syn match Number '[0-9]'
syn match Operator '[\+\-\*\/]'
syn match Comment '//.*\n'
syn match String '\".\{-}\"'
syn match String '\'.\{-}\''
syn match Keyword '\w*\d*(\@='
