// TODO: Return an error for invalid status codes
pub fn parse_filter_list(filter: Option<String>) -> Option<Vec<String>> {
    filter.map(|f| f.split(',').map(|s| s.to_string()).collect())
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
    pub status_filters: Option<Vec<String>>,
    pub size_filters: Option<Vec<(usize, usize)>>,
    pub line_filters: Option<Vec<(usize, usize)>>,
}
