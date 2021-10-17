macro_rules! input {
    ($q:tt) => {{
        use std::io;
        use std::io::Write;

        let mut buff = String::new();
        print!($q);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buff).unwrap();
        while buff.ends_with('\n') || buff.ends_with('\r') {
            buff.pop();
        }
        buff
    }};

($($exp:expr),+) => {
    {
        use std::io;
        use std::io::Write;

        let mut buff = String::new();
        print!($($exp),+);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buff).unwrap();
        while buff.ends_with('\n') || buff.ends_with('\r') {
            buff.pop();
        }
        buff
    }
}
}

macro_rules! time_print {
    ($q:tt, $f:expr) => {{
        print!($q);

        let start = std::time::Instant::now();
        let res = ($f)();
        let time = start.elapsed().as_micros();

        print!(" ({}us)\n", time);

        res
    }};
}

macro_rules! quick_res {
    ($code:expr, $data:expr) => {{
        return Response::new(
            $code,
            $data,
            vec![Header::new("Content-Type", "text/plain")],
        );
    }};
}

/// Check if a string contains any values in a vector
pub fn contains_any(text: &str, items: &[&str]) -> bool {
    for item in items {
        if text.contains(item) {
            return true;
        }
    }
    false
}
