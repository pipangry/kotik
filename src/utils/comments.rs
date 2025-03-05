// Use it only for json
pub fn clear_comments(content: &mut String) {
    let mut in_big_comment = false;
    let mut in_str = false;
    let mut write_ptr = 0;
    let mut chars = content.chars().collect::<Vec<_>>();

    let mut i = 0;
    while i < chars.len() {
        let sym = chars[i];

        if sym == '"' {
            in_str = !in_str;
        }

        if !in_str {
            if sym == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            } else if sym == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                in_big_comment = true;
                i += 2;
                continue;
            } else if in_big_comment && sym == '*' && i + 1 < chars.len() && chars[i + 1] == '/' {
                in_big_comment = false;
                i += 2;
                continue;
            } else if in_big_comment {
                i += 1;
                continue;
            }
        }

        if !in_big_comment {
            if write_ptr != i {
                chars[write_ptr] = chars[i];
            }
            write_ptr += 1;
        }
        i += 1;
    }

    chars.truncate(write_ptr);
    *content = chars.into_iter().collect();
}