use anyhow::{Result, anyhow};

pub fn parse_process<S>(k: S) -> Result<(String, String)>
where
    S: ToString,
{
    let k = k.to_string();

    let Some(pos_head) = k.find('{') else {
        return Err(anyhow!("Missing character '{'".to_string()));
    };
    let Some(pos_end) = k.find('}') else {
        return Err(anyhow!("Missing character '{'".to_string()));
    };
    let process = k.get(pos_head + 1..pos_end).unwrap();
    let package = k.get(..pos_head).unwrap();

    Ok((process.to_string(), package.to_string()))
}

pub fn parse_cpus<S>(v: S) -> Vec<u8>
where
    S: ToString,
{
    let v = v.to_string();
    if v.contains('-') {
        let pos: Vec<&str> = v.split('-').collect();

        let pos_start: u8 = pos[0].parse().unwrap();
        let pos_end: u8 = pos[1].parse().unwrap();
        (pos_start..pos_end).collect()
    } else {
        vec![v.parse::<u8>().unwrap()]
    }
}
