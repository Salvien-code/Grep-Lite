use std::vec;

fn main() {
    let ctx_lines = 2;
    let needle = "oo";

    let haystack = "\
Every face, every shop, 
bedroom window, public-house, and
dark square is a picture 
feverishly turned--in search of what?
It is the same with books.
What do we seek 
through millions of pages?";

    // tags holds line numbers where numbers matches occur.
    let mut tags: Vec<usize> = vec![];

    // ctx contains a vector per match to hold the context lines.
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    // Iterate through the lines, recording the numbers where matches
    // are encountered.
    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            // Vec::with_capacity(n) reserves space for n items. No explicit
            // type signature is required as it can be inferred via the
            // definition on line 20.
            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    // When there are no matches, exits early
    if tags.is_empty() {
        return;
    }

    // For each tag, at every line, checks to see if we are near a match. When
    // we are, adds that line to the relevant Vec<T> within ctx.
    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            // saturating_sub() is subtraction that returns 0 on integer
            // underflow rather than crashing the program (CPUs don't like
            // attempting to send usize below zero).
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                // Copies line into a new String and stores that locally for
                // each match.
                let line_as_string = String::from(line);
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    for local_ctx in ctx.iter() {
        // ref line informs the compiler that we want to borrow this value
        // rather than move it.
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}
