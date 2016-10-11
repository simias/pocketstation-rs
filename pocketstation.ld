MEMORY
{
    /* RAM, first 512 bytes are reserved for kernel */
    RAM (rwx) : ORIGIN = 512, LENGTH = 2 * 1024 - 512,
    FLASH (rx) : ORIGIN = 0x02000000 + 0x4400, LENGTH = 16 * 1024
}

ENTRY(main);

SECTIONS
{
	__exidx_start = .;

	.text :
	{
		*(.text*)
	} > FLASH

	.data :
	{
		*(.data*)
		*(.rodata*)
	} > FLASH

	.bss :
	{
		*(.bss*)
		*(COMMON)
	} > FLASH

	__exidx_end = .;
}
