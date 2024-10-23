use anyhow::{Context, Result};

pub fn parse_filter_list(filter: Option<String>) -> Result<Option<Vec<usize>>> {
    if let Some(f) = filter {
        let parsed: Result<Vec<usize>> = f
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .with_context(|| format!("Invalid status code: '{}'", s))
            })
            .collect();

        parsed.map(Some)
    } else {
        Ok(None)
    }
}

pub fn parse_range_filter(filter: Option<String>) -> Result<Option<Vec<(usize, usize)>>> {
    if let Some(f) = filter {
        let parsed_ranges: Result<Vec<(usize, usize)>> = f
            .split(',')
            .map(|range| {
                let parts: Vec<&str> = range.split('-').map(|s| s.trim()).collect();

                if parts.len() == 2 {
                    let start = parts[0]
                        .parse::<usize>()
                        .with_context(|| format!("Invalid start of range: '{}'", parts[0]))?;
                    let end = parts[1]
                        .parse::<usize>()
                        .with_context(|| format!("Invalid end of range: '{}'", parts[1]))?;
                    Ok((start, end))
                } else if parts.len() == 1 {
                    let number = parts[0]
                        .parse::<usize>()
                        .with_context(|| format!("Invalid single number: '{}'", parts[0]))?;
                    Ok((number, number))
                } else {
                    Err(anyhow::anyhow!("Invalid range format: '{}'", range))
                }
            })
            .collect();

        parsed_ranges.map(Some)
    } else {
        Ok(None)
    }
}

#[derive(Debug)]
pub struct ResponseFilters {
    pub status_matches: Option<Vec<u16>>,
    pub size_matches: Option<Vec<(usize, usize)>>,
    pub line_matches: Option<Vec<(usize, usize)>>,
    pub status_filters: Option<Vec<u16>>,
    pub size_filters: Option<Vec<(usize, usize)>>,
    pub line_filters: Option<Vec<(usize, usize)>>,
}

pub fn parse_response_filters(
    match_status: Option<String>,
    match_size: Option<String>,
    match_lines: Option<String>,
    filter_status: Option<String>,
    filter_size: Option<String>,
    filter_lines: Option<String>,
) -> Result<ResponseFilters> {
    Ok(ResponseFilters {
        status_matches: convert_usize_to_u16(parse_filter_list(match_status)?)
            .expect("Invalid match-status options"),
        size_matches: parse_range_filter(match_size)?,
        line_matches: parse_range_filter(match_lines)?,
        status_filters: convert_usize_to_u16(parse_filter_list(filter_status)?)
            .expect("Invalid filter-status options"),
        size_filters: parse_range_filter(filter_size)?,
        line_filters: parse_range_filter(filter_lines)?,
    })
}

fn convert_usize_to_u16(vec: Option<Vec<usize>>) -> Result<Option<Vec<u16>>, String> {
    match vec {
        None => Ok(None),
        Some(vec) => {
            let vec = vec
                .into_iter()
                .map(|x| {
                    u16::try_from(x)
                        .map_err(|_| format!("Value {} is too large for u16", x))
                        .unwrap()
                })
                .collect();
            Ok(Some(vec))
        }
    }
}
