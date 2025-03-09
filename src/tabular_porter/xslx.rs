use calamine::{open_workbook, Data, Range, Reader, Rows, Xlsx};
use chrono::NaiveDateTime;
use std::{fs::File, io::BufReader};

use super::{error::TabularPortError, TabularCursor, TabularPorter, TabularStringRecord};
pub struct XlsxPorter {
    workbook: Xlsx<BufReader<File>>,
    range_data: Option<Range<Data>>,
}

pub struct XlsxRecordCursor<'a> {
    row: Rows<'a, Data>,
}

impl TabularPorter for XlsxPorter {
    fn new<P>(src: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let excel: Xlsx<_> = open_workbook(src)?;
        Ok(XlsxPorter { workbook: excel, range_data: None })
    }
}

impl XlsxPorter {
    pub fn init_range(&mut self, range_name: &str) {
        let range = self.workbook.worksheet_range(range_name);
        self.range_data = range.ok();
    }
}

impl TabularCursor for XlsxPorter {
    fn cursor(
        &mut self,
    ) -> Result<impl Iterator<Item = TabularStringRecord>, Box<dyn std::error::Error>> {
        let range = self
            .range_data
            .as_ref()
            .ok_or(TabularPortError::NotInitialized("Cannot get Range Data".to_string()))?;
        Ok(XlsxRecordCursor { row: range.rows() })
    }
}

impl Iterator for XlsxRecordCursor<'_> {
    type Item = TabularStringRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.row.next();
        match row {
            Some(data) => Some(
                data.iter()
                    .map(|val| match val {
                        Data::Int(i) => i.to_string(),
                        Data::Float(f) => f.to_string(),
                        Data::String(s) => s.to_string(),
                        Data::Bool(b) => b.to_string(),
                        Data::DateTime(dt) => {
                            dt.as_datetime().unwrap_or(NaiveDateTime::default()).to_string()
                        }
                        Data::DateTimeIso(dt) => dt.to_string(),
                        Data::DurationIso(d) => d.to_string(),
                        Data::Error(e) => e.to_string(),
                        Data::Empty => "".to_string(),
                    })
                    .collect(),
            ),
            _ => None,
        }
    }
}
