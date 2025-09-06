use prettytable::format;

pub fn summary_styling() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders(' ')
        .padding(2, 1)
        .build()
}

pub fn table_styling() -> format::TableFormat {
    format::FormatBuilder::new()
        .borders(' ')
        .separators(
            &[format::LinePosition::Title, format::LinePosition::Bottom],
            format::LineSeparator::new('─', '─', ' ', '─'),
        )
        .padding(2, 1)
        .build()
}
