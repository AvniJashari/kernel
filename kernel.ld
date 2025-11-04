STACK_SIZE = 512K; /* 512 KB, o mezzo MB*/
OFFSET = 8; /*in arch 64 bit e' 8 B, in 32 e' 4 B*/
MEM_START = 0x80200000;

ENTRY(boot) /*punto di partenza del kernel*/

SECTIONS {
    . = MEM_START;

    .text :{/*logica del codice, funzioni*/
        KEEP(*(.text.boot));
        *(.text .text.*);
        . = ALIGN(8);
        *(.text.stvec);
    }

    .rodata : ALIGN(OFFSET){/*Read Only data, dati immutabili*/
        *(.rodata .rodata.*);
    }

    .data : ALIGN(OFFSET){/*dati globali*/
        *(.data .data.*);
    }

    .bss : ALIGN(OFFSET){/*valori a 0, a differenza delle altre sezioni non e' inizializata*/
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(OFFSET * 2);/*RISC V vuole che l'allineamento nello stack sia 16*/
    . += STACK_SIZE;
    __stack_top = .;/*dove parte lo stack, quando lo stack cresce il suo valore decresce*/
}
