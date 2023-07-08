/*****************************************************************************
 *                                                                           *
 *                              p i t                                        *
 *                                                                           *
 *---------------------------------------------------------------------------*
 * Beschreibung:    Programmable Interval Timer.                             *
 *                                                                           *
 * Autor:           Michael Schoettner, HHU, 18.5.2023                       *
 *****************************************************************************/
#![allow(dead_code)]

use spin::Mutex;
use alloc::{boxed::Box};

use crate::devices::cga;
use crate::kernel::cpu;
use crate::kernel::interrupts::isr;
use crate::kernel::interrupts::pic;
use crate::kernel::interrupts::intdispatcher;
use crate::kernel::threads::scheduler;


// Ports
const PORT_CTRL:u16  = 0x43;
const PORT_DATA0:u16 = 0x40;


// Global PIT; fuer den Zugriff auf 'get_systime'
static mut TIME: Option<PIT> = None;				     

// Registrieren des Interrupt-Handlers
pub fn plugin() {
   unsafe {
      if TIME.is_none() {
	     PIT::init();
      }
      TIME.as_mut().unwrap().plugin();
  }	
 }

pub fn get_systime() -> u64 {
   unsafe {
      if TIME.is_none() {
	     return 0
	  }
      TIME.as_ref().unwrap().systime	
   }	
}


struct PIT { 
   systime: u64, 
   direction: u32,
}


/*****************************************************************************
 * Implementierung: PIT                                                      *
 *****************************************************************************/
impl PIT {
	
   /*****************************************************************************
    * Funktion:        init                                                     *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Alloziert globales Objekt TIME für den PIT.              *
    *****************************************************************************/
   fn init() {
		 
       /* Hier muss Code eingefuegt werden */
      unsafe {
         TIME = Some(PIT {
            systime: 0,
            direction: 0,
         });
      }

   }

   /*****************************************************************************
    * Funktion:        interval                                                 *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Zeitinervall programmieren.                              *
    *                                                                           *
    * Parameter:                                                                *
    *      us:         Zeitintervall in Mikrosekunden, nachdem periodisch ein   * 
    *                  Interrupt erzeugt werden soll.                           *
    *****************************************************************************/
   pub fn interval (us: u32) {

       /* Hier muss Code eingefuegt werden */
         let count = us * 1000/838;
         cpu::outb(PORT_CTRL, 0x34);
         cpu::outb(PORT_DATA0, (count &0xff) as u8);
         cpu::outb(PORT_DATA0, (count>>8 &0xff) as u8);
   }

   /*****************************************************************************
    * Funktion:        plugin                                                   *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Unterbrechungen fuer den PIT erlauben. Ab sofort wird    *
    *                  bei einem Timer-Interrupt die Funktion 'trigger'         *
    *                  aufgerufen.                                              *
    *****************************************************************************/
   pub fn plugin (&mut self) {
      // hier wurde code ergänzt
      PIT::interval(10000);
      intdispatcher::assign(intdispatcher::INT_VEC_TIMER, Box::new(PitISR));
      pic::allow(pic::IRQ_TIMER);
   }
   
}


/*****************************************************************************
 * Implementierung: ISR                                                      *
 *****************************************************************************/
struct PitISR;
impl isr::ISR for PitISR {

   /*****************************************************************************
    * Funktion:        trigger                                                  *
    *---------------------------------------------------------------------------*
    * Beschreibung:    ISR fuer den Zeitgeber. Wird aufgerufen, wenn der        * 
    *                  Zeitgeber eine Unterbrechung ausloest. Anzeige der Uhr   *
    *                  aktualisieren und Thread wechseln durch Setzen der       *
    *                  Variable 'threadSwitch', wird in 'int_disp' behandelt.   *
    *****************************************************************************/
   fn trigger(&self)  {


      // Systemzeit erhoehen
      /* Hier muss Code eingefuegt werden */
      unsafe{
         TIME.as_mut().unwrap().systime +=1;
      }

      // Alle 100 Ticks den Uhrzeiger rechts oben in der Ecke etwas
      // weiter drehen. Bei einer Unterbrechungsfrequenz von 100 Herz
      // bewegt er sich dann etwa im Sekunden Rhythmus.

      /* Hier muss Code eingefuegt werden */
      if get_systime() % 100 ==0{
         unsafe{
            TIME.as_mut().unwrap().direction +=1;
            if TIME.as_mut().unwrap().direction % 4 == 0{
               cga::show(70, 1, '|', cga::CGA_STD_ATTR);

            }else if TIME.as_mut().unwrap().direction % 4 == 1 {
               cga::show(70, 1, '/', cga::CGA_STD_ATTR);

            }else if TIME.as_mut().unwrap().direction % 4 == 2 {
               cga::show(70, 1, '-', cga::CGA_STD_ATTR);

            }else if TIME.as_mut().unwrap().direction % 4 == 3 {
               cga::show(70, 1, '\\', cga::CGA_STD_ATTR);
            }
         }
      }

      // intdispatcher entsperren, sonst gibt es einen Deadlock
      // (wir kehren vorerst nicht zurueck)
      unsafe {
         intdispatcher::INT_VECTORS.force_unlock();
      }

      // Auch den Scheduler sicherheitshalber entsperren, um Dedlocks
      // zu vermeiden
      unsafe {
         scheduler::SCHEDULER.force_unlock();
      }

      // Bei jedem Tick einen Threadwechsel ausloesen.
      scheduler::SCHEDULER.lock().preempt();
      
   }
}
