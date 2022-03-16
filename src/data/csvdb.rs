use super::{StockDB, StockDataItem};
use anyhow::{Error, Result};
use csv::{Terminator, WriterBuilder};
use std::{fs::File, path::Path};

struct CSVDB {
    file: File,
}

impl CSVDB {
    pub fn open(path: &Path) -> Result<Self, Error> {
        let file = File::options()
            .read(true)
            .write(true)
            .append(true)
            .open(path)?;
        Ok(CSVDB { file })
    }
}

impl StockDB for CSVDB {
    fn insert_stock_data_items(&mut self, code: &str, items: Vec<StockDataItem>) -> Result<()> {
        let mut wtr = WriterBuilder::new()
            .terminator(Terminator::CRLF)
            .from_writer(&self.file);
        for item in items {
            wtr.serialize((
                &code,
                &item.datetime,
                &item.open,
                &item.close,
                &&item.high,
                &item.low,
                &item.volume,
                &item.money,
            ))?;
        }
        wtr.flush()?;
        Ok(())
    }

    fn get_stock_data_items(
        &self,
        code: &str,
        start_time: &str,
        end_time: &str,
        unit: &str,
    ) -> Result<Vec<StockDataItem>, Error> {
        Ok(csv::Reader::from_reader(&self.file)
            .deserialize::<StockDataItem>()
            .map(|i| i.unwrap())
            .collect::<Vec<StockDataItem>>())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::{StockDB, StockDataItem};

    use super::CSVDB;
    use anyhow::Result;
    use std::path::Path;

    #[test]
    fn it_works() -> Result<()> {
        let db = CSVDB::open(Path::new("./data/300.csv"))?;
        let items = db.get_stock_data_items("test", "start_time", "", "")?;
        for item in items {
            println!("{item:?}");
        }
        Ok(())
    }

    #[test]
    fn insert_stock_data_items() -> Result<()> {
        let mut db = CSVDB::open(Path::new("./data/300.csv"))?;
        let items = vec![StockDataItem {
            datetime: "0".to_owned(),
            open: 1.0,
            close: 2.0,
            high: 3.0,
            low: 4.0,
            volume: 5.0,
            money: 6.0,
        }];
        db.insert_stock_data_items("300", items)?;
        Ok(())
    }
}
