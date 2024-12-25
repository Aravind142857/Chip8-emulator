## Steps
### Memory, memory map
#### Specification:
1. Memory = 4KB [Reserved = [0, 511], Program = [512, 4095]] => Struct {res: Byte[512], program: Byte[3583]}
2. SYSTEM_OFFSET: 4 BYTES = Address on system mapped to 0x00 on CHIP-8
### Registers
#### General Specification:
1. 16 8-bit registers: Global variables (V0-F)
2. 1 16-bit register: I
3. 2 8-bit registers for delay and sound timers: DT and ST
4. 16-bit PC
5. 8-bit SP
6. Stack: 16 16-bit array for addresses
#### Keyboard Specification:
#### Display:
1. 64x32 bit
2. Monochrome
3. Sprites to store graphics. Max size: 8x15 bit
4. Pre-loaded sprites for "0" to "F": 8x5 bits stored in reserve area
#### Timer and Sound:
1. Refresh Rate = 60Hz
2. When DT is non-zero, decrement at RR.
3. When ST is non-zero, decrement at RR and buzz with custom frequency.
### Instructions General:
1. 36 instructions.
2. Each instruction = 2 Bytes.
3. MSB first.
4. First byte should always be at even address.
5. Pad sprites if included.
### Instruction set:
1. 0nnn - SYS addr ( Jump to routine at nnn. Usually ignored. )
2. 00E0 - CLS ( Clear Screen. Set display to 0x00 )
3. 00EE - RET ( Return from subroutine. Set PC to addr at SP and decrement SP. )
4. 1nnn - JP addr (Jump to location. PC to nnn. )
5. 2nnn - CALL addr ( Increment SP. Put PC at SP. PC to nnn. )
6. 3xkk - SE Vx, byte ( Skip next instruction if vx = kk )
7. 4xkk - SNE Vx, byte ( Skip Not Equals )
8. 5xy0 - ( Skip if Vx = Vy )
9. 6xkk - ( Set Vx to kk )
10. 7xkk - ( Vx = Vx + kk )
11. 8xy0 - ( Vx = Vy )
12. 8xy1 - ( Vx = Vx | Vy )
13. 8xy2 - ( Vx = Vx & Vy )
14. 8xy3 - ( Vx = Vx ^ Vy )
15. 8xy4 - ( Vx = Vx + Vy. VF = Vx > 255? 1: 0 )
16. 8xy5 - ( VF = Vx > Vy ? 1: 0. Vx = Vx - Vy ) ## TODO: Check type of subtraction.
17. 8xy6 - ( Vx = Vx SHR 1. VF = Vx odd? 1: 0. Vx = Vx / 2 )
18. 8xy7 - ( VF = Vy > Vx ? 1: 0. Vx = Vy - Vx )
19. 8xye - ( VF = MSB Vx = 1? 1: 0. Vx = Vx * 2 )
20. 9xy0 - ( Skip next instruction if Vx != Vy )
21. Annn - ( I = nnn )
22. Bnnn - ( PC = nnn + V0 )
23. Cxkk - ( Vx = Rand byte & kk )
24. Dxyn - ( Read n bytes from mem representing the sprite, starting at address stored in I. Display bytes as sprites at coordinates (Vx, Vy). XORed onto screen. If pixels are erased set VF to 1, else 0. Sprites wrap around edges of screen )
25. Ex9e - ( Skip next instruction if key with val Vx is being pressed )
26. Exa1 - ( Skip next instruction if key with val Vx is NOT being pressed )
27. Fx07 - ( Vx = DT )
28. Fx0a - ( wait for key press, store value of key in Vx )
29. Fx15 - ( DT = Vx )
30. Fx18 - ( ST = Vx )
31. Fx1e - ( I += Vx )
32. Fx29 - ( I = Location of pre-loaded sprite of Vx )
33. Fx33 - ( Split the 3 digits of Vx into (nnn -> I, I+1, I+2) )
34. Fx55 - ( V0 - Vx stored from I to I + x )
35. Fx65 - ( I to I + x stored from V0 - Vx )