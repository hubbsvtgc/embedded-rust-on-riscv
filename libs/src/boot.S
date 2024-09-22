
.globl __boot_; /* To be visible to linker for entry */

.section .boot /* To keep in a particular location */

__boot_:

    # Disable INDIVIDUAL Machine software(bit#3), timer(bit#7) & 
    # external(bit#11) machine interrupts while we set boot code

    csrw mie, zero;

    /* set stack pointer */
    lui sp, %hi(__stack_start_);
    addi sp, sp, %lo(__stack_start_);

