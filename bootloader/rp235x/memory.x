INCLUDE flash.x

MEMORY {
    /* Bootloader executable parttion */
    FLASH            : ORIGIN = BOOTLOADER_ORIGIN, LENGTH = BOOTLOADER_LENGTH
    BOOTLOADER_STATE : ORIGIN = BOOTLOADER_STATE_ORIGIN, LENGTH = BOOTLOADER_STATE_LENGTH
    ACTIVE           : ORIGIN = ACTIVE_ORIGIN, LENGTH = ACTIVE_LENGTH
    DFU              : ORIGIN = DFU_ORIGIN, LENGTH = DFU_LENGTH

    RAM              : ORIGIN = 0x20000000, LENGTH = 512K
}

SECTIONS {
    .start_block : ALIGN(4) {
    __start_block_addr = .;
    KEEP(*(.start_block));
    KEEP(*(.boot_info));
    } > FLASH
} INSERT AFTER .vector_table;

_stext = ADDR(.start_block) + SIZEOF(.start_block);

SECTIONS {
    .bi_entries : ALIGN(4) {
    __bi_entries_start = .;
    KEEP(*(.bi_entries));
    . = ALIGN(4);
    __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .text;

SECTIONS {
    .end_block : ALIGN(4) {
    __end_block_addr = .;
    KEEP(*(.end_block));
    } > FLASH
} INSERT AFTER .uninit;

PROVIDE(start_to_end = __end_block_addr - __start_block_addr);
PROVIDE(end_to_start = __start_block_addr - __end_block_addr);

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(FLASH);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(FLASH);
__bootloader_active_start = ORIGIN(ACTIVE) - ORIGIN(FLASH);
__bootloader_active_end = ORIGIN(ACTIVE) + LENGTH(ACTIVE) - ORIGIN(FLASH);
__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(FLASH);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(FLASH);
