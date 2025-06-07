use crate::*;
use rayon::prelude::*;

pub fn list(profit_list: &[model::Profit]) -> Vec<model::AnnualProfit> {
    let year_list = profit_list
        .par_iter()  // 使用rayon并行迭代
        .map(|item| item.date.split('-').next().unwrap().to_string())
        .collect::<std::collections::BTreeSet<String>>();

    year_list
        .par_iter()  // 再次使用rayon并行迭代
        .map(|year| {
            let mut iter = profit_list.iter().filter(|item| item.date.contains(year));
            if iter.clone().count() < 2 {
                return model::AnnualProfit {
                    year: year.to_string(),
                    index_profit: 0.0,
                    ma_profit: 0.0,
                };
            }
            let first = iter.next().unwrap();
            let last = iter.last().unwrap();
            model::AnnualProfit {
                year: year.to_string(),
                index_profit: last.close_point - first.close_point,
                ma_profit: last.value - first.value,
            }
        })
        .collect()
}

pub fn list_by_quarter(profit_list: &[model::Profit]) -> Vec<model::QuarterlyProfit> {
    let quarter_list = profit_list
        .par_iter()
        .map(|item| {
            let parts: Vec<&str> = item.date.split('-').collect();
            let year = parts[0];
            let month = parts[1].parse::<u32>().unwrap();
            let quarter = (month - 1) / 3 + 1;
            format!("{}-Q{}", year, quarter)
        })
        .collect::<std::collections::BTreeSet<String>>();

    quarter_list
        .par_iter()
        .map(|quarter| {
            let mut iter = profit_list.iter().filter(|item| {
                let parts: Vec<&str> = item.date.split('-').collect();
                let year = parts[0];
                let month = parts[1].parse::<u32>().unwrap();
                let item_quarter = (month - 1) / 3 + 1;
                format!("{}-Q{}", year, item_quarter) == *quarter
            });
            
            if iter.clone().count() < 2 {
                return model::AnnualProfit {
                    year: year.to_string(),
                    index_profit: 0.0,
                    ma_profit: 0.0,
                };
            }
            let first = iter.next().unwrap();
            let last = iter.last().unwrap();
            model::AnnualProfit {
                year: year.to_string(),
                index_profit: last.close_point - first.close_point,
                ma_profit: last.value - first.value,
            }
        })
        .collect()
}
