#[inline(always)]
pub fn get_addr(opcode: u16) -> u16 {
  opcode & 0x0FFF
}

#[inline(always)]
pub fn get_vx(opcode: u16) -> usize {
  ((opcode & 0x0F00) >> 8) as usize
}

#[inline(always)]
pub fn get_vy(opcode: u16) -> usize {
  ((opcode & 0x00F0) >> 4) as usize
}
