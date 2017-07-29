use super::types::KeyCode;

pub enum Signal {
//Common ASCII characters first.
//	A-Z
//	0-9
//	Minus, Equals
//	Grave/Tilde
//	Left and Right Bracket
//	Backslash
//	Semicolon and Apostrophe
//	Comma, Period, and Forward Slash
//	Escape
//	F1-F12
//	Backspace
//	Tab
//	CapsLock
//	Enter (Return on Mac keyboards)
//	Left and Right Shift
//	Left and Right Control
//	Left and Right Alt
//	Arrow keys
//	Space

//Then Mac-specific characters:
//	Left and Right Command?
//		Alternately, have the KeyCode for Control represent the character code for Command when on a Mac.
//	Mac Control Button

//Then gamepad/mouse constants:
//	Buttons 0-63. Some of these have aliases for an XBox controller:
//		Button 0: A Button
//		Button 1: B Button
//		Button 2: X Button
//		Button 3: Y Button
//		Button 4: Left Shoulder
//		Button 5: Right Shoulder
//		Button 6: Start Button
//		Button 7: Back Button
//		Button 8: Left Stick Press
//		Button 9: Right Stick Press
//		Button 10: XBox Button
//	DPads/POVs 0-8 (see how many actually need to be supported). Each has specific codes:
//		DPad X,0: DPad X Up
//		DPad X,1: DPad X Down
//		DPad X,2: DPad X Left
//		DPad X,3: DPad X Right
}