//! Render the Headroom footer block.

use std::collections::HashMap;

const RULE: &str = "─────────────────────────────────────────────────────────────";

/// Deterministic compressor-hint label for a tool name.
fn hint_for(tool: &str) -> &'static str {
    match tool {
        "Bash" | "bash" => "likely CCR-compressible",
        "Read" | "Edit" | "Write" => "ContentRouter target",
        "Grep" | "Glob" => "low value; rarely worth compressing",
        "Agent" | "Task" => "subagent output, SmartCrusher target",
        "<system-reminder>" => "system reminder; check skill/hook bloat",
        t if t.starts_with("mcp__") => "MCP tool result; check schema re-injection",
        _ => "candidate for ContentRouter",
    }
}

fn human_tokens(n: usize) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{}k", n / 1_000)
    } else {
        format!("{}", n)
    }
}

/// Render the footer. If `counts` is empty, returns an empty string
/// (callers should suppress emission in that case).
pub fn render(counts: &HashMap<String, usize>) -> String {
    if counts.is_empty() {
        return String::new();
    }

    let total: usize = counts.values().sum();
    if total == 0 {
        return String::new();
    }

    let mut entries: Vec<(&String, &usize)> = counts.iter().collect();
    entries.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    let top: Vec<_> = entries.into_iter().take(3).collect();

    let mut out = String::new();
    out.push_str(RULE);
    out.push('\n');
    out.push_str("Headroom: top compression opportunities in this session\n");
    for (tool, &count) in &top {
        let pct = (count * 100) / total.max(1);
        out.push_str(&format!(
            "  ▸ {tool:<22} {tokens:>6} tokens ({pct:>2}%)  — {hint}\n",
            tool = tool,
            tokens = human_tokens(count),
            pct = pct,
            hint = hint_for(tool),
        ));
    }
    out.push_str("  → Run `headroom xray replay` (coming soon) for exact savings.\n");
    out.push_str(RULE);
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_counts_returns_empty_string() {
        let counts = HashMap::new();
        assert_eq!(render(&counts), "");
    }

    #[test]
    fn renders_top_three_sorted_descending() {
        let mut counts = HashMap::new();
        counts.insert("Bash".to_string(), 53_000);
        counts.insert("Read".to_string(), 28_000);
        counts.insert("Grep".to_string(), 100);
        counts.insert("Edit".to_string(), 4_000);
        let r = render(&counts);
        let bash = r.find("Bash").unwrap();
        let read = r.find("Read").unwrap();
        let edit = r.find("Edit").unwrap();
        assert!(bash < read);
        assert!(read < edit);
        assert!(!r.contains("Grep"));
    }

    #[test]
    fn mcp_tool_hint() {
        assert_eq!(
            hint_for("mcp__codebase-memory-mcp__search_graph"),
            "MCP tool result; check schema re-injection"
        );
    }
}
