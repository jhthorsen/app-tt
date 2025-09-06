use prettytable::{Table, format};

pub const DASH: &str = "—";

pub fn plain_table() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders(' ')
        .padding(1, 1)
        .build()
}

pub fn print_table(mut t: Table, format: format::TableFormat, padding: [usize; 2]) {
    t.set_format(format);
    (0..padding[0]).for_each(|_| println!());
    t.printstd();
    (0..padding[1]).for_each(|_| println!());
}

pub fn regular_table() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders(' ')
        .padding(1, 1)
        .separators(
            &[format::LinePosition::Title, format::LinePosition::Bottom],
            format::LineSeparator::new('─', '─', ' ', '─'),
        )
        .build()
}
