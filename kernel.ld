KILO = 1024;               /*1KiB, o 1024B*/
MEGA = 1024 * KILO;        /*1MiB, o 1024KiB*/
GIGA = 1024 * MEGA;        /*1GiB, o 1024MiB*/

STACK_SIZE = 512 * KILO;   /*512 KiB, o mezzo MiB per lo stack*/
HEAP_SIZE = 32 * MEGA;     /*32 MiB per il heap*/
ALIGNAMENT = 8;            /*in arch 64 bit e' 8 B, in 32 e' 4 B*/
STACK_HEAP_ALIGNAMENT = 16; /*alignament of the stack and the HEAP*/
MEM_START = 0x80200000;    /*Start of the program*/

ENTRY(boot) /*punto di partenza del kernel*/

SECTIONS {
    . = MEM_START;

    /*logica del codice, funzioni*/
    .text :{
        KEEP(*(.text.boot));
        *(.text .text.*);
        . = ALIGN(ALIGNAMENT);
        *(.text.stvec);
    }

    /*Read Only data, dati immutabili*/
    .rodata : ALIGN(ALIGNAMENT){
        *(.rodata .rodata.*);
    }

    /*dati globali*/
    .data : ALIGN(ALIGNAMENT){
        *(.data .data.*);
    }

    /*valori a 0, a differenza delle altre sezioni non e' inizializata*/
    .bss : ALIGN(ALIGNAMENT){
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        __bss_end = .;
    }

    . = ALIGN(STACK_HEAP_ALIGNAMENT);/*RISC V vuole che l'allineamento nello stack sia 16*/
    __heap = .;
    . += HEAP_SIZE;
    __heap_end = .;



    . += STACK_SIZE;
    __stack_top = .;      /*dove parte lo stack, quando lo stack cresce il suo valore decresce*/


}
