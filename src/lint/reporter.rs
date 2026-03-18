use crate::lint::diagnostic::Diagnostic;
use colored::*;

pub struct Reporter;

impl Reporter {
    pub fn report(diagnostic: &Diagnostic) {
        let error_label = "error".red();
        let prefix = format!("{}{}", error_label, ":".white()).bold();

        println!("{} {}", prefix, diagnostic.title.bold());

        let line_number_width = diagnostic
            .location
            .as_ref()
            .and_then(|loc| loc.line_number)
            .map(|n| n.to_string().len())
            .unwrap_or(0);

        let padding_width = line_number_width.max(1);

        if let Some(ref location) = diagnostic.location {
            Self::report_location(location, padding_width);
        } else {
            Self::report_empty_location(padding_width);
        }

        Self::report_help(&diagnostic.help_text, padding_width);
        println!();
    }

    fn report_location(location: &crate::lint::diagnostic::Location, padding_width: usize) {
        let line_info = match (location.line_number, location.column_number) {
            (Some(line), Some(col)) => format!(":{}:{}", line, col),
            (Some(line), None) => format!(":{}", line),
            _ => String::new(),
        };

        println!(
            "{}{} {}{}",
            " ".repeat(padding_width),
            "-->".blue().bold(),
            location.file_path.display(),
            line_info
        );

        let gutter = format!("{:>width$} |", "", width = padding_width)
            .blue()
            .bold();
        println!("{}", gutter);

        if let Some(ref snippet) = location.snippet {
            let line_num = location.line_number.unwrap_or(0);
            println!(
                "{} {} {}",
                format!("{:>width$}", line_num, width = padding_width)
                    .blue()
                    .bold(),
                "|".blue().bold(),
                snippet
            );
            println!("{}", gutter);
        }
    }

    fn report_empty_location(padding_width: usize) {
        println!(
            "{}",
            format!("{:>width$} |", "", width = padding_width)
                .blue()
                .bold()
        );
    }

    fn report_help(help_text: &str, padding_width: usize) {
        println!(
            "{} {} {}",
            " ".repeat(padding_width),
            "=".blue().bold(),
            format!("help: {}", help_text).bold()
        );
    }
}
