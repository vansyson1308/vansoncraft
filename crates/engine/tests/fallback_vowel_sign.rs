//! A cluster whose vowel sign is drawn before its consonant, in a fallback face: the glyph reported
//! for the consonant is the fallback face's glyph for it, laid out or overflowing, not the vowel
//! sign's.
//!
//! Its own test binary, so the process-wide font database starts with only the bundled fonts. The
//! fallback face for a character is the first loaded family that covers it, and any test that sets
//! type no loaded font covers (Georgian, say) loads a system font for it. Where that font also
//! covers Devanagari but has no Devanagari shaping (GNU Unifont), it came first and drew the vowel
//! sign after Ka: among the library's tests, this one failed whenever
//! `menucmds::change_case_leaves_georgian_uncapitalized` had run before it.
// Integration tests: unwrapping and panicking on failure is fine here, unlike in shipped code (AGENTS.md › Robustness).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use serde_json::json;
use vansoncraft_doc::{NodeId, NodeKind, TextObject};
use vansoncraft_engine::Session;
use vansoncraft_text::FontDb;
use vansoncraft_text::test_fonts::{MATRA_FAMILY, alternates_family, alternates_font, matra_font};

fn obj(s: &Session, id: NodeId) -> TextObject {
    match &s.doc().unwrap().doc.node(id).unwrap().kind {
        NodeKind::Text(t) => (**t).clone(),
        _ => panic!("not text"),
    }
}

/// A fallback face draws Ka with the vowel sign I in one cluster, the vowel sign first. The glyph
/// reported for Ka is the fallback face's glyph for Ka, laid out or overflowing, not the vowel
/// sign's.
#[test]
fn the_fallback_glyph_of_a_consonant_after_its_vowel_sign_is_reported() {
    let db = FontDb::global();
    db.add_font(alternates_font(true).expect("the test font builds"));
    db.add_font(matra_font().expect("the test font builds"));
    let requested = db.face(alternates_family(true), "Regular").expect("the test face");
    let text = "क\u{93F}";
    assert_eq!(requested.glyph_for('क'), 0);
    let mut s = Session::new();
    s.prefs.placeholder_text = false;
    s.execute("file.new", &json!({"width": 800, "height": 600})).unwrap();
    let create = |s: &mut Session, height: f64| {
        let spec = json!({"x": 0, "y": 0, "text": text, "font": alternates_family(true), "area": {"width": 200, "height": height}});
        NodeId(s.execute("text.create", &spec).unwrap()["id"].as_u64().unwrap())
    };
    let open = create(&mut s, 200.0);
    let glyphs = vansoncraft_text::layout(db, &obj(&s, open)).glyphs;
    let fallback = db.face_by_id(glyphs[0].font_id).expect("the fallback face");
    assert_ne!(fallback.id(), requested.id());
    let ka = fallback.glyph_for('क');
    let drawn: Vec<_> = glyphs.iter().map(|g| (g.gid, g.byte)).collect();
    // A face may draw the vowel sign with a glyph other than its own, so only its place is checked.
    // The fallback is the test face unless an installed Devanagari face is preferred (Nirmala UI,
    // Noto Sans Devanagari…), which shapes the cluster the same way.
    let sign_first = matches!(drawn[..], [(sign, 0), (k, 0)] if sign != 0 && sign != ka && k == ka);
    assert!(sign_first, "the vowel sign is drawn first, in Ka's cluster ({} or {MATRA_FAMILY}): {drawn:?}", fallback.family);
    let query = s.execute("text.glyphAlternates", &json!({"id": open.0, "start": 0})).unwrap();
    assert_eq!(query["chosen"], json!(ka), "the fallback face's glyph for Ka");

    let boxed = create(&mut s, 2.0);
    assert!(vansoncraft_text::layout(db, &obj(&s, boxed)).glyphs.is_empty(), "the text overflows");
    let query = s.execute("text.glyphAlternates", &json!({"id": boxed.0, "start": 0})).unwrap();
    assert_eq!(query["chosen"], json!(ka), "an overflowing Ka reports the fallback face's glyph for Ka");
}
