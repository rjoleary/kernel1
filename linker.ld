ENTRY(start)

SECTIONS {
    . = 1M;

    .text : {
        _TextStart = . ;
        *(.text.begin)
        *(.text*)
        _TextEnd = . ;
    }

    .data : {
        _DataStart = . ;
        *(.data*)
        _DataEnd = . ;
    }

    .bss : {
        _BssStart = . ;
        *(.bss*)
        _BssEnd = . ;
    }
}
