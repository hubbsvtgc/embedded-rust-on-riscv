
// FE310 has two uart instances with base addresses 0x10013000 & 0x10023000
pub const UART0_BASE_ADDR: usize = 0x10013000;
pub const UART1_BASE_ADDR: usize = 0x10023000;

pub struct Uart {
    base_address: usize, // Base address of UART register block
}

impl Uart {

    // Get one of the instance with base address

    pub fn from_base_address (base_address: usize) -> Self {
        Uart { base_address }
    }

    // Initialize the UART with desired baud_rate
    // May be should provide a enum with supported baud rates

    pub fn init(&self, baud_rate: u32) {

        unsafe {

            if baud_rate != 115200 {
                panic!("Invalid baud");
            }

            let div_register = (self.base_address + 0x18) as *mut u32;
            div_register.write_volatile(138); /* Baud 115200 at tlclk 16 Mhz */
    
            // At reset/default, number of stop bits in txctrl is 1 
            // No tx/rx watermark interrupts
    
            /* Enable tx */
    
            let tx_ctrl = (self.base_address + 0x8) as *mut u32;
            tx_ctrl.write_volatile(tx_ctrl.read_volatile() | 0x1 as u32);
    
            /* Enable rx */
            let rx_ctrl = (self.base_address + 0xC)  as *mut u32;
            rx_ctrl.write_volatile(rx_ctrl.read_volatile() | 0x1 as u32);
        }
    }

    /// Send a byte
    /// Writing to txdata register, enqueues the character 
    /// to transmit FIFO if FIFO is able to accept new entries.
    /// Reading from txdata returns the currrent value of the full flag
    /// and zero in the data field.
    /// 
    /// ```
    /// --------------------------
    /// | FULL | RESERVED | DATA |
    /// --------------------------
    /// | 31   | 30 : 8 | [7:0]|
    /// --------------------------
    /// ```
    /// 
    /// `Full` flag indicates whether the FIFO is able to accept new entries.
    /// When `full` flag is set, writes are ignored. 
    /// RISC-V `amoor.w` instruction can be used to both read the status 
    /// and attempt to enqueue data with a non-zero return value 
    /// indicating the character was NOT accepted.

    pub fn send_byte(&self, byte: u8) {
        unsafe {
            // Write to the UART data register
            let data_register = self.base_address as *mut u8;
            data_register.write_volatile(byte);
        }
    }

    /// Receive a byte
    /// Reading the rxdata register dequeues the data from the receive FIFO
    /// and returns the value in the data field. The 'empty` flag indicates if
    /// fifo is empty and when set 'data` field dont have valid data. 
    /// Writes to `rxdata` register are ignored. 
    /// 
    /// ```
    /// --------------------------
    /// | EMPTY | RESERVED | DATA |
    /// --------------------------
    /// | 31   | [30: 8]  | [7:0]|
    /// --------------------------
    /// ```

    pub fn receive_byte(&self) -> u8 {
        unsafe {
            // Read from the UART data register
            
            let data_register = (self.base_address +0x04) as *mut u8;
            data_register.read_volatile()
        }
    }

    // Check if data is available to read
    pub fn is_data_ready(&self) -> bool {
        unsafe {
            // Check rx data register
            let status_register = (self.base_address + 0x04) as *mut u32; 
            status_register.read_volatile() & 0x8000_0000 == 0 // #31 empty
        }
    }

    // Check if TX is ready to transmit data
    pub fn is_tx_ready(&self) -> bool {
        unsafe {
            let status_register = (self.base_address ) as *mut u32; // Example offset
            status_register.read_volatile() & 0x8000_0000 == 0 // #31 full
        }
    }
}
