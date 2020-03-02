use std::env::args;
use std::iter::Peekable;
use std::str::Chars;

fn atomic_mass(element: &str) -> Option<f64> {
    match element {
        "H"  => Some(1.00794),
        "He" => Some(4.002602),
        "Li" => Some(6.941),
        "Be" => Some(9.01218),
        "B"  => Some(10.811),
        "C"  => Some(12.011),
        "N"  => Some(14.00674),
        "O"  => Some(15.9994),
        "F"  => Some(18.998403),
        "Ne" => Some(20.1797),
        "Na" => Some(22.989768),
        "Mg" => Some(24.305),
        "Al" => Some(26.981539),
        "Si" => Some(28.0855),
        "P"  => Some(30.973762),
        "S"  => Some(32.066),
        "Cl" => Some(35.4527),
        "Ar" => Some(39.948),
        "K"  => Some(39.0983),
        "Ca" => Some(40.078),
        "Sc" => Some(44.95591),
        "Ti" => Some(47.88),
        "V"  => Some(50.9415),
        "Cr" => Some(51.9961),
        "Mn" => Some(54.93805),
        "Fe" => Some(55.847),
        "Co" => Some(58.9332),
        "Ni" => Some(58.6934),
        "Cu" => Some(63.546),
        "Zn" => Some(65.39),
        "Ga" => Some(69.723),
        "Ge" => Some(72.61),
        "As" => Some(74.92159),
        "Se" => Some(78.96),
        "Br" => Some(79.904),
        "Kr" => Some(83.8),
        "Rb" => Some(85.4678),
        "Sr" => Some(87.62),
        "Y"  => Some(88.90585),
        "Zr" => Some(91.224),
        "Nb" => Some(92.90638),
        "Mo" => Some(95.94),
        "Tc" => Some(97.9072),
        "Ru" => Some(101.07),
        "Rh" => Some(102.9055),
        "Pd" => Some(106.42),
        "Ag" => Some(107.8682),
        "Cd" => Some(112.411),
        "In" => Some(114.818),
        "Sn" => Some(118.71),
        "Sb" => Some(121.76),
        "Te" => Some(127.6),
        "I"  => Some(126.90447),
        "Xe" => Some(131.29),
        "Cs" => Some(132.90543),
        "Ba" => Some(137.327),
        "La" => Some(138.9055),
        "Ce" => Some(140.115),
        "Pr" => Some(140.90765),
        "Nd" => Some(144.24),
        "Pm" => Some(144.9127),
        "Sm" => Some(150.36),
        "Eu" => Some(151.965),
        "Gd" => Some(157.25),
        "Tb" => Some(158.92534),
        "Dy" => Some(162.5),
        "Ho" => Some(164.93032),
        "Er" => Some(167.26),
        "Tm" => Some(168.93421),
        "Yb" => Some(173.04),
        "Lu" => Some(174.967),
        "Hf" => Some(178.49),
        "Ta" => Some(180.9479),
        "W"  => Some(183.84),
        "Re" => Some(186.207),
        "Os" => Some(190.23),
        "Ir" => Some(192.22),
        "Pt" => Some(195.08),
        "Au" => Some(196.96654),
        "Hg" => Some(200.59),
        "Tl" => Some(204.3833),
        "Pb" => Some(207.2),
        "Bi" => Some(208.98037),
        "Po" => Some(208.9824),
        "At" => Some(209.9871),
        "Rn" => Some(222.0176),
        "Fr" => Some(223.0197),
        "Ra" => Some(226.0254),
        "Ac" => Some(227.0278),
        "Th" => Some(232.0381),
        "Pa" => Some(231.03588),
        "U"  => Some(238.0289),
        "Np" => Some(237.048),
        "Pu" => Some(244.0642),
        "Am" => Some(243.0614),
        "Cm" => Some(247.0703),
        "Bk" => Some(247.0703),
        "Cf" => Some(251.0796),
        "Es" => Some(252.083),
        "Fm" => Some(257.0951),
        "Md" => Some(258.1),
        "No" => Some(259.1009),
        "Lr" => Some(262.11),
        _ => None,
    }
}

type Parser<'a> = Peekable<Chars<'a>>;

fn parse_element(parser: &mut Parser) -> Result<String, char> {
    // An element symbol is always an upper case letter, followed by lower case letters
    let c = match parser.next() {
        Some(c) => c,
        None => return Ok(String::new()), // end of string
    };

    let mut s = String::new();
    if !c.is_ascii_uppercase() { return Err(c); }
    else { s.push(c); }

    while let Some(&c) = parser.peek() {
        if c.is_ascii_lowercase() { s.push(c); }
        else { break; }
        parser.next();
    }
    Ok(s)
}

fn parse_number(parser: &mut Parser) -> Option<i64> {
    let mut s = String::new();
    while let Some(&c) = parser.peek() {
        if c.is_ascii_digit() { s.push(c); }
        else { break; }
        parser.next();
    }
    if s.is_empty() { None }
    else {
        let result = s.parse().expect("string should be an integer");
        Some(result)
    }
}

fn molar_mass(formula: &str) -> Result<f64, String> {
    let mut parser: Parser = formula.chars().peekable();
    let mut total_mass = 0.;

    loop {
        let mass;
        
        if let Some('(') = parser.peek() {
            parser.next();
            let mut depth = 1;
            let mut s = String::new();
            loop {
                let c = parser.next().ok_or("Parentheses not closed")?;
                if c == '(' { depth += 1; }
                if c == ')' { depth -= 1; }
                if depth == 0 { break; }
                s.push(c);
            }
            mass = molar_mass(&s)?;
        } else {
            let element = parse_element(&mut parser)
                .map_err(|c| {
                    format!("Expected atomic symbol, found '{}' \
                             (Did you check capitalization?)", c)
                }
            )?;
            // Empty string signals that the string was fully consumed
            if element.is_empty() { break; }

            mass = atomic_mass(&element)
                .ok_or(format!("Unknown atomic symbol '{}'", element))?;
        }
        let count = parse_number(&mut parser).unwrap_or(1);

        total_mass += mass * count as f64;
    }
    Ok(total_mass)
}

fn main() {
    let args = args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("usage: molarmass [formula]");
        std::process::exit(-1);
    }

    match molar_mass(&args[1]) {
        Ok(mass) => println!("{:.03} g/mol", mass),
        Err(e) => eprintln!("Error parsing formula: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_delta {
        ($formula:expr, $expected:expr, $delta:expr) => {
            let mass = molar_mass($formula).unwrap();
            let error = f64::abs(mass - $expected);
            if !(error < $delta) {
                panic!("{}: Expected {}, got {} (error: {})",
                    $formula, $expected, mass, error); }
        }
    }

    macro_rules! assert_invalid {
        ($formula:expr) => {
            if let Ok(i) = molar_mass($formula) {
                panic!("Invalid input '{}' returned {}", $formula, i);
            }
        }
    }

    #[test]
    fn test_molar_mass() {
        // atoms
        assert_delta!("H", 1.0, 0.1);
        assert_delta!("Na", 23.0, 0.1);
        assert_delta!("U", 238.0, 0.1);

        assert_invalid!("Cat");
        assert_invalid!("he");
        assert_invalid!("💯");

        // simple compounds
        assert_delta!("HCl", 36.5, 0.1);
        assert_delta!("KIO3", 214.0, 0.1);
        assert_delta!("KIO3", 214.0, 0.1);

        assert_invalid!("HAx");
        assert_invalid!("NaHST");
        assert_invalid!("MKCl");

        // numbers!!
        assert_delta!("FeCl3", 162.2, 0.1);
        assert_delta!("HC2H3O2", 60.0, 0.1);
        assert_delta!("XeF6", 245.3, 0.1);
        assert_delta!("PbSO4", 303.3, 0.1);

        // parentheses
        assert_delta!("Fe(NO3)3", 241.9, 0.1);
        assert_delta!("(NH4)2SO4", 132.1, 0.1);
        assert_delta!("(Pb(H2)2)2", 422.46, 0.1);
        assert_delta!("(Pb(H2)2)2NO3", 484.47, 0.1);
        assert_delta!("Na2(CH3(CH2)2CH3)2", 162.2, 0.1);
        assert_delta!("(Ag(Pb(H2)2)2)2SO4", 1156.7, 1.0);
        assert_delta!("(Tc(H2O)3CO(Fe3(SO4)2)2)2", 1800.0, 10.0);

        assert_invalid!("Fe(OH");
        assert_invalid!("Ba(((((OH)");
        assert_invalid!(")FeH3");
        assert_invalid!("BaC(l2)");
    }
}
