use std::fs;

enum Ins {
    Addx(i32),
    Noop,
}

struct CPU {
    cycles: i32,
    x: i32,
    sum: i32,

    row: Vec<bool>,
    crt: Vec<Vec<bool>>,
}

impl CPU {
    fn cycle(&mut self) {
        let pos = (self.cycles - 1) % 40;
        self.row.push((pos - self.x).abs() < 2);
        if self.row.len() == 40 {
            let row = std::mem::replace(&mut self.row, Vec::with_capacity(40));
            self.crt.push(row);
        }

        if (self.cycles - 20) % 40 == 0 {
            self.sum += self.cycles * self.x;
        }
        self.cycles += 1;
    }

    fn run_ins(&mut self, ins: &str) {
        let ins = if let Some(num) = ins.strip_prefix("addx ") {
            let num: i32 = str::parse(num).unwrap();
            Ins::Addx(num)
        } else if ins == "noop" {
            Ins::Noop
        } else {
            panic!()
        };
        let ins_cycles = match ins {
            Ins::Addx(_) => 2,
            Ins::Noop => 1,
        };
        for _ in 0..ins_cycles {
            self.cycle();
        }
        if let Ins::Addx(num) = ins {
            self.x += num;
        }
    }

}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    
    let mut cpu = CPU {
        cycles: 1,
        x: 1,
        sum: 0,
        row: Vec::with_capacity(40),
        crt: Vec::with_capacity(6),
    };

    for line in input.lines() {
        cpu.run_ins(line);
    }

    for row in cpu.crt {
        for pixel in row {
            print!("{}", if pixel { "#" } else { "." });
        }
        println!();
    }

    dbg!(cpu.x, cpu.sum);
}
