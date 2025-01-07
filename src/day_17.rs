use std::{fmt::{self, Debug}, fs::read_to_string};

use nom::{
    bytes::complete::tag, character::complete::{
        self, digit1, line_ending, multispace1
    }, combinator::map_res, multi::separated_list1, sequence::{delimited, preceded, separated_pair}, IResult
};

const EXAMPLE_FILEPATH: &str = "./data/example_17.txt";

const INPUT_FILEPATH: &str = "./data/input_17.txt";

// main entry point for day 17
pub fn run(part: i16) {
    // part 1
    match part {
        1 => {
            let example_result: i64 = part_1(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_1(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        2 => {
            let example_result: i64 = part_2(EXAMPLE_FILEPATH);
            println!("Example result: {example_result}");

            let question_result: i64 = part_2(INPUT_FILEPATH);
            println!("Question result: {question_result}");
        }
        _ => {
            panic!("Invalid part specified for day 17")
        }
    }
}

fn part_1(file_path: &str) -> i64 {
    let file_str = read_to_string(file_path).expect("It reads the file");
    let (_input, (registers, instructions)) = parse(&file_str)
    .expect("It parsed the input");
    println!("{:?}", registers);
    println!("{:?}", instructions);
    
    0_i64
}

fn part_2(file_path: &str) -> i64 {
    let file_str = read_to_string(file_path).expect("It reads the file");
    let (_inout, (registers, instructions)) = parse(&file_str)
    .expect("It parsed the input");

    0_i64
}

struct Registers {
    a: i32,
    b: i32,
    c: i32,
    pointer: usize
}
impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {}, B: {}, C: {}, Pointer: {}", self.a, self.b, self.c, self.pointer)
    }
}

enum Instructions {
    // The adv instruction (opcode 0) performs division. 
    // The numerator is the value in the A register. 
    // The denominator is found by raising 2 to the power of the instruction's combo operand. 
    // (So, an operand of 2 would divide A by 4 (2^2); 
    // an operand of 5 would divide A by 2^B.) 
    // The result of the division operation is truncated to an 
    // integer and then written to the A register.
    Adv = 0,

    // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and 
    // the instruction's literal operand, then stores the result in register B.
    Bxl = 1,
    
    // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 
    // (thereby keeping only its lowest 3 bits), then writes that value to the B register.
    Bst = 2,

    // The jnz instruction (opcode 3) does nothing if the A register is 0. 
    // However, if the A register is not zero, it jumps by setting the instruction pointer 
    // to the value of its literal operand; if this instruction jumps, 
    // the instruction pointer is not increased by 2 after this instruction.
    Jnz = 3,

    // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, 
    // then stores the result in register B. 
    // (For legacy reasons, this instruction reads an operand but ignores it.)
    Bxc = 4,

    // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, 
    // then outputs that value. (If a program outputs multiple values, they are separated by commas.)
    Out = 5,

    // The bdv instruction (opcode 6) works exactly like the adv instruction 
    // except that the result is stored in the B register. (The numerator is still read from the A register.)
    Bdv = 6,

    // The cdv instruction (opcode 7) works exactly like the adv instruction 
    // except that the result is stored in the C register. (The numerator is still read from the A register.)
    Cdv = 7
}

// fn instructions(input: &str) -> IResult<&str, Vec<i32>> {
//     let (input, mchinstructions) = preceded(
//         tag("Program: "), 
//         separated_list1(tag(","), ),
//     );
//     Ok(
//         (
//             input,
//             mchinstructions
//         )

//     )

// }
fn registers(input: &str) -> IResult<&str, Registers> {
    let (input, a) = delimited(
        tag("Register A: "), 
        complete::i32, 
        line_ending,
    )(input)?;
    let (input, b) = delimited(
        tag("Register B: "), 
        complete::i32, 
        line_ending
    )(input)?;
    let (input, c) = delimited(
        tag("Register C: "), 
        complete::i32, 
        line_ending
    )(input)?;
    Ok(
        (input, 
        Registers {
            a,
            b,
            c,
            pointer: 0
        }),
    )
}

fn parse(input: &str,) -> IResult<&str, (Registers, Vec<i32>)> {
    let (input, registers) =
        // registers(input);
        separated_pair(
            registers,
            multispace1,
            preceded(
                tag("Program: "), 
                separated_list1(nom::character::complete::char(','), parse_number)
            )
        )
        (input)?;

    // let (input, _) =
    //     all_consuming(opt(line_ending))(input)?;

    Ok((input, registers))
}

// Parser to parse a single number
fn parse_number(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse::<i32>)(input)
}

///////////////////////////////////////////
//   Some but not all tests
///////////////////////////////////////////

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(0, part_1(EXAMPLE_FILEPATH));
    }

    #[test]
    fn test_part_2() {
        assert_eq!(0, part_2(EXAMPLE_FILEPATH));
    }

}
