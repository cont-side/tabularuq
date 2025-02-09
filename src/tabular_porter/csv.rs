use super::{TabularCursor, TabularPorter, TabularStringRecord};

pub struct CsvPorter {
    reader: csv::Reader<std::fs::File>,
}

pub struct CsvRecordCursor<'a> {
    cursor: csv::StringRecordsIter<'a, std::fs::File>,
}

impl TabularPorter for CsvPorter {
    fn new<P>(src: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(src)?;
        let reader = csv::Reader::from_reader(file);
        Ok(CsvPorter { reader })
    }
}

impl TabularCursor for CsvPorter {
    fn cursor(
        &mut self,
    ) -> Result<impl Iterator<Item = TabularStringRecord>, Box<dyn std::error::Error>> {
        Ok(CsvRecordCursor { cursor: self.reader.records() })
    }
}

impl Iterator for CsvRecordCursor<'_> {
    type Item = TabularStringRecord;

    fn next(&mut self) -> Option<Self::Item> {
        let record = self.cursor.next();
        match record {
            Some(Ok(record)) => Some(record.iter().map(|s| s.to_string()).collect()),
            _ => None,
        }
    }
}
