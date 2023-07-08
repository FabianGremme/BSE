use alloc::rc::{Rc};
use core::cell::RefCell;
use alloc::{boxed::Box};
use spin::Mutex;
use core::ptr;
use core::mem;

use crate::mylib::queue as queue;
use crate::kernel::threads::thread as thread;
use crate::devices::cga as cga;
use crate::kernel::cpu as cpu;
use crate::kernel::threads::thread::Thread;



pub static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());


struct Dispatcher {
   	
}

const PRIO_2_LOWER_BOUND: i32 = 1000;
const PRIO_1_LOWER_BOUND: i32 = 500;

//Strafen zum Anpassen für die Threads
const PUNISHMENT_PRIO_2:i32 = -100;
const PUNISHMENT_PRIO_1:i32 = -50;
const PUNISHMENT_PRIO_0:i32 = 100;


pub struct Scheduler {
   active: *mut thread::Thread,
   prio_0: queue::Queue<Box<thread::Thread>>,
    prio_1: queue::Queue<Box<thread::Thread>>,
    prio_2: queue::Queue<Box<thread::Thread>>,// auf die CPU wartende Threads
   next_thread_id: u64,
    initialized: bool,
}

// Notwendig, da sonst der Compiler 'SCHEDULER' als nicht akzeptiert
unsafe impl Send for Scheduler {}


impl Scheduler {
	
   // Scheduler mit Ready-Queue anlegen
   pub const fn new() -> Self {
   
      /* Hier muss Code eingefuegt werden */
       Scheduler{
           active:ptr::null_mut(),
           prio_0: queue::Queue::new(),
           prio_1: queue::Queue::new(),
           prio_2: queue::Queue::new(),
           next_thread_id: 0,
           initialized: false,
       }
      
   }


   // ID fuer neuen Thread zurueckgeben
   pub fn get_next_tid (&mut self) -> u64 {

      /* Hier muss Code eingefuegt werden */
      return self.next_thread_id;

   }

    pub fn set_initialized(value: bool){
        unsafe{
            SCHEDULER.force_unlock();
        }
       SCHEDULER.lock().initialized = value;
    }
   
   
   /*****************************************************************************
    * Funktion:        schedule                                                 *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Scheduler starten. Wird nur einmalig gerufen und kehrt   *
    *                  nicht mehr zurueck.                                      *
    *****************************************************************************/
   pub fn schedule () {
    unsafe{
        SCHEDULER.force_unlock();
    }
      let to_start = Self::get_element_of_right_queue();
      if let Some(that) = to_start {
		 // Mit dem naechsten Aufruf uebernehmen wir das Memory-Mgmt.
		 // fuer 'that', ansonsten wuerde dies spaeter beim Umschalten
		 // zu frueh geloescht. Warum ist unklar. Durch das Speichern
		 // der Raw-Pointers muessen wir spaeter manuell 'drop' aufrufen
		 // Wir machen das in 'exitÄ'
		 let raw = Box::into_raw(that);
	
		 SCHEDULER.lock().active = raw;
	     thread::Thread::start( raw);
	  }
	  else {
 		cga::print_str("Panic: no thread, cannot start scheduler", cga::CGA_STD_ATTR);
		cpu::halt ();
	  }
   }
       
       
   /*****************************************************************************
    * Funktion:        ready                                                    *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Thread in readyQueue eintragen.                          *
    *                                                                           *
    * Parameter:                                                                *
    *      that        Einzutragender Thread                                    *
    *                                                                           *
    * Rückgabewert:                                                             *
    *      id          ID fuer den eingetragenen Thread                         *
    *****************************************************************************/
   pub fn ready (mut that: Box<dyn thread::ThreadEntry>) -> u64 {
       let currency = that.get_currency();
       unsafe{
           SCHEDULER.force_unlock();
       }
      let tid = SCHEDULER.lock().get_next_tid();        //damit die ids fortlaufend sind
       SCHEDULER.lock().next_thread_id = tid;
       SCHEDULER.lock().next_thread_id +=1;

       let thread_wrapper = thread::Thread::new(tid, that, currency);

       if currency >= PRIO_2_LOWER_BOUND{
           println!("ready 2 , currency:{}", currency);
           SCHEDULER.lock().prio_2.enqueue( thread_wrapper);
           return tid;
       }else if currency >= PRIO_1_LOWER_BOUND{
           println!("ready 1 , currency:{}", currency);
           SCHEDULER.lock().prio_1.enqueue( thread_wrapper);
           return tid;
       }else{
           println!("ready 0 , currency:{}", currency);
           SCHEDULER.lock().prio_0.enqueue( thread_wrapper);
           return tid;
       }


   }
    
    
   /*****************************************************************************
    * Funktion:        exit                                                     *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Thread ist fertig und terminiert sich selbst. Hier muss  *
    *                  nur auf den naechsten Thread umgeschaltet werden. Der    * 
    *                  aktuell laufende Thread ist nicht in der readyQueue.     *
    *****************************************************************************/
   pub fn exit (that: *mut thread::Thread) {
       cpu::disable_int();
	  // Naechsten Thread aus der Ready-Queue holen
       //wurde verändert, um die richtige queue zu erhalten
      let next = Self::get_element_of_right_queue();
      
      // Falls kein weiterer Thread wartet, abbrechen
      if next.is_none() {
         cga::print_str("Panic: cannot exit thread", cga::CGA_STD_ATTR);
         cpu::halt ();
	  }

      // Speicher des Aufrufers freigeben, siehe Beschreibung in 
      // 'schedule'	   
      unsafe {
         drop(Box::from_raw(that));
	  }
	  
      // Umschalten
      if let Some(nx) = next {
          // Aufruf verhindert, dass 'nx' geloescht wird, siehe auch
          // 'schedule'
		  let raw = Box::into_raw(nx);
          thread::Thread::switch( that, raw );
      }
       cpu::enable_int();
   }


   /*****************************************************************************
    * Funktion:        kill                                                     *
    *---------------------------------------------------------------------------*
    * Beschreibung:    Thread mit 'Gewalt' terminieren. Er wird aus der         *
    *                  readyQueue ausgetragen und wird dann nicht mehr aufge-   *
    *                  rufen. Der Aufrufer dieser Methode muss ein anderer      *
    *                  Thread sein.                                             *
    *                                                                           *
    * Parameter:                                                                *
    *      that        Zu terminierender Thread                                 *
    *****************************************************************************/
   pub fn kill (tokill_tid: u64) {

      /* Hier muss Code eingefuegt werden */
        cpu::disable_int();
        let dummy = Thread::new(tokill_tid, Box::new(Dummy{}), 0);

       // hier muss in allen Listen gesucht werden
       if !SCHEDULER.lock().prio_2.is_empty(){
           SCHEDULER.lock().prio_2.remove(dummy);
       }else if !SCHEDULER.lock().prio_1.is_empty(){
           SCHEDULER.lock().prio_1.remove(dummy);
       }else if !SCHEDULER.lock().prio_0.is_empty() {
           SCHEDULER.lock().prio_0.remove(dummy);
       }
       cpu::enable_int();

   }

    fn get_element_of_right_queue() -> Option<Box<Thread>>{
        cpu::disable_int();
        let mut current:Option<Box<Thread>>;
        if !SCHEDULER.lock().prio_2.is_empty(){
            cga::setpos(20, 14);
            print!("dequeue 2");
            current = SCHEDULER.lock().prio_2.dequeue();
            current.as_mut()?.reward_punish(PUNISHMENT_PRIO_2);
            cpu::enable_int();
            return current;
        }else if !SCHEDULER.lock().prio_1.is_empty(){
            cga::setpos(20, 14);
            print!("dequeue 1");
            current = SCHEDULER.lock().prio_1.dequeue();
            current.as_mut()?.reward_punish(PUNISHMENT_PRIO_1);
            cpu::enable_int();
            return current;
        }else if !SCHEDULER.lock().prio_0.is_empty(){
            cga::setpos(20, 14);
            print!("dequeue 0");
            current = SCHEDULER.lock().prio_0.dequeue();
            current.as_mut()?.reward_punish(PUNISHMENT_PRIO_0);
            cpu::enable_int();
            return current;
        }else{
            cpu::enable_int();
            return None;
        }
    }


   /*****************************************************************************
    * Funktion:        yield                                                    *
    *---------------------------------------------------------------------------*
    * Beschreibung:    CPU freiwillig abgeben und Auswahl des naechsten Threads.*
    *                  Naechsten Thread aus der readyQueue holen, den aktuellen *
    *                  aus der readyQueue austragen und auf den naechsten Thread*
    *                  umschalten.                                              *
    *                                                                           *
    * Achtung:         Falls nur der Idle-Thread läuft, so ist die readyQueue   * 
    *                  leer.                                                    *
    *****************************************************************************/
   pub fn yield_cpu (that: *mut thread::Thread) {
       cpu::disable_int();
       unsafe{
           SCHEDULER.force_unlock();
       }

      /* Hier muss Code eingefuegt werden */
       //dann neuen Thread mit dequeue aus der richtigen Queue holen
       /*if SCHEDULER.lock().prio_0.is_empty(){
           // es läuft nur idle
           return;
       }*/
       // hier muss in der richtigen Queue gesucht werden.
       // wird definitiv überschrieben
       let next = Self::get_element_of_right_queue();
       if next.is_none(){
           // es wurde nichts passendes gefunden
           cga::setpos(20, 13);
           print!("error");
           return;
       }

       //umschalten
       if let Some(nx) = next {
           unsafe {
               // hier muss die currency aus dem Thread genommen werden
               let current_currency = Box::from_raw(that).get_currency();
               cga::setpos(20, 15);
               print!("current_currency: {}", current_currency);

               if current_currency >= PRIO_2_LOWER_BOUND{
                   cga::setpos(20, 16);
                   print!("enqueue 2");
                   SCHEDULER.lock().prio_2.enqueue(Box::from_raw(that));
               }else{
                   if current_currency >= PRIO_1_LOWER_BOUND{
                       cga::setpos(20, 16);
                       print!("enqueue 1");
                       SCHEDULER.lock().prio_1.enqueue(Box::from_raw(that));
                   }else{
                       cga::setpos(20, 16);
                       print!("enqueue 0");
                       SCHEDULER.lock().prio_0.enqueue(Box::from_raw(that));
                   }
               }
           }
           let raw = Box::into_raw(nx);
           //neuen Thread in den scheduler einarbeiten
           SCHEDULER.lock().active = raw;


           //in Thread switch den Prozessor überarbeiten
           thread::Thread::switch( that, raw );
       }
       cpu::enable_int();


   }

}


// Dummy, wird zum Loeschen eines Threads benoetigt
// Siehe Queue::remove
struct Dummy { }

impl thread::ThreadEntry for Dummy {
	
    fn run(&mut self, thread_object: *mut thread::Thread) {
	}
    fn get_currency(&mut self)->i32{
        return 0;
    }

    fn reward_punish(&mut self, value:i32){}
}

impl Scheduler {
	
 
   /*****************************************************************************
    * Funktion:         preempt                                                 *
    *---------------------------------------------------------------------------*
    * Beschreibung:    CPU soll aktuellem Thread entzogen werden. Wird nur      *
    *                  aus dem Zeitgeber-Interrupt-Handler aufgerufen. Daher    *
    *                  muss nicht gegenueber Interrupts synchronisiert werden.  *
    *****************************************************************************/
   pub fn preempt (&mut self) {
		 
        /* Hier muss Code eingefuegt werden */

       if self.initialized == true{
           cga::setpos(20, 18);
           print!("timer ready");
            Scheduler::yield_cpu(self.active);
       }

   }
   
}

