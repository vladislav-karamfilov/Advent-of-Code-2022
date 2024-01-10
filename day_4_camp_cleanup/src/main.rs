fn main() {
    // solve_puzzle1();
    solve_puzzle2();
}

#[allow(dead_code)]
fn solve_puzzle1() {
    let mut fully_contained_sections_count = 0;

    loop {
        let mut assigned_sections = String::new();

        std::io::stdin()
            .read_line(&mut assigned_sections)
            .expect("Failed to read line");

        let assigned_sections = assigned_sections.trim();
        if assigned_sections.is_empty() {
            break;
        }

        let (first_section, second_section) = parse_sections(assigned_sections);

        if first_section.is_fully_contained_in(&second_section)
            || second_section.is_fully_contained_in(&first_section)
        {
            fully_contained_sections_count += 1;
        }
    }

    println!("{fully_contained_sections_count}");
}

#[allow(dead_code)]
fn solve_puzzle2() {
    let mut partially_contained_sections_count = 0;

    loop {
        let mut assigned_sections = String::new();

        std::io::stdin()
            .read_line(&mut assigned_sections)
            .expect("Failed to read line");

        let assigned_sections = assigned_sections.trim();
        if assigned_sections.is_empty() {
            break;
        }

        let (first_section, second_section) = parse_sections(assigned_sections);

        if first_section.is_partially_contained_in(&second_section)
            || second_section.is_partially_contained_in(&first_section)
        {
            partially_contained_sections_count += 1;
        }
    }

    println!("{partially_contained_sections_count}");
}

fn parse_sections(raw_sections: &str) -> (Section, Section) {
    let mut sections_splitter = raw_sections.split(',');

    let raw_first_section = sections_splitter.next().unwrap();
    let mut first_section_splitter = raw_first_section.split('-');
    let first_section = Section {
        from: first_section_splitter
            .next()
            .unwrap()
            .parse()
            .expect("Number expected"),
        to: first_section_splitter
            .next()
            .unwrap()
            .parse()
            .expect("Number expected"),
    };

    let raw_second_section = sections_splitter.next().unwrap();
    let mut second_section_splitter = raw_second_section.split('-');
    let second_section = Section {
        from: second_section_splitter
            .next()
            .unwrap()
            .parse()
            .expect("Number expected"),
        to: second_section_splitter
            .next()
            .unwrap()
            .parse()
            .expect("Number expected"),
    };

    (first_section, second_section)
}

struct Section {
    from: i32,
    to: i32,
}

impl Section {
    fn is_fully_contained_in(&self, other: &Section) -> bool {
        self.from <= other.from && other.to <= self.to
    }

    fn is_partially_contained_in(&self, other: &Section) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}
