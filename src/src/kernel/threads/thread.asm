;*****************************************************************************
;*                                                                           *
;*                  C O R O U T I N E                                        *
;*                                                                           *
;*---------------------------------------------------------------------------*
;* Beschreibung:    Assemblerfunktionen zum Starten des ersten Koroutine und *
;*                  zum Umschalten zwischen Koroutinen.                      *
;*                                                                           *
;* Autor:           Michael, Schoettner, HHU, 14.03.2023                     *
;*****************************************************************************


; EXPORTIERTE FUNKTIONEN

[GLOBAL _thread_start]
[GLOBAL _thread_switch]

; IMPLEMENTIERUNG DER FUNKTIONEN

[SECTION .text]
[BITS 64]


; _coroutine_start: Startet die erste Koroutine
;
; C Prototyp:      void _coroutine_start(context: *mut c_void); 
 _thread_start:

; * 
; * Hier muss Code eingefuegt werden
; * 

mov rsp, [rdi + 8]

popfq
pop rbp
pop rdi
pop rsi
pop rdx
pop rcx
pop rbx
pop rax

pop r15
pop r14
pop r13
pop r12
pop r11
pop r10
pop r9
pop r8

sti
ret



;
; _coroutine_switch: Coroutinen-Umschaltung. Der aktuelle Registersatz wird
;                    auf dem Stack gesichert und der Registersatz der
;                    neuen Coroutine wird in den Prozessor geladen.
;
; C Prototyp:       void _coroutine_switch (context_now: *mut c_void, context_then: *mut c_void);
_thread_switch:
; *
; * Hier muss Code eingefuegt werden
; *

push r8
push r9
push r10
push r11
push r12
push r13
push r14
push r15

push rax
push rbx
push rcx
push rdx

push rsi
push rdi
push rbp
pushfq

mov [rdi +8], rsp

mov rsp, [rsi +8]

popfq
pop rbp
pop rdi
pop rsi
pop rdx
pop rcx
pop rbx
pop rax

pop r15
pop r14
pop r13
pop r12
pop r11
pop r10
pop r9
pop r8

sti
ret
