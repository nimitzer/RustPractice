use crate::polygon::Polygon;

pub fn handle_area(polygons: &[Polygon], arg: Option<&str>) {
    match arg {
        Some("ODD") => {
            let area: f64 = polygons.iter().filter(|p| p.points.len() % 2 == 1).map(|p| p.area()).sum();
            println!("{:.1}", area);
        }
        Some("EVEN") => {
            let area: f64 = polygons.iter().filter(|p| p.points.len() % 2 == 0).map(|p| p.area()).sum();
            println!("{:.1}", area);
        }
        Some("MEAN") => {
            let area: f64 = polygons.iter().map(|p| p.area()).sum::<f64>() / polygons.len() as f64;
            println!("{:.1}", area);
        }
        Some(n_str) => {
            if let Ok(n) = n_str.parse::<usize>() {
                let area: f64 = polygons.iter().filter(|p| p.points.len() == n).map(|p| p.area()).sum();
                println!("{:.1}", area);
            } else {
                println!("<INVALID COMMAND>");
            }
        }
        None => println!("<INVALID COMMAND>"),
    }
}

pub fn handle_max(polygons: &[Polygon], arg: Option<&str>) {
    match arg {
        Some("AREA") => {
            if polygons.len() < 2 {
                println!("<INVALID COMMAND>");
            } else {
                let max = polygons
                    .iter()
                    .map(|p| p.area())
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0);
                println!("{:.1}", max);
            }
        }
        Some("VERTEXES") => {
            if polygons.len() < 2 {
                println!("<INVALID COMMAND>");
            } else {
                let max = polygons.iter().map(|p| p.points.len()).max().unwrap_or(0);
                println!("{}", max);
            }
        }
        _ => println!("<INVALID COMMAND>"),
    }
}

pub fn handle_min(polygons: &[Polygon], arg: Option<&str>) {
    match arg {
        Some("AREA") => {
            if polygons.len() < 2 {
                println!("<INVALID COMMAND>");
            } else {
                let min = polygons
                    .iter()
                    .map(|p| p.area())
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0);
                println!("{:.1}", min);
            }
        }
        Some("VERTEXES") => {
            if polygons.len() < 2 {
                println!("<INVALID COMMAND>");
            } else {
                let min = polygons.iter().map(|p| p.points.len()).min().unwrap_or(0);
                println!("{}", min);
            }
        }
        _ => println!("<INVALID COMMAND>"),
    }
}

pub fn handle_count(polygons: &[Polygon], arg: Option<&str>) {
    match arg {
        Some("ODD") => {
            let count = polygons.iter().filter(|p| p.points.len() % 2 == 1).count();
            println!("{}", count);
        }
        Some("EVEN") => {
            let count = polygons.iter().filter(|p| p.points.len() % 2 == 0).count();
            println!("{}", count);
        }
        Some(n_str) => {
            if let Ok(n) = n_str.parse::<usize>() {
                let count = polygons.iter().filter(|p| p.points.len() == n).count();
                println!("{}", count);
            } else {
                println!("<INVALID COMMAND>");
            }
        }
        None => println!("<INVALID COMMAND>"),
    }
}

pub fn handle_maxseq(polygons: &[Polygon], arg: Option<&str>) {
    match arg.and_then(|s| s.parse::<Polygon>().ok()) {
        Some(target) => {
            let max_seq = polygons
                .iter()
                .map(|p| p.points == target.points)
                .scan(0, |state, is_match| {
                    *state = if is_match { *state + 1 } else { 0 };
                    Some(*state)
                })
                .max()
                .unwrap_or(0);

            println!("{}", max_seq);
        }
        None => println!("<INVALID COMMAND>"),
    }
}

pub fn handle_lessarea(polygons: &[Polygon], arg: Option<&str>) {
    match arg.and_then(|s| s.parse::<Polygon>().ok()) {
        Some(target) => {
            let count = polygons.iter().filter(|p| p.area() < target.area()).count();
            println!("{}", count);
        }
        None => println!("<INVALID COMMAND>"),
    }
}
