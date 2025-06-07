use crate::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn list_by_code(code: &str) -> Result<Vec<model::IndexData>, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("index-data/{}.json", code)).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let mut index_data_list = serde_json::from_str::<Vec<model::IndexData>>(&contents)?;
    index_data_list.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(index_data_list)
}

pub async fn list_by_code_with_range(
    code: &str,
    date_begin: &str,
    date_end: &str,
) -> Result<Vec<model::IndexData>, Box<dyn std::error::Error>> {
    let mut file = File::open(format!("index-data/{}.json", code)).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let mut index_data_list = serde_json::from_str::<Vec<model::IndexData>>(&contents)?;
    
    // 过滤指定时间范围
    index_data_list.retain(|data| {
        data.date >= date_begin && data.date <= date_end
    });
    
    index_data_list.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(index_data_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_list_by_code() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let index_data_list = list_by_code("000300").await.unwrap();
            assert_eq!(index_data_list.len(), 4419);
            assert_eq!(index_data_list[0].date, "2005-01-04");
            assert_eq!(index_data_list[0].close_point, 982.79);
        });
    }
}
