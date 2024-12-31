use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Overview<T> {
    pub total_items: usize,                // Total number of items
    pub total_pages: usize,                 // Total number of pages
    pub current_page: usize,                // Current page
    pub page_size: usize,                   // Number of items per page
    pub items: Vec<T>,                    // List of items (generic)
}

pub struct OverviewRequest {
    pub pagination: Pagination,
    pub sorting: Sorting
}

pub struct Pagination {
    pub page: usize,
    pub size: usize
}

pub struct Sorting {
    pub field: String,
    pub direction: SortingDirection
}

impl Sorting {
    pub fn default() -> Self {
        Sorting { field: "Name".to_string(), direction: SortingDirection::ASC}
    }
}

pub enum SortingDirection {
    ASC,
    DESC,
}

impl Pagination {
    // Default pagination values
    pub fn default() -> Self {
        Pagination { page: 1, size: 10 }
    }
}

pub fn pagination_from_request(params: &HashMap<String, String>) -> Pagination {
    // Extract the pagination string from the parameters
    if let Some(pagination_param) = params.get("pagination") {
        // Split the string into page and size based on commas
        let pagination_parts: HashMap<_, _> = pagination_param
            .split(',')
            .filter_map(|part| {
                let mut split = part.splitn(2, ':');
                Some((split.next()?, split.next()?))
            })
            .collect();

        // Extract `page` and `size` values, using defaults if missing
        let page = pagination_parts.get("page")
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or(1);

        let size = pagination_parts.get("size")
            .and_then(|val| val.parse::<usize>().ok())
            .unwrap_or(10);

        Pagination { page, size }
    } else {
        // If no pagination param, return default pagination values
        Pagination::default()
    }
}

pub fn sorting_from_request(params: &HashMap<String, String>) -> Sorting {
    if let Some(sorting_param) = params.get("sorting") {
        let sorting_parts: HashMap<_, _> = sorting_param
            .split(',')
            .filter_map(|part| {
                let mut split = part.splitn(2, ':');
                Some((split.next()?, split.next()?))
            })
            .collect();

       // Extract `field` and `direction` values, using defaults if missing
       let field = sorting_parts.get("field")
       .cloned() // Ensure we get a String type
       .unwrap_or_else(|| "Name" );


        // Manually map the string to the SortingDirection enum
        let direction = sorting_parts.get("direction")
            .and_then(|val| match val.to_lowercase().as_str() {
                "asc" => Some(SortingDirection::ASC),
                "desc" => Some(SortingDirection::DESC),
                _ => None, // Default to None if the direction is not "asc" or "desc"
            })
            .unwrap_or(SortingDirection::ASC); // Default to SortingDirection::ASC if not found or invalid

        Sorting {field: String::from(field), direction }
    } else {
        Sorting::default()
    }
}
