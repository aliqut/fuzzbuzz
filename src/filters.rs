// TODO: Return an error for invalid status codes
pub fn parse_filter_list(filter: Option<String>) -> Option<Vec<usize>> {
    filter.map(|f| f.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
}

// TODO: Return an error for invalid size
pub fn parse_range_filter(filter: Option<String>) -> Option<Vec<(usize, usize)>> {
    filter.map(|f| {
        f.split(',')
            .filter_map(|range| {
                let parts: Vec<&str> = range.split('-').collect();

                // If it is a range, e.g., 100-200, return tuple (100,200)
                if parts.len() == 2 {
                    Some((
                        parts[0].parse::<usize>().unwrap(),
                        parts[1].parse::<usize>().unwrap(),
                    ))

                // If it is a single number, e.g., 200, return tuple (200,200)
                } else if parts.len() == 1 {
                    let number = parts[0].parse::<usize>().unwrap();
                    Some((number, number))
                } else {
                    None
                }
            })
            .collect()
    })
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
) -> ResponseFilters {
    ResponseFilters {
        status_matches: convert_usize_to_u16(parse_filter_list(match_status))
            .expect("Invalid match-status options"),
        size_matches: parse_range_filter(match_size),
        line_matches: parse_range_filter(match_lines),
        status_filters: convert_usize_to_u16(parse_filter_list(filter_status))
            .expect("Invalid filter-status options"),
        size_filters: parse_range_filter(filter_size),
        line_filters: parse_range_filter(filter_lines),
    }
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
