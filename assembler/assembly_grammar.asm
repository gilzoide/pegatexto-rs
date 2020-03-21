.Grammar
  qinit
  call .Rule
  qleast 1
  ret

.Rule
  push
  byte '.'
  jmpf fail
  call .Identifier
  jmpf fail
  call .EOL
  jmpf fail
  call .CommandOrLabel
  jmps end
fail:
  peek
end:
  pop
  ret

.CommandOrLabel  ; (Command / Label)+
  qinit
;; Command / Label
  call .Command
  jmps success
  call .Label
  jmps success
success:

  qleast 1
  ret

.Command
indent:
  set " \t"
  jpms indent:
  succ

  push
  call .Nop
  jmps success
  peek
  call .Succeed
  jmps success
  peek
  call .Fail
  jmps success
  peek
  call .FailIfLessThan
  jmps success
  peek
  call .ToggleSuccess
  jmps success
  peek
  call .QcZero
  jmps success
  peek
  call .QcIncrement
  jmps success
  peek
  call .Jump
  jmps success
  peek
  call .Call
  jmps success
  peek
  call .Return
  jmps success
  peek
  call .Push
  jmps success
  peek
  call .Peek
  jmps success
  peek
  call .Pop
  jmps success
  peek
  call .Byte
  jmps success
  peek
  call .NotByte
  jmps success
  peek
  call .Class
  jmps success
  peek
  call .Literal
  jmps success
  peek
  call .Set
  jmps success
  peek
  call .Range
  jmps success
  peek
  call .Action
  jmps success
  peek
success:
  pop

  jmpf end
  call .EOL
end:
  ret

.Label
  call .Identifier
  jmpf end
  byte ':'
  jmpf end
  call .EOL
end:
  ret

.Nop
  str "nop"
  ret

.Succeed
  str "succ"
  ret

.Fail
  str "fail"
  ret

.FailIfLessThan
  str "flt"
  jmpf end
  call .IntParam
end:
  ret

.ToggleSuccess
  str "togl"
  ret

.QcZero
  str "qcz"
  ret

.QcIncrement
  str "qci"
  ret

.Jump
  str "jmp"
  jmpf end
  set "fs"
  succ
  call .LabelParam
end:
  peekf
  ret

.Call
  str "call"
  jmpf end
  call .LabelParam
end:
  ret

.Return
  str "ret"
  ret

.Push
  str "push"
  ret

.Peek
  str "peek"
  ret

.Pop
  str "pop"
  ret

.Byte
  str "byte"
  jmpf end
  call .CharParam
end:
  ret

.NotByte
  str "nbyte"
  jmpf end
  call .CharParam
end:
  ret

.Class
  str "cls"
  jmpf end
  call .CharClassParam
end:
  ret

.Literal
  str "str"
  jmpf end
  call .StringParam
end:
  ret

.Set
  str "set"
  jmpf end
  call .StringParam
end:
  ret

.Range
  str "rng"
  jmpf end
  call .IntParam
  jmpf end
  call .IntParam
end:
  ret

.Action
  str "act"
  jmpf end
  call .LabelParam
end:
  ret

.Identifier
  cls a
  jmpf end
expression:
  cls w
  jmps expression
  succ
end:
  ret

.IntParam
  qinit
  byte ' '
  qleast 1
  jmpf end

  qinit
  cls d
  qleast 1
end:
  ret
