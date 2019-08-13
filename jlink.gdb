target remote :2331
monitor semihosting enable
monitor semihosting breakOnError
monitor semihosting IOclient 2
load
break main
continue
