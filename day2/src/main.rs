use std::fs::read_to_string;

#[derive(PartialEq)]
enum Derivative {
    Inc,
    Dec,
}

enum ReportType {
    Safe,
    Unsafe(usize),
}

fn check_report(report: Vec<usize>) -> ReportType {
    let mut der = None;

    for (i, levels) in report.windows(2).enumerate() {
        let uno = levels[0];
        let dos = levels[1];

        if uno.abs_diff(dos) > 3 || uno.abs_diff(dos) < 1 {
            return ReportType::Unsafe(i);
        }

        if uno < dos {
            if der == Some(Derivative::Dec) {
                return ReportType::Unsafe(i);
            }
            der = Some(Derivative::Inc)
        }
        if uno > dos {
            if der == Some(Derivative::Inc) {
                return ReportType::Unsafe(i);
            }
            der = Some(Derivative::Dec)
        }
    }

    ReportType::Safe
}

fn main() {
    let input = read_to_string("input").expect("input not in root");

    let mut safe = 0;
    let mut fuckup_safe = 0;
    for line in input.lines() {
        let report: Vec<usize> = line
            .split_whitespace()
            .map(|measurement| {
                measurement
                    .to_string()
                    .parse()
                    .expect("should have been a number")
            })
            .collect();

        let report_type = check_report(report.clone());
        match report_type {
            ReportType::Safe => {
                safe += 1;
                fuckup_safe += 1;
            }
            ReportType::Unsafe(problem_i) => {
                let mut report_c = report.clone();
                report_c.remove(problem_i);
                match check_report(report_c) {
                    ReportType::Safe => {
                        fuckup_safe += 1;
                    }
                    ReportType::Unsafe(_) => {
                        let mut report_c = report.clone();
                        report_c.remove(problem_i + 1);
                        match check_report(report_c) {
                            ReportType::Safe => {
                                fuckup_safe += 1;
                            }
                            ReportType::Unsafe(_) => {}
                        }
                    }
                }
            }
        }
    }

    println!("(Part 1) Safe Reports: {safe}");
    println!("(Part 2) Safe Reports: {fuckup_safe}");
}
