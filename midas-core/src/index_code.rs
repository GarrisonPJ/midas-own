use crate::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub async fn list() -> Result<Vec<model::IndexCode>, Box<dyn std::error::Error>> {
    let mut file = File::open("index-data/codes.json").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let index_code_list = serde_json::from_str::<Vec<model::IndexCode>>(&contents)?;
    Ok(index_code_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_list() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let index_code_list = list().await.unwrap();
            assert_eq!(index_code_list.len(), 8);
        });
    }
}
