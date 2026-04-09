use multipass_aaak::{AaakBrief, AaakRecentRecord};

#[test]
fn render_includes_recent_records() {
    let brief = AaakBrief {
        ship: "/tmp/ship".to_string(),
        total_records: 3,
        wings: vec![("alpha".into(), 3)],
        rooms: vec![("src".into(), 2), ("general".into(), 1)],
        recent_records: vec![AaakRecentRecord {
            wing: "alpha".into(),
            room: "src".into(),
            preview: "auth flow updated".into(),
        }],
    };

    let rendered = brief.render();
    assert!(rendered.contains("AAAK wake-up"));
    assert!(rendered.contains("records: 3 across 1 wing(s) and 2 room(s)"));
    assert!(rendered.contains("[alpha/src] auth flow updated"));
}
