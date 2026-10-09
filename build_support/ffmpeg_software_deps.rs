// Only add optional H.26x libraries requested by the selected FFmpeg build.
// Finding an unrelated x264/x265 library in a shared prefix is not an opt-in.
pub fn requested_software_h26x(metadata: &str) -> Vec<&'static str> {
    let mut requested = Vec::new();
    for name in ["x264", "x265", "openh264"] {
        let found = metadata.lines().any(|line| {
            let Some((field, value)) = line.split_once(':') else {
                return false;
            };
            if !matches!(
                field.trim(),
                "Libs" | "Libs.private" | "Requires" | "Requires.private"
            ) {
                return false;
            }
            value.split_whitespace().any(|token| {
                let token = token.trim_matches(|c| c == '"' || c == ',' || c == ';');
                let token = token
                    .strip_prefix("-l:")
                    .or_else(|| token.strip_prefix("-l"))
                    .unwrap_or(token);
                let token = token.rsplit(['/', '\\']).next().unwrap_or(token);
                let token = token.strip_prefix("lib").unwrap_or(token);
                let token = token
                    .strip_suffix(".lib")
                    .or_else(|| token.strip_suffix(".a"))
                    .unwrap_or(token);
                token == name
            })
        });
        if found {
            requested.push(name);
        }
    }
    requested
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_declared_dependencies_opt_in() {
        assert!(
            requested_software_h26x("Description: x264\nprefix=F:/x265\nLibs: -lavcodec -lz")
                .is_empty()
        );
        assert_eq!(
            requested_software_h26x(
                "Libs.private: -lx264 -lx265 -lx264\nRequires.private: openh264 >= 2.0"
            ),
            ["x264", "x265", "openh264"]
        );
    }

    #[test]
    fn msvc_archive_names_and_exact_matches() {
        assert_eq!(
            requested_software_h26x(
                "Libs.private: F:\\SDK\\lib\\libx264.lib -l:libx265.a -lx264-lookalike"
            ),
            ["x264", "x265"]
        );
        assert!(requested_software_h26x("Libs.private: -lx264-lookalike -lx265-extra").is_empty());
    }
}
