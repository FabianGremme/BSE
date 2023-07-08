/****************************************************************************
 *                                                                          *
 *                                    c g a                                 *
 *                                                                          *
 *--------------------------------------------------------------------------*
 * Beschreibung:    Mit Hilfe dieses Moduls kann man auf den Textbildschirm *
 *                  des PCs zugreifen. Der Zugriff erfolgt direkt auf der   * 
 *                  Hardwareebene, d.h. ueber den Bildschirmspeicher und    *
 *                  den I/O-Ports der Grafikkarte.                          *
 *                                                                          *
 * Autor:           Michael Schoetter, HHU Duesseldorf, 27.12.2021          *
 ****************************************************************************/


use crate::kernel::cpu as cpu;


// make type comparable, printable and enable copy semantics
#[allow(dead_code)]   // avoid warnings for unused colors
#[repr(u8)]           // store each enum variant as an u8
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}


pub const CGA_STD_ATTR: u8       = (Color::Black as u8) << 4 | (Color::Green as u8);

const CGA_BASE_ADDR: u32     = 0xb8000;
const CGA_ROWS   : u32       = 25;
const CGA_COLUMNS: u32       = 80;

const CGA_INDEX_PORT: u16    = 0x3d4;  // select register
const CGA_DATA_PORT: u16     = 0x3d5;  // read/write register
const CGA_HIGH_BYTE_CMD: u8  = 14;     // cursor high byte
const CGA_LOW_BYTE_CMD: u8   = 15;     // cursor high byte


/*****************************************************************************
 * Funktion:        clear                                                    *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Lösche den Textbildschirm.                               *
 *****************************************************************************/
pub fn clear() {

   /* Hier muss Code eingefuegt werden */
    for y in 0..CGA_ROWS{
        for x in 0..CGA_COLUMNS{
            show(x, y, ' ', CGA_STD_ATTR);
        }
    }
    setpos(0, 0);

}


/*****************************************************************************
 * Funktion:        show                                                     *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Anzeige eines Zeichens mit Attribut an einer bestimmten  *
 *                  Stelle auf dem Bildschirm.                               *
 *                                                                           *
 * Parameter:                                                                *
 *      x,y         Position des Zeichens                                    *
 *      character   Das auszugebende Zeichen                                 *
 *      attrib      Attributbyte fuer das Zeichen                            *
 *****************************************************************************/
pub fn show (x: u32, y: u32, character: char, attrib: u8) {
    let pos: u32;

    if x>CGA_COLUMNS || y>CGA_ROWS
    {    
		return ; 
    }
    
    pos = (y * CGA_COLUMNS + x) * 2;

    unsafe {
        *((CGA_BASE_ADDR + pos) as *mut u8)     = character as u8;
        *((CGA_BASE_ADDR + pos + 1) as *mut u8) = attrib;
    }
}


/*****************************************************************************
 * Funktion:        getpos                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Abfragem der Cursorposition                              *
 *                                                                           *
 * Rückgabewerte:   x und y                                                  *
 *****************************************************************************/
pub fn getpos () -> (u32, u32) {

   /* Hier muss Code eingefuegt werden */
    let mut pos = 0 as u32;
    cpu::outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);
    pos |= cpu::inb(CGA_DATA_PORT) as u32;
    cpu::outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);
    pos |= (cpu::inb(CGA_DATA_PORT) as u32) << 8;
    let y = pos/CGA_COLUMNS;
    let x = pos%CGA_COLUMNS;
    return (x, y);
}


/*****************************************************************************
 * Funktion:        setpos                                                   *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Setzen des Cursors in Spalte x und Zeile y.              *
 *****************************************************************************/
pub fn setpos (x:u32, y:u32) {

   /* Hier muss Code eingefuegt werden */
    let pos: u32;
    pos = y * CGA_COLUMNS + x;
    cpu::outb(CGA_INDEX_PORT, CGA_LOW_BYTE_CMD);
    cpu::outb(CGA_DATA_PORT, (pos & 0xFF)as u8);
    cpu::outb(CGA_INDEX_PORT, CGA_HIGH_BYTE_CMD);
    cpu::outb(CGA_DATA_PORT, ((pos >> 8) & 0xFF)as u8);

}


/*****************************************************************************
 * Funktion:        print_dec                                                *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Ausgabe eines u32 Wertes als Dezimal-Zahl, ohne führende *
 *                  Null, an der aktuellen Cursorposition mit dem Standard-  *
 *                  Attribut.                                                *
 *                                                                           *
 * Parameter:       zahl       auszugebende Hex-Zahl                         *
 *****************************************************************************/
pub fn print_dec (zahl: u32) {

   /* Hier muss Code eingefuegt werden */
    print!("{}", zahl);

}

 
/*****************************************************************************
 * Funktion:        print_hex                                                *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Ausgabe eines u32 Wertes als Hex-Zahl, ohne führende     *
 *                  Null, an der aktuellen Cursorposition mit dem Standard-  *
 *                  Attribut.                                                *
 *                                                                           *
 * Parameter:       zahl       auszugebende Hex-Zahl                         *
 *****************************************************************************/
pub fn print_hex (zahl: u32) {

   /* Hier muss Code eingefuegt werden */
    print!("{:x}", zahl);


}

 
/*****************************************************************************
 * Funktion:        print_byte                                               *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Ausgabe eines Bytes an aktuellen Cursorposition mit dem  *
 *                  Standard-Attribut. '\n' fuer Zeilenvorschub.             *
 *                                                                           *
 * Parameter:       b       auszugebendes Zeichen                            *
 *****************************************************************************/
pub fn print_byte (b: u8) {

   /* Hier muss Code eingefuegt werden */
    let (mut x,mut y) = getpos();
    let mychar = b as char;
    if mychar == '\n'{
        y = y + 1;
        x = 0;
    }else{
        show(x, y, mychar, CGA_STD_ATTR);
        x = x + 1;
        if x > CGA_COLUMNS{
            x = 0;
            if y + 1< CGA_ROWS{
                y = y +1;
            }else{
                scrollup();
            }
        }
    }

    setpos(x, y);

}


/*****************************************************************************
 * Funktion:        print_str                                                *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Ausgabe einer Zeichenkette, ab der aktuellen Cursor-     *
 *                  position. '\n' fuer Zeilenvorschub.                      *
 *                                                                           *
 * Parameter:       string      Auszugebende Zeichenkette                    *
 *                  attrib      Attributbyte fuer alle Zeichen der Z.kette   *
 *****************************************************************************/
pub fn print_str (string: &str, attrib: u8) {

   /* Hier muss Code eingefuegt werden */
    let (mut x,mut y) = getpos();
    for z in string.chars(){
        show(x, y, z, attrib);
        x = x + 1;
        if x > CGA_COLUMNS{
            x = 0;
            if y + 1 < CGA_ROWS{
                y = y +1;
            }else{
                scrollup();
            }
        }
    }
    setpos(x, y);


}
    

/*****************************************************************************
 * Funktion:        scrollup                                                 *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Verschiebt den Bildschirminhalt um eine Zeile nach oben. *
 *                  Die neue Zeile am unteren Bildrand wird mit Leerzeichen  *
 *                  gefuellt.                                                *
 *****************************************************************************/
pub fn scrollup () {

   /* Hier muss Code eingefuegt werden */
    print!("sclollup");
    for y in 1..CGA_ROWS{
        for x in 0..CGA_COLUMNS{
            let pos: u32;

            if x>CGA_COLUMNS || y>CGA_ROWS
            {
                return ;
            }

            pos = (y * CGA_COLUMNS + x) * 2;

            unsafe {
                let current_character = *((CGA_BASE_ADDR + pos) as *mut u8) as char;
                let current_attrib = *((CGA_BASE_ADDR + pos + 1) as *mut u8);
                show(x, y-1, current_character, current_attrib);
            }
        }
    }
    // hier wird die letzte Zeile neu hinzugefuegt
    for x in 0..CGA_COLUMNS{
        show(x, CGA_ROWS-1, ' ', CGA_STD_ATTR);
    }

}
 
 
/*****************************************************************************
 * Funktion:        attribute                                                *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Hilfsfunktion zur Erzeugung eines Attribut-Bytes aus     *
 *                  Hintergrund- und Vordergrundfarbe und der Angabe, ob das *
 *                  Zeichen blinkend darzustellen ist.                       *
 *                                                                           *
 * Parameter:       bg          Background color                             *
 *                  fg          Foreground color                             *
 *                  blink       yes/no                                       *
 *                                                                           *
 * Rückgabewert:    u8          Attribut-Code                                *
 *****************************************************************************/
pub fn attribute (bg: Color, fg: Color, blink: bool) -> u8 {

   /* Hier muss Code eingefuegt werden */
    let mut output = 0 as u8;
    output = output + blink as u8;
    output = output + (bg as u8) <<1;
    output = output + (fg as u8) <<3;
   
   return output;
}
