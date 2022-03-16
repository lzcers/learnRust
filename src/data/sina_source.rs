use anyhow::Result;
use reqwest::header;
use std::collections::HashMap;

use super::{Level5thData, StockDataItem, StockPrice};
pub struct SinaSource {}

impl SinaSource {
    pub async fn get_rt_stock_data(code: &str) -> Result<StockPrice> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Referer",
            header::HeaderValue::from_static(
                "http://finance.sina.com.cn/realstock/company/sh000300/nc.shtml",
            ),
        );

        let respdata = reqwest::Client::builder()
            .default_headers(headers)
            .build()?
            .get("https://hq.sinajs.cn/list=sh000300")
            .send()
            .await?
            .text()
            .await?;

        let extract_result = regex::Regex::new("\"(.*)\"")?
            .captures(&respdata)
            .unwrap()
            .get(1)
            .map_or("", |r| r.as_str());

        // ['股票名字', '今日开盘价', '昨日收盘价', '当前价格', '今日最高价', '今日最低价', '竞买价', '竞卖价',
        //    '成交的股票数', '成交金额', '买一量', '买一价', '买二量', '买二价', '买三量', '买三价', '买四量', '买四价',
        //    '买五量', '买五价', '卖一量', '卖一价', '卖二量', '卖二价', '卖三量', '卖三价', '卖四量', '卖四价',
        //    '卖五量', '卖五价', '日期', '时间']
        let names = vec![
            "name", "open", "close", "current", "high", "low", "cab", "cas", "volume", "money",
            "b10", "b11", "b20", "b21", "b30", "b31", "b40", "b41", "b50", "b51", "a10", "a11",
            "a20", "a21", "a30", "a31", "a40", "a41", "a50", "a51", "date", "time",
        ];

        let res = names
            .into_iter()
            .zip(extract_result.split(','))
            .collect::<HashMap<_, _>>();

        let datetime = {
            let date = res.get("date").unwrap();
            let time = res.get("time").unwrap();
            format!("{} {}", date, time)
        };

        let parse_key_to_value = |s: &str| res.get(s).unwrap().parse::<f64>().unwrap();

        let stock_data_item = StockDataItem {
            datetime,
            open: parse_key_to_value("open"),
            close: parse_key_to_value("current"),
            high: parse_key_to_value("high"),
            low: parse_key_to_value("low"),
            volume: parse_key_to_value("volume"),
            money: parse_key_to_value("money"),
        };

        let level5hData = Level5thData {
            a1: (parse_key_to_value("a10"), parse_key_to_value("a11")),
            a2: (parse_key_to_value("a20"), parse_key_to_value("a21")),
            a3: (parse_key_to_value("a30"), parse_key_to_value("a31")),
            a4: (parse_key_to_value("a40"), parse_key_to_value("a41")),
            a5: (parse_key_to_value("a50"), parse_key_to_value("a51")),
            b1: (parse_key_to_value("b10"), parse_key_to_value("b11")),
            b2: (parse_key_to_value("b20"), parse_key_to_value("b21")),
            b3: (parse_key_to_value("b30"), parse_key_to_value("b31")),
            b4: (parse_key_to_value("b40"), parse_key_to_value("b41")),
            b5: (parse_key_to_value("b50"), parse_key_to_value("b51")),
        };

        Ok(StockPrice {
            code: code.to_owned(),
            name: res.get("name").unwrap().to_string(),
            data: stock_data_item,
            level5hData: level5hData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SinaSource;
    use anyhow::Result;

    #[tokio::test]
    async fn test_get_rt_stock_data() -> Result<()> {
        let data = SinaSource::get_rt_stock_data("").await?;
        println!("{data:#?}");
        Ok(())
    }
}
