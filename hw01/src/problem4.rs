/// #[derive(...)] statements define certain properties on the enum for you for
/// free (printing, equality testing, the ability to copy values). More on this
/// when we cover Enums in detail.

/// You can use any of the variants of the `Peg` enum by writing `Peg::B`, etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg {
    A,
    B,
    C,
}

/// A move between two pegs: (source, destination).
pub type Move = (Peg, Peg);

/// Solves for the sequence of moves required to move all discs from `src` to
/// `dst`.
pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dst: Peg) -> Vec<Move> {
    if num_discs == 0 {
        return vec![];
    }
    let mut src_to_aux = hanoi(num_discs - 1, src, dst, aux);
    src_to_aux.push((src, dst));
    let mut aux_to_dst = hanoi(num_discs - 1, aux, src, dst);
    src_to_aux.append(&mut aux_to_dst);
    src_to_aux
}