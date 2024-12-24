use clap::Parser;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    data_file: String,
    #[arg(long)]
    debug: bool,
}

#[derive(Debug, Clone, Hash)]
enum Gate {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone, Hash)]
enum WireOrOperation {
    Wire(String),
    Operation(OperationRec),
}

impl WireOrOperation {
    fn calculate(&self, values: &HashMap<String, i64>) -> Option<i64> {
        match self {
            WireOrOperation::Wire(wire) => values.get(wire).copied(),
            WireOrOperation::Operation(operation) => operation.calculate(values),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct OperationRec {
    left: Rc<WireOrOperation>,
    right: Rc<WireOrOperation>,
    target: String,
    gate: Gate,
}

impl ToString for WireOrOperation {
    fn to_string(&self) -> String {
        match self {
            WireOrOperation::Wire(wire) => wire.clone(),
            WireOrOperation::Operation(operation) => operation.to_string(),
        }
    }
}

impl ToString for OperationRec {
    fn to_string(&self) -> String {
        format!(
            "({} {:?} {})",
            self.left.to_string(),
            self.gate,
            self.right.to_string()
        )
    }
}

impl OperationRec {
    fn calculate(&self, values: &HashMap<String, i64>) -> Option<i64> {
        Some(match self.gate {
            Gate::AND => self.left.calculate(values)? & self.right.calculate(values)?,
            Gate::OR => self.left.calculate(values)? | self.right.calculate(values)?,
            Gate::XOR => self.left.calculate(values)? ^ self.right.calculate(values)?,
        })
    }
}

#[derive(Debug, Clone, Hash)]
struct Operation {
    left: String,
    right: String,
    target: String,
    gate: Gate,
}

#[derive(Debug, Clone)]
struct Input {
    values: HashMap<String, i64>,
    operations: Vec<Operation>,
}

fn main() {
    let args = Args::parse();
    let data_file = if args.data_file.is_empty() {
        format!("{}/src/data.txt", env!("CARGO_MANIFEST_DIR"))
    } else {
        args.data_file
    };

    let input = parse(&data_file);

    let result1 = part1(&input);
    println!("Part1: {}", result1);

    println!("Part 2: {}", part2_better(&input));
    // println!("Part 2: {}", part2(&input))

    // FINE, let's do it by hand
    /*
    CORRECT z00 = (x00 XOR y00)
    CORRECT z01 = ((y00 AND x00) XOR (x01 XOR y01))
    CORRECT z02 = (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) XOR (y02 XOR x02))
    CORRECT z03 = ((y03 XOR x03) XOR ((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))))
    CORRECT z04 = ((y04 XOR x04) XOR ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))

    First wrong is at z12 so somewhere the carry adder is wrong here
    We should have (X12 XOR Y12) XOR (CARRY FROM Z11)
    We have y12 xor x12 so we've got a mistake in the carry part
    djg is z12
    dsd is z37
    sbg is z19


    WRONG----- z12 = ((y12 XOR x12) AND ((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)))
    Wires: ["cgt", "csm", "dcc", "dps", "dts", "fbk", "fhf", "fjd", "fkc", "ftg", "gpv", "gvt", "hcd", "hww", "jjc", "jmr", "jsb", "kdm", "kfb", "kgp", "kmb", "ktt", "kwm", "mhv", "mpf", "mtm", "njf", "njj", "ntj", "pmb", "pvb", "qss", "rdt", "rkn", "rqp", "rvb", "sch", "sqt", "ssq", "trw", "vsc", "whm", "wkn", "wnv", "wqw", "wvw", "z12"]
    WRONG----- z13 = ((x13 XOR y13) XOR ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12)))
    Wires: ["cgt", "csm", "dcc", "djg", "djr", "dps", "dts", "fbk", "fhf", "fjd", "fkc", "fnc", "ftg", "gpv", "gvt", "hcd", "hww", "jjc", "jmr", "jsb", "kdm", "kfb", "kgp", "kmb", "ktt", "kwm", "mhv", "mpf", "mtm", "nbf", "njf", "njj", "ntj", "pmb", "pvb", "qss", "rdt", "rkn", "rqp", "rvb", "sch", "sqt", "ssq", "trw", "vsc", "whm", "wkn", "wnv", "wqw", "wvw", "z13"]
    WRONG----- z19 = (x19 AND y19)
    WRONG----- z20 = ((x20 XOR y20) XOR ((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))))
    WRONG----- z21 = ((x21 XOR y21) XOR ((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)))
    WRONG----- z22 = (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) XOR (y22 XOR x22))
    WRONG----- z24 = (((y23 AND x23) OR (((x22 AND y22) OR (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) AND (y22 XOR x22))) AND (y23 XOR x23))) XOR (y24 AND x24))
    WRONG----- z37 = ((((y36 AND x36) OR (((y35 AND x35) OR ((x35 XOR y35) AND (((y34 XOR x34) AND ((y33 AND x33) OR ((y33 XOR x33) AND (((((y31 XOR x31) AND ((y30 AND x30) OR (((x29 AND y29) OR (((x28 AND y28) OR ((y28 XOR x28) AND (((y27 XOR x27) AND ((((y25 AND x25) OR (((x24 XOR y24) OR ((y24 AND x24) AND ((y23 AND x23) OR (((x22 AND y22) OR (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) AND (y22 XOR x22))) AND (y23 XOR x23))))) AND (y25 XOR x25))) AND (y26 XOR x26)) OR (x26 AND y26))) OR (x27 AND y27)))) AND (x29 XOR y29))) AND (y30 XOR x30)))) OR (x31 AND y31)) AND (y32 XOR x32)) OR (y32 AND x32))))) OR (y34 AND x34)))) AND (y36 XOR x36))) AND (y37 XOR x37)) OR (y37 AND x37))
    WRONG----- z38 = ((((y36 AND x36) OR (((y35 AND x35) OR ((x35 XOR y35) AND (((y34 XOR x34) AND ((y33 AND x33) OR ((y33 XOR x33) AND (((((y31 XOR x31) AND ((y30 AND x30) OR (((x29 AND y29) OR (((x28 AND y28) OR ((y28 XOR x28) AND (((y27 XOR x27) AND ((((y25 AND x25) OR (((x24 XOR y24) OR ((y24 AND x24) AND ((y23 AND x23) OR (((x22 AND y22) OR (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) AND (y22 XOR x22))) AND (y23 XOR x23))))) AND (y25 XOR x25))) AND (y26 XOR x26)) OR (x26 AND y26))) OR (x27 AND y27)))) AND (x29 XOR y29))) AND (y30 XOR x30)))) OR (x31 AND y31)) AND (y32 XOR x32)) OR (y32 AND x32))))) OR (y34 AND x34)))) AND (y36 XOR x36))) XOR (y37 XOR x37)) XOR (y38 XOR x38))
    WRONG----- z39 = ((((((y36 AND x36) OR (((y35 AND x35) OR ((x35 XOR y35) AND (((y34 XOR x34) AND ((y33 AND x33) OR ((y33 XOR x33) AND (((((y31 XOR x31) AND ((y30 AND x30) OR (((x29 AND y29) OR (((x28 AND y28) OR ((y28 XOR x28) AND (((y27 XOR x27) AND ((((y25 AND x25) OR (((x24 XOR y24) OR ((y24 AND x24) AND ((y23 AND x23) OR (((x22 AND y22) OR (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) AND (y22 XOR x22))) AND (y23 XOR x23))))) AND (y25 XOR x25))) AND (y26 XOR x26)) OR (x26 AND y26))) OR (x27 AND y27)))) AND (x29 XOR y29))) AND (y30 XOR x30)))) OR (x31 AND y31)) AND (y32 XOR x32)) OR (y32 AND x32))))) OR (y34 AND x34)))) AND (y36 XOR x36))) XOR (y37 XOR x37)) AND (y38 XOR x38)) OR (y38 AND x38)) XOR (y39 XOR x39))
    WRONG----- z40 = ((y40 XOR x40) XOR (((((((y36 AND x36) OR (((y35 AND x35) OR ((x35 XOR y35) AND (((y34 XOR x34) AND ((y33 AND x33) OR ((y33 XOR x33) AND (((((y31 XOR x31) AND ((y30 AND x30) OR (((x29 AND y29) OR (((x28 AND y28) OR ((y28 XOR x28) AND (((y27 XOR x27) AND ((((y25 AND x25) OR (((x24 XOR y24) OR ((y24 AND x24) AND ((y23 AND x23) OR (((x22 AND y22) OR (((y21 AND x21) OR (((((((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) AND (x19 XOR y19)) OR (((x18 AND y18) OR (((x17 AND y17) OR ((((((((x14 XOR y14) AND ((x13 AND y13) OR ((x13 XOR y13) AND ((((((((x09 AND y09) OR (((y08 AND x08) OR ((y08 XOR x08) AND (((y07 XOR x07) AND ((x06 AND y06) OR (((y05 AND x05) OR ((y05 XOR x05) AND ((x04 AND y04) OR ((y04 XOR x04) AND ((x03 AND y03) OR (((y02 AND x02) OR (((x01 AND y01) OR ((y00 AND x00) AND (x01 XOR y01))) AND (y02 XOR x02))) AND (y03 XOR x03))))))) AND (y06 XOR x06)))) OR (x07 AND y07)))) AND (x09 XOR y09))) AND (x10 XOR y10)) OR (y10 AND x10)) AND (y11 XOR x11)) OR (y11 AND x11)) XOR (y12 XOR x12)) OR (y12 AND x12))))) OR (x14 AND y14)) AND (y15 XOR x15)) OR (y15 AND x15)) AND (x16 XOR y16)) OR (x16 AND y16)) AND (y17 XOR x17))) AND (y18 XOR x18))) XOR (x19 XOR y19))) AND (x20 XOR y20)) OR (y20 AND x20)) AND (x21 XOR y21))) AND (y22 XOR x22))) AND (y23 XOR x23))))) AND (y25 XOR x25))) AND (y26 XOR x26)) OR (x26 AND y26))) OR (x27 AND y27)))) AND (x29 XOR y29))) AND (y30 XOR x30)))) OR (x31 AND y31)) AND (y32 XOR x32)) OR (y32 AND x32))))) OR (y34 AND x34)))) AND (y36 XOR x36))) XOR (y37 XOR x37)) AND (y38 XOR x38)) OR (y38 AND x38)) AND (y39 XOR x39)) OR (x39 AND y39)))

    */
}

fn part1(input: &Input) -> i64 {
    let mut values = input.values.clone();

    let mut all_zs = HashSet::new();
    for (k, _v) in &values {
        if k.starts_with("z") {
            all_zs.insert(k.clone());
        }
    }

    for op in &input.operations {
        if op.target.starts_with("z") {
            all_zs.insert(op.target.clone());
        }
    }

    loop {
        let mut change_happened = false;
        for i in 0..input.operations.len() {
            let operation = &input.operations[i];

            if values.contains_key(&operation.left)
                && values.contains_key(&operation.right)
                && !values.contains_key(&operation.target)
            {
                change_happened = true;
                values.insert(
                    operation.target.clone(),
                    match operation.gate {
                        Gate::OR => values[&operation.left] | values[&operation.right],
                        Gate::XOR => values[&operation.left] ^ values[&operation.right],
                        Gate::AND => values[&operation.left] & values[&operation.right],
                    },
                );
            }
        }

        if !change_happened {
            return -1;
        }

        let mut all_fulfilled = true;
        let mut result = 0;
        for k in all_zs.iter().sorted().rev() {
            if !values.contains_key(k) {
                all_fulfilled = false;
                break;
            } else {
                result = (result << 1) + values[k];
            }
        }

        if all_fulfilled {
            return result;
        }
    }
}

fn part2_better(input: &Input) -> String {
    let mut initial_x = 0;
    let mut initial_y = 0;

    for (k, v) in input.values.iter().sorted().rev() {
        if k.starts_with("x") {
            initial_x = (initial_x << 1) + v;
        } else if k.starts_with("y") {
            initial_y = (initial_y << 1) + v;
        }
    }

    let expected_z = initial_x + initial_y;

    // Let's find the formula for each Z output.
    let mut all_zs = HashSet::new();
    for op in &input.operations {
        if op.target.starts_with("z") {
            all_zs.insert(op.target.clone());
        }
    }
    let all_zs = all_zs
        .iter()
        .sorted()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut cache = HashMap::new();
    let standard_zs = (0..all_zs.len())
        .map(|z| find_formula(&all_zs[z], &input.operations, &mut cache, &HashSet::new()))
        .collect_vec();

    for i in 0..input.operations.len() {
        let op = &input.operations[i];
        let formula = find_formula(&op.target, &input.operations, &mut cache, &HashSet::new());
        println!("{}: {}", op.target, formula.to_string());
    }

    standard_zs.iter().for_each(|z| {
        let WireOrOperation::Operation(op) = z.as_ref() else {
            panic!()
        };
        let correct = is_correct(
            z,
            &input.values,
            (expected_z >> op.target[1..].parse::<usize>().unwrap()) & 1,
        );
        println!(
            "{} {} = {}",
            if correct { "CORRECT" } else { "WRONG-----" },
            op.target,
            z.to_string()
        );
        if !correct {
            println!("Wires: {:?}", find_targets(z).iter().sorted().collect_vec())
        }
    });

    let used_wires: HashSet<String> = standard_zs.iter().flat_map(|z| find_targets(z)).collect();
    let all_wires: HashSet<String> = input
        .operations
        .iter()
        .map(|op| op.target.clone())
        .collect();

    let wrong_zs = standard_zs
        .iter()
        .filter(|z| {
            let WireOrOperation::Operation(op) = z.as_ref() else {
                panic!()
            };
            println!("{:?}", op.target);
            !is_correct(
                z,
                &input.values,
                (expected_z >> op.target[1..].parse::<usize>().unwrap()) & 1,
            )
        })
        .collect_vec();

    let possible_wires: HashSet<String> = wrong_zs
        .iter()
        .flat_map(|z| find_targets(z))
        .chain(all_wires.difference(&used_wires).map(|w| w.clone()))
        .collect();

    let possible_ops = input
        .operations
        .iter()
        .filter(|op| possible_wires.contains(&op.target))
        .cloned()
        .collect_vec();

    let wrong_sets = standard_zs
        .iter()
        .filter(|z| {
            let WireOrOperation::Operation(op) = z.as_ref() else {
                panic!()
            };
            println!("{:?}", op.target);
            !is_correct(
                z,
                &input.values,
                (expected_z >> op.target[1..].parse::<usize>().unwrap()) & 1,
            )
        })
        .map(|z| HashSet::from_iter(find_targets(z).into_iter()))
        .collect_vec();

    println!("{:?}", wrong_sets);

    // There are 8 (4 pairs) wires to change, let's just get all combinations and then combinations of combinations.
    for a in 0..possible_ops.len() {
        for b in a..possible_ops.len() {
            if b == a {
                continue;
            }
            for _c in 0..1 {
                let c = possible_ops
                    .iter()
                    .find_position(|o| o.target == "z19")
                    .unwrap()
                    .0;
                if c == b || c == a {
                    continue;
                }
                for _d in 0..1 {
                    let d = possible_ops
                        .iter()
                        .find_position(|o| o.target == "sbg")
                        .unwrap()
                        .0;
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for _e in 0..1 {
                        let e = possible_ops
                            .iter()
                            .find_position(|o| o.target == "z37")
                            .unwrap()
                            .0;
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        for _f in 0..1 {
                            let f = possible_ops
                                .iter()
                                .find_position(|o| o.target == "dsd")
                                .unwrap()
                                .0;
                            if f == e || f == d || f == c || f == b || f == a {
                                continue;
                            }
                            for _g in 0..1 {
                                let g = possible_ops
                                    .iter()
                                    .find_position(|o| o.target == "z12")
                                    .unwrap()
                                    .0;
                                if g == f || g == e || g == d || g == c || g == b || g == a {
                                    continue;
                                }
                                'h: for _h in 0..1 {
                                    let h = possible_ops
                                        .iter()
                                        .find_position(|o| o.target == "djg")
                                        .unwrap()
                                        .0;
                                    if h == g
                                        || h == f
                                        || h == e
                                        || h == d
                                        || h == c
                                        || h == b
                                        || h == a
                                    {
                                        continue;
                                    }

                                    let pairs = vec![(a, b), (c, d), (e, f), (g, h)];

                                    let mut modified_wires = HashSet::new();
                                    let mut operations = possible_ops.clone();
                                    for p in &pairs {
                                        let a = p.0;
                                        let b = p.1;
                                        let a_target = operations[a].target.clone();
                                        let b_target = operations[b].target.clone();
                                        operations[a].target = b_target.clone();
                                        operations[b].target = a_target.clone();

                                        modified_wires.insert(a_target);
                                        modified_wires.insert(b_target);
                                    }

                                    // There must be at least one wire in each wrong Z
                                    for s in &wrong_sets {
                                        if s.intersection(&modified_wires).count() == 0 {
                                            continue 'h;
                                        }
                                    }

                                    println!("{:?}", pairs);
                                    println!("{:?}", modified_wires);

                                    let mut cache = HashMap::new();
                                    let mut any_wires_modified = false;

                                    for z in 0..wrong_zs.len() {
                                        let WireOrOperation::Operation(z_op) = wrong_zs[z].as_ref()
                                        else {
                                            panic!()
                                        };
                                        let formula = find_formula(
                                            &z_op.target,
                                            &operations,
                                            &mut cache,
                                            &HashSet::new(),
                                        );
                                        if !contains_targets(&formula, &modified_wires) {
                                            continue;
                                        }
                                        any_wires_modified = true;
                                        if !is_correct(
                                            formula.as_ref(),
                                            &input.values,
                                            (expected_z >> z) & 1,
                                        ) {
                                            continue 'h;
                                        }

                                        //println!("{} = {}", z, formula.to_string());
                                    }

                                    if any_wires_modified {
                                        // NOTE: You will need to modify the inputs at this point to ensure that the answer is correct or else you may get a false positive
                                        println!("Answer = {:?}", pairs);
                                        return format!(
                                            "{:?}",
                                            modified_wires.iter().sorted().join(",")
                                        );
                                    } else {
                                        println!("No wires modified???");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    return "".to_string();
    // let pairs = (0..input.operations.len())
    //     .combinations(2)
    //     .combinations(4)
    //     .filter(|pairs| pairs.iter().flatten().sorted().unique().count() == 8)
    //     .find(|pairs| {
    //         println!("{:?}", pairs);
    //         let mut modified_wires = HashSet::new();
    //         let mut operations = input.operations.clone();
    //         for p in pairs {
    //             let a = p[0];
    //             let b = p[1];
    //             let a_target = operations[a].target.clone();
    //             let b_target = operations[b].target.clone();
    //             operations[a].target = b_target.clone();
    //             operations[b].target = a_target.clone();

    //             modified_wires.insert(a_target);
    //             modified_wires.insert(b_target);
    //         }

    //         let mut cache = HashMap::new();

    //         for z in 0..all_zs.len() {
    //             let standard_formula = standard_zs[z].clone();
    //             if !contains_targets(standard_formula.as_ref(), &modified_wires) {
    //                 continue;
    //             }
    //             let formula = find_formula(&all_zs[z], &input.operations, &mut cache);
    //             if !is_correct(
    //                 formula.as_ref(),
    //                 &input.values,
    //                 *input.values.get(&format!("x{:02}", z)).unwrap()
    //                     + *input.values.get(&format!("y{:02}", z)).unwrap(),
    //             ) {
    //                 return false;
    //             }

    //             println!("{} = {}", z, formula.to_string());
    //         }

    //         return true;
    //     })
    //     .unwrap();
}

fn find_targets(wire_or_op: &WireOrOperation) -> Vec<String> {
    match wire_or_op {
        WireOrOperation::Wire(_) => vec![],
        WireOrOperation::Operation(op) => find_targets(&op.left)
            .into_iter()
            .chain(find_targets(&op.right))
            .chain(vec![op.target.clone()].into_iter())
            .collect_vec(),
    }
}

fn contains_targets(wire_or_op: &WireOrOperation, modified_wires: &HashSet<String>) -> bool {
    match wire_or_op {
        WireOrOperation::Wire(w) => modified_wires.contains(w),
        WireOrOperation::Operation(op) => {
            modified_wires.contains(&op.target)
                || contains_targets(&op.left, modified_wires)
                || contains_targets(&op.right, modified_wires)
        }
    }
}

fn is_correct(operation: &WireOrOperation, values: &HashMap<String, i64>, expected: i64) -> bool {
    operation.calculate(values).is_some_and(|v| v == expected)
}

fn find_formula(
    wire: &str,
    operations: &Vec<Operation>,
    cache: &mut HashMap<String, Rc<WireOrOperation>>,
    seen: &HashSet<String>,
) -> Rc<WireOrOperation> {
    if cache.contains_key(wire) {
        return cache[wire].clone();
    }

    if seen.contains(wire) {
        return Rc::new(WireOrOperation::Wire(wire.to_string()));
    }

    let mut seen = seen.clone();
    seen.insert(wire.to_string());

    let operation = operations.iter().find(|op| op.target == wire);

    if operation.is_none() {
        cache.insert(
            wire.to_string(),
            Rc::new(WireOrOperation::Wire(wire.to_string())),
        );
        return cache[wire].clone();
    }
    let operation = operation.unwrap();

    let formula = WireOrOperation::Operation(OperationRec {
        left: find_formula(&operation.left, operations, cache, &seen),
        gate: operation.gate.clone(),
        right: find_formula(&operation.right, operations, cache, &seen),
        target: operation.target.clone(),
    });
    cache.insert(wire.to_string(), Rc::new(formula));
    return cache[wire].clone();
}

fn _part2(input: &Input) -> String {
    let mut initial_x = 0;
    let mut initial_y = 0;

    for (k, v) in input.values.iter().sorted().rev() {
        if k.starts_with("x") {
            initial_x = (initial_x << 1) + v;
        } else if k.starts_with("y") {
            initial_y = (initial_y << 1) + v;
        }
    }

    let expected_z = initial_x + initial_y;

    println!("Initial X: {}, Initial Y: {}", initial_x, initial_y);
    println!("Expected Z: {}", expected_z);

    let result = (0..input.operations.len())
        .combinations(8)
        .find(|c| {
            let result = c.iter().combinations(2).find_map(|p| {
                Some((
                    c.iter()
                        .filter(|i| **i != *p[0] && **i != *p[1])
                        .combinations(2)
                        .find_map(|p2| {
                            Some((
                                c.iter()
                                    .filter(|i| {
                                        **i != *p[0]
                                            && **i != *p[1]
                                            && **i != *p2[0]
                                            && **i != *p2[1]
                                    })
                                    .combinations(2)
                                    .find_map(|p3| {
                                        let mut input = input.clone();
                                        let old = input.operations[*p[1]].target.clone();
                                        input.operations[*p[1]].target =
                                            input.operations[*p[0]].target.clone();
                                        input.operations[*p[0]].target = old;

                                        let old = input.operations[*p2[1]].target.clone();
                                        input.operations[*p2[1]].target =
                                            input.operations[*p2[0]].target.clone();
                                        input.operations[*p2[0]].target = old;

                                        let old = input.operations[*p3[1]].target.clone();
                                        input.operations[*p3[1]].target =
                                            input.operations[*p3[0]].target.clone();
                                        input.operations[*p3[0]].target = old;

                                        let p4 = c
                                            .iter()
                                            .filter(|i| {
                                                **i != *p[0]
                                                    && **i != *p[1]
                                                    && **i != *p2[0]
                                                    && **i != *p2[1]
                                                    && **i != *p3[0]
                                                    && **i != *p3[1]
                                            })
                                            .collect_vec();
                                        let old = input.operations[*p4[1]].target.clone();
                                        input.operations[*p4[1]].target =
                                            input.operations[*p4[0]].target.clone();
                                        input.operations[*p4[0]].target = old;

                                        let part1_result = part1(&input);
                                        // println!("{}", part1_result);
                                        if part1_result == expected_z {
                                            return Some(p3);
                                        }

                                        None
                                    })?,
                                p2.clone(),
                            ))
                        })?,
                    p.clone(),
                ))
            });
            println!("{:?}", result);
            // println!("Trying: {:?}", c);
            // let old0 = input.operations.get(c[0]).unwrap().target.clone();
            // let old1 = input.operations.get(c[1]).unwrap().target.clone();
            // let old2 = input.operations.get(c[2]).unwrap().target.clone();
            // let old3 = input.operations.get(c[3]).unwrap().target.clone();

            // // Need to try swapping 0 and 1, 0 and 2, and 0 and 4
            // let mut input01 = input.clone();
            // input01.operations[c[0]].target = old1.clone();
            // input01.operations[c[1]].target = old0.clone();
            // input01.operations[c[2]].target = old3.clone();
            // input01.operations[c[3]].target = old2.clone();
            // let part1_result = part1(&input01);
            // println!("0123: {}", part1_result);
            // if part1_result == expected_z {
            //     return true;
            // }

            // let mut input02 = input.clone();
            // input02.operations[c[0]].target = old2.clone();
            // input02.operations[c[2]].target = old0.clone();
            // input02.operations[c[3]].target = old1.clone();
            // input02.operations[c[1]].target = old3.clone();
            // let part1_result = part1(&input02);
            // println!("0231: {}", part1_result);
            // if part1_result == expected_z {
            //     return true;
            // }

            // let mut input03 = input.clone();
            // input03.operations[c[0]].target = old3.clone();
            // input03.operations[c[3]].target = old0.clone();
            // input03.operations[c[2]].target = old1.clone();
            // input03.operations[c[1]].target = old2.clone();
            // let part1_result = part1(&input03);
            // println!("0321: {}", part1_result);
            // if part1_result == expected_z {
            //     return true;
            // }

            false
        })
        .unwrap();

    let mut results = Vec::new();
    for i in result {
        results.push(&input.operations.get(i).unwrap().target);
    }
    results.iter().sorted().join(",")
}

fn parse(file: &str) -> Input {
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();

    let values = lines
        .iter()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (gate, value) = l.split_once(": ").unwrap();
            (gate.to_string(), value.parse::<i64>().unwrap())
        })
        .collect::<HashMap<String, i64>>();

    let operations = lines
        .iter()
        .skip_while(|l| !l.is_empty())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (left, gate, right, _arrow, target) = l.split_whitespace().collect_tuple().unwrap();
            Operation {
                left: left.to_string(),
                right: right.to_string(),
                target: target.to_string(),
                gate: match gate {
                    "OR" => Gate::OR,
                    "XOR" => Gate::XOR,
                    "AND" => Gate::AND,
                    _ => panic!("Unknown gate: {}", gate),
                },
            }
        })
        .collect_vec();

    Input { values, operations }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test1.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 4);
    }

    #[test]
    fn test_part1_2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test2.txt"));
        let result1 = part1(&input);

        assert_eq!(result1, 2024);
    }

    #[test]
    fn test_part2() {
        let input = parse(&(env!("CARGO_MANIFEST_DIR").to_owned() + "/src/test3.txt"));
        let result2 = part2_better(&input);

        assert_eq!(result2, "z00,z01,z02,z05");
    }
}
