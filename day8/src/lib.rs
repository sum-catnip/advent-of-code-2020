#![feature(str_split_once)]
#![feature(bool_to_option)]
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_d8_p1_example() {
        let data: Vec<String> = input::file_lines("../data/day8-1-test.txt").collect();
        let prog = parse_prog(data.iter().map(AsRef::as_ref).collect());
        let (finished, acc) = find_loop(prog);
        assert!(!finished);
        assert_eq!(5, acc);
    }

    #[test]
    fn test_d8_p1() {
        let data: Vec<String> = input::file_lines("../data/day8-1.txt").collect();
        let prog = parse_prog(data.iter().map(AsRef::as_ref).collect());
        let (finished, acc) = find_loop(prog);
        assert!(!finished);
        assert_eq!(1420, acc);
    }

    #[test]
    fn test_d8_p2_example() {
        let data: Vec<String> = input::file_lines("../data/day8-1-test.txt").collect();
        let prog = parse_prog(data.iter().map(AsRef::as_ref).collect());
        assert_eq!(8, find_fix(prog));
    }

    #[test]
    fn test_d8_p2() {
        let data: Vec<String> = input::file_lines("../data/day8-1.txt").collect();
        let prog = parse_prog(data.iter().map(AsRef::as_ref).collect());
        assert_eq!(1245, find_fix(prog));
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Inst { Acc(i64), Jmp(i64), Nop(i64) }
pub type Program = Vec<Inst>;

struct Vm<'a> { prog: &'a Program, ip: usize, acc: i64 }
impl<'a> Vm<'a> {
    fn inst_acc(&mut self, v: i64) -> usize {
        self.acc += v;
        self.ip += 1;
        self.ip
    }

    fn inst_jmp(&mut self, v: i64) -> usize {
        self.ip = (self.ip as i64 + v) as usize;
        self.ip
    }

    fn inst_nop(&mut self) -> usize { self.ip += 1; self.ip }

    pub fn new(prog: &'a Program) -> Vm {
        Vm { prog, ip: 0, acc: 0 }
    }

    pub fn acc(&self) -> i64   { self.acc }

    pub fn next(&mut self) -> Option<usize> {
        match self.prog.get(self.ip) {
            Some(Inst::Acc(v)) => Some(self.inst_acc(*v)),
            Some(Inst::Jmp(v)) => Some(self.inst_jmp(*v)),
            Some(Inst::Nop(_)) => Some(self.inst_nop()),
            None => None
        }
    }
}

fn parse_inst(line: &str) -> Inst {
    let (op, val) = line.split_once(' ').expect("invalid format");
    match op {
        "nop" => Inst::Nop(val.parse().expect("invalid nop val")),
        "acc" => Inst::Acc(val.parse().expect("invalid acc val")),
        "jmp" => Inst::Jmp(val.parse().expect("invalid jmp val")),
        _ => panic!(format!("invalid opcode: {}", op))
    }
}

pub fn parse_prog(lines: Vec<&str>) -> Program {
    lines.iter().map(|l| parse_inst(l)).collect()
}

pub fn find_loop(prog: Program) -> (bool, i64) {
    let mut vm = Vm::new(&prog);
    let mut ips = HashSet::new();
    
    while ips.insert(vm.next()) {}
    (ips.contains(&None), vm.acc())
}

pub fn find_fix(prog: Program) -> i64 {
    prog.iter().enumerate().find_map(|(i, inst)| {
        let mut new = prog.clone();
        new[i] = match *inst {
            Inst::Nop(v) => Inst::Jmp(v),
            Inst::Jmp(v) => Inst::Nop(v),
            x => x
        };
        let (finished, acc) = find_loop(new);
        finished.then_some(acc)
    }).unwrap()
}
