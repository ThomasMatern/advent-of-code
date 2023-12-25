#![allow(unused_variables, unused_imports, dead_code)]

use nom::{
    IResult,
    Parser,
    character::complete::{ self, line_ending },
    multi::separated_list1,
    sequence::{ separated_pair, tuple },
};
use nom_supreme::{ tag::complete::tag, ParserExt };
use glam::I64Vec3;

use z3::{ Config, Context, Solver, ast::{ Ast, Int } };

fn main() {
    let input = include_str!("./input-1.txt");
    let output = process(input);
    dbg!(output);
}

struct HailStone {
    location: I64Vec3,
    direction: I64Vec3,
}

fn parse(i: &str) -> IResult<&str, Vec<HailStone>> {
    separated_list1(
        line_ending,
        separated_pair(
            tuple((
                complete::i64.terminated(tag(", ")),
                complete::i64.terminated(tag(", ")),
                complete::i64,
            )).map(|(x, y, z)| I64Vec3::new(x, y, z)),
            tag(" @ "),
            tuple((
                complete::i64.terminated(tag(", ")),
                complete::i64.terminated(tag(", ")),
                complete::i64,
            )).map(|(x, y, z)| I64Vec3::new(x, y, z))
        ).map(|(location, direction)| HailStone { location, direction })
    ).parse(i)
}

pub fn process(input: &str) -> String {
    let (_, hailstones) = parse(input).unwrap();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let dx = Int::new_const(&ctx, "dx");
    let dy = Int::new_const(&ctx, "dy");
    let dz = Int::new_const(&ctx, "dz");

    let zero = Int::from_i64(&ctx, 0);

    for (i, hs) in hailstones.iter().enumerate() {
        let ti = Int::new_const(&ctx, format!("t{i}"));
        solver.assert(&(&x - hs.location.x + (&dx - hs.direction.x) * &ti)._eq(&zero));
        solver.assert(&(&y - hs.location.y + (&dy - hs.direction.y) * &ti)._eq(&zero));
        solver.assert(&(&z - hs.location.z + (&dz - hs.direction.z) * &ti)._eq(&zero));
    }
    if solver.check() != z3::SatResult::Sat {
        panic!("Not satisfiable!");
    }

    let result = solver
        .get_model()
        .and_then(|model| model.eval(&(x + y + z), true))
        .unwrap()
        .to_string();
    result
}

#[cfg(test)]
mod tests {
    use super::process;

    #[test]
    fn test_process() {
        let input =
            "\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!("47", process(input));
    }
}
