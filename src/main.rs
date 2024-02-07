#![allow(unused)]
use std::{fs, ops::Range};

type Position = usize;
type PositionRange = Range<Position>;
type Item = char;
type ReturnCode = Result<(), ()>;

#[derive(Debug)]
struct Piece {
    start: usize,
    length: usize,
    is_original: bool,
}

#[derive(Debug)]
struct Sequence {
    original: &'static str,
    add: String,
    pieces: Vec<Piece>,
}

impl Sequence {
    pub fn new(original: &'static str) -> Self {
        Self {
            original,
            add: String::new(),
            pieces: vec![Piece {
                start: 0,
                length: original.len(),
                is_original: true,
            }],
        }
    }

    pub fn insert(&mut self, at: usize, s: &str) -> ReturnCode {
        let mut pieces = Vec::new();

        if at > 0 {
            pieces.push(Piece {
                start: 0,
                length: at,
                is_original: true,
            });
        }

        pieces.push(Piece {
            start: 0,
            length: s.len(),
            is_original: false,
        });

        if at < self.original.len() {
            pieces.push(Piece {
                start: at,
                length: self.original.len() - at,
                is_original: true,
            });
        }

        self.pieces = pieces;

        Ok(())
    }

    pub fn delete(&mut self, pos: usize, length: usize) -> ReturnCode {
        let mut pieces = Vec::new();

        if pos > 0 {
            pieces.push(Piece {
                start: 0,
                length: pos,
                is_original: true,
            });
        }

        if pos + length < self.original.len() {
            pieces.push(Piece {
                start: pos + length,
                length: self.original.len() - (pos + length),
                is_original: true,
            });
        }

        self.pieces = pieces;

        Ok(())
    }

    pub fn close(&self) -> ReturnCode {
        todo!()
    }

    pub fn copy(&self, from: PositionRange, to: Position) -> ReturnCode {
        todo!()
    }

    pub fn move_(&self, from: PositionRange, to: Position) -> ReturnCode {
        todo!()
    }

    pub fn replace(&self, from: PositionRange, with: Sequence) -> ReturnCode {
        todo!()
    }

    pub fn sequence_at(&self, pos: PositionRange) -> &Sequence {
        todo!()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_data() {
        let seq = Sequence::new("Hello, World!");
        assert_eq!(seq.original, "Hello, World!");

        let pieces = seq.pieces;
        assert_eq!(pieces.len(), 1);
        assert_eq!(pieces[0].start, 0);
        assert_eq!(pieces[0].length, 13);
    }

    #[test]
    fn test_delete_middle() {
        let mut sequence = Sequence::new("A large span of text");
        sequence.delete(2, 5);

        let pieces = sequence.pieces;
        assert_eq!(pieces.len(), 2);

        assert_eq!(pieces[0].start, 0);
        assert_eq!(pieces[0].length, 2);
        assert_eq!(pieces[0].is_original, true);

        assert_eq!(pieces[1].start, 8);
        assert_eq!(pieces[1].length, 12);
        assert_eq!(pieces[1].is_original, true);
    }

    #[test]
    fn test_delete_beginning() {
        let mut sequence = Sequence::new("A large span of text");
        sequence.delete(0, 8);

        let pieces = sequence.pieces;
        println!("{:?}", pieces);
        assert_eq!(pieces.len(), 1);

        assert_eq!(pieces[0].start, 9);
        assert_eq!(pieces[0].length, 11);
        assert_eq!(pieces[0].is_original, true);
    }

    #[test]
    fn test_delete_end() {
        let mut sequence = Sequence::new("A large span of text");
        sequence.delete(9, 11);

        let pieces = sequence.pieces;
        assert_eq!(pieces.len(), 1);

        assert_eq!(pieces[0].start, 0);
        assert_eq!(pieces[0].length, 9);
        assert_eq!(pieces[0].is_original, true);
    }

    #[test]
    fn test_insert() {
        let mut sequence = Sequence::new("A large span of text");
        sequence.insert(16, "English ");

        let pieces = sequence.pieces;
        assert_eq!(pieces.len(), 3);

        assert_eq!(pieces[0].start, 0);
        assert_eq!(pieces[0].length, 16);
        assert_eq!(pieces[0].is_original, true);

        assert_eq!(pieces[1].start, 0);
        assert_eq!(pieces[1].length, 8);
        assert_eq!(pieces[1].is_original, false);

        assert_eq!(pieces[2].start, 16);
        assert_eq!(pieces[2].length, 4);
        assert_eq!(pieces[2].is_original, true);
    }
}
