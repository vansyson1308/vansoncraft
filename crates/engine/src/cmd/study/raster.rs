//! The studies that look at the rendered page: `study.render`, `study.values`, `study.balance`.
//!
//! The region is drawn as a PNG export draws it, on white (linked images and placed documents
//! from their files), and each pixel's lightness is CIE L* (D65 white, sRGB primaries): the
//! standard measure of how light a colour looks, 0 for black and 100 for white. Value groups are
//! bands of L*; the defaults split 0–100 into equal steps.

use serde_json::{Value, json};
use vectorcraft_geom::{Affine, Rect};
use vectorcraft_render::Rendered;
use vectorcraft_render::encode::{RasterExportOptions, RasterFormat};

use super::{flag, num, r3, whole};
use crate::cmd::bad;
use crate::{EngineError, Result, Session};

/// Most pixels a study renders (4096 × 4096).
const MAX_PIXELS: u64 = 4096 * 4096;

/// A rendered region as L* per pixel, with what it was drawn from.
pub(super) struct Lightness {
    pub width: u32,
    pub height: u32,
    pub l: Vec<f32>,
    pub region: Rect,
    pub scale: f64,
    /// The opaque RGBA pixels as drawn (straight alpha; every pixel is opaque on the white).
    pub rgba: Vec<u8>,
}

/// The region a study looks at: `region` when given, else the artboard (`artboard`, default 0).
fn region_param(s: &Session, p: &Value, c: &str) -> Result<Rect> {
    let doc = &s.doc()?.doc;
    if let Some(v) = p.get("region").filter(|v| !v.is_null()) {
        let a = v.as_array().filter(|a| a.len() == 4).ok_or_else(|| bad(c, "`region` must be [x, y, width, height]"))?;
        let n: Vec<f64> = a.iter().filter_map(Value::as_f64).filter(|x| x.is_finite() && x.abs() <= crate::MAX_COORD).collect();
        let [x, y, w, h] = n.as_slice() else { return Err(bad(c, "`region` must be four finite numbers")) };
        if *w <= 0.0 || *h <= 0.0 {
            return Err(bad(c, "`region` must have a positive width and height"));
        }
        return Ok(Rect::new(*x, *y, x + w, y + h));
    }
    let i = whole(p, "artboard", 0, 0, u64::from(u32::MAX), c)?;
    let count = doc.artboards.len();
    usize::try_from(i)
        .ok()
        .and_then(|i| doc.artboards.get(i))
        .map(|a| a.rect)
        .ok_or_else(|| bad(c, format!("no artboard {i} (0-based; the document has {count})")))
}

/// The scale a study renders at: `scale` when given, else the region fitted to `maxSide`.
fn scale_param(p: &Value, region: Rect, default_side: u64, max_side: u64, c: &str) -> Result<f64> {
    let side = whole(p, "maxSide", default_side, 16, max_side, c)? as f64;
    let scale = match p.get("scale").filter(|v| !v.is_null()) {
        Some(_) => num(p, "scale", 1.0, 0.01, 64.0, c)?,
        None => (side / region.width().max(region.height())).min(64.0),
    };
    // A scale fitted to `side` lands on it up to rounding (841.89 × 1024 / 841.89 can be 1024.0000001):
    // a hair over is not over.
    let (w, h) = ((region.width() * scale - 1e-6).ceil(), (region.height() * scale - 1e-6).ceil());
    if !(w.is_finite() && h.is_finite()) || w > side || h > side || (w * h) as u64 > MAX_PIXELS {
        return Err(bad(c, format!("the region at scale {scale} is larger than {side} pixels a side: lower `scale` or raise `maxSide`")));
    }
    Ok(scale)
}

/// The region `p` names, rendered on white and measured as L*.
pub(super) fn lightness(s: &Session, p: &Value, c: &str, default_side: u64, max_side: u64) -> Result<Lightness> {
    let region = region_param(s, p, c)?;
    let scale = scale_param(p, region, default_side, max_side, c)?;
    let show_templates = flag(p, "showTemplates", c)?;
    let (w, h) = vectorcraft_render::raster_size(region, scale).map_err(|e| bad(c, e))?;
    let doc = &s.doc()?.doc;
    let mut opts = RasterExportOptions { background: Some([255; 3]), ..Default::default() }.render_options(RasterFormat::Png);
    if show_templates {
        opts.skip_templates = false;
        opts.dim_templates = true;
    }
    let (full, _) = crate::cmd::place::document::full_documents(doc);
    let view = Affine::scale(scale) * Affine::translate((-region.x0, -region.y0));
    let img = vectorcraft_render::Renderer::new().render(&full, w, h, view, &opts);
    // A cancelled render stops drawing: the picture is not whole.
    vectorcraft_doc::cancel::check()?;
    let rgba = img.to_straight();
    let lut = linear_lut();
    let l = rgba.as_chunks::<4>().0.iter().map(|px| lstar(&lut, px)).collect();
    Ok(Lightness { width: img.width, height: img.height, l, region, scale, rgba })
}

/// sRGB 8-bit → linear light.
fn linear_lut() -> [f32; 256] {
    let mut t = [0.0f32; 256];
    for (i, v) in t.iter_mut().enumerate() {
        let c = i as f32 / 255.0;
        *v = if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    }
    t
}

/// CIE L* of an opaque sRGB pixel (Rec. 709 luminance; D65 white).
fn lstar(lut: &[f32; 256], px: &[u8; 4]) -> f32 {
    let [r, g, b, _] = px.map(|v| lut[usize::from(v)]);
    let y = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let f = if y > 0.008_856 { y.cbrt() } else { y * 7.787 + 16.0 / 116.0 };
    (116.0 * f - 16.0).clamp(0.0, 100.0)
}

/// The sRGB grey (0–255) whose L* is `l`.
fn grey_of(l: f32) -> u8 {
    let f = (l + 16.0) / 116.0;
    let y = if f > 0.206_893 { f * f * f } else { (f - 16.0 / 116.0) / 7.787 };
    let c = if y <= 0.003_130_8 { 12.92 * y } else { 1.055 * y.powf(1.0 / 2.4) - 0.055 };
    (c.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// The value-group boundaries: `thresholds` when given (rising, inside 0–100, `levels − 1` of
/// them), else equal steps of L*.
fn thresholds(p: &Value, levels: usize, c: &str) -> Result<Vec<f32>> {
    match p.get("thresholds").filter(|v| !v.is_null()) {
        None => Ok((1..levels).map(|k| (100.0 * k as f64 / levels as f64) as f32).collect()),
        Some(v) => {
            let a = v.as_array().ok_or_else(|| bad(c, "`thresholds` must be a list of L* values"))?;
            let t: Vec<f32> = a.iter().filter_map(Value::as_f64).filter(|x| x.is_finite() && *x > 0.0 && *x < 100.0).map(|x| x as f32).collect();
            if t.len() != a.len() || t.len() + 1 != levels || t.windows(2).any(|w| w.first() >= w.get(1)) {
                return Err(bad(c, format!("`thresholds` must be {} rising L* values between 0 and 100 (one fewer than `levels`)", levels - 1)));
            }
            Ok(t)
        }
    }
}

/// The value group (0: darkest) of each pixel.
fn groups_of(l: &[f32], t: &[f32]) -> Vec<u8> {
    l.iter().map(|v| t.iter().filter(|b| *v >= **b).count() as u8).collect()
}

/// The names of `n` value groups, darkest first.
fn names(n: usize) -> &'static [&'static str] {
    match n {
        2 => &["dark", "light"],
        3 => &["dark", "middle", "light"],
        4 => &["dark", "low middle", "high middle", "light"],
        _ => &["dark", "low middle", "middle", "high middle", "light"],
    }
}

/// Each group's connected patches (8-connected): (patches of at least `min` pixels, largest).
fn masses(group: &[u8], width: usize, height: usize, levels: usize, min: usize) -> Vec<(usize, usize)> {
    let mut out = vec![(0usize, 0usize); levels];
    let mut seen = vec![false; group.len()];
    let mut stack: Vec<usize> = Vec::new();
    for start in 0..group.len() {
        if seen.get(start).copied().unwrap_or(true) {
            continue;
        }
        let g = group.get(start).copied().unwrap_or(0);
        if let Some(s) = seen.get_mut(start) {
            *s = true;
        }
        stack.push(start);
        let mut size = 0usize;
        while let Some(i) = stack.pop() {
            size += 1;
            let (x, y) = (i % width, i / width);
            for (dx, dy) in [(-1i64, -1i64), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                if nx < 0 || ny < 0 || nx >= width as i64 || ny >= height as i64 {
                    continue;
                }
                let j = ny as usize * width + nx as usize;
                if group.get(j) == Some(&g) && seen.get(j) == Some(&false) {
                    if let Some(s) = seen.get_mut(j) {
                        *s = true;
                    }
                    stack.push(j);
                }
            }
        }
        if let Some(e) = out.get_mut(usize::from(g)) {
            if size >= min {
                e.0 += 1;
            }
            e.1 = e.1.max(size);
        }
    }
    out
}

/// A PNG of `width` × `height` grey or RGB pixels, as base64.
fn png_base64(width: u32, height: u32, rgba: Vec<u8>) -> Result<String> {
    let png = Rendered { width, height, pixels: rgba }.to_png().map_err(EngineError::Other)?;
    Ok(vectorcraft_format::base64_encode(&png))
}

fn region_json(r: Rect) -> Value {
    json!([r3(r.x0), r3(r.y0), r3(r.width()), r3(r.height())])
}

/// `study.render`.
pub(super) fn render(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.render";
    let mode = match p.get("mode").filter(|v| !v.is_null()) {
        None => "color",
        Some(Value::String(m)) if ["color", "gray", "values"].contains(&m.as_str()) => m.as_str(),
        Some(_) => return Err(bad(C, "`mode` must be \"color\", \"gray\" or \"values\"")),
    };
    let flip = match p.get("flip").filter(|v| !v.is_null()) {
        None => (false, false),
        Some(Value::String(f)) => match f.as_str() {
            "none" => (false, false),
            "horizontal" => (true, false),
            "vertical" => (false, true),
            "both" => (true, true),
            _ => return Err(bad(C, "`flip` must be \"none\", \"horizontal\", \"vertical\" or \"both\"")),
        },
        Some(_) => return Err(bad(C, "`flip` must be a string")),
    };
    let levels = whole(p, "levels", 3, 2, 5, C)? as usize;
    // Checked before the render, so a bad value fails fast.
    let t = thresholds(p, levels, C)?;
    let lt = lightness(s, p, C, 1024, 4096)?;
    let mut out = json!({"width": lt.width, "height": lt.height, "scale": r3(lt.scale), "region": region_json(lt.region), "mode": mode});
    let pixels = match mode {
        "gray" => {
            lt.l.iter()
                .flat_map(|v| {
                    let g = grey_of(*v);
                    [g, g, g, 255]
                })
                .collect()
        }
        "values" => {
            let g = groups_of(&lt.l, &t);
            let means = group_means(&lt.l, &g, levels);
            out["thresholds"] = json!(t);
            g.iter()
                .flat_map(|k| {
                    let v = grey_of(means.get(usize::from(*k)).copied().unwrap_or(50.0));
                    [v, v, v, 255]
                })
                .collect()
        }
        _ => lt.rgba,
    };
    let pixels = flipped(pixels, lt.width as usize, flip);
    if flip != (false, false) {
        out["flip"] = p.get("flip").cloned().unwrap_or(Value::Null);
    }
    out["pngBase64"] = json!(png_base64(lt.width, lt.height, pixels)?);
    Ok(out)
}

/// RGBA rows mirrored left to right (`h`) and top to bottom (`v`); both is a half turn.
fn flipped(px: Vec<u8>, width: usize, (h, v): (bool, bool)) -> Vec<u8> {
    if !h && !v || width == 0 {
        return px;
    }
    let mut rows: Vec<&[u8]> = px.chunks(width * 4).collect();
    if v {
        rows.reverse();
    }
    let mut out = Vec::with_capacity(px.len());
    for row in rows {
        if h {
            for p in row.chunks(4).rev() {
                out.extend_from_slice(p);
            }
        } else {
            out.extend_from_slice(row);
        }
    }
    out
}

/// Each group's mean L* (its threshold band's middle when it is empty).
fn group_means(l: &[f32], g: &[u8], levels: usize) -> Vec<f32> {
    let mut sum = vec![(0.0f64, 0usize); levels];
    for (v, k) in l.iter().zip(g) {
        if let Some(e) = sum.get_mut(usize::from(*k)) {
            e.0 += f64::from(*v);
            e.1 += 1;
        }
    }
    sum.iter()
        .enumerate()
        .map(|(i, (s, n))| if *n > 0 { (*s / *n as f64) as f32 } else { (100.0 * (i as f64 + 0.5) / levels as f64) as f32 })
        .collect()
}

/// `study.values`.
pub(super) fn values(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.values";
    let levels = whole(p, "levels", 3, 2, 5, C)? as usize;
    let t = thresholds(p, levels, C)?;
    let min_mass = num(p, "minMass", 0.002, 0.0, 1.0, C)?;
    let image = flag(p, "image", C)?;
    let lt = lightness(s, p, C, 512, 2048)?;
    let (w, h) = (lt.width as usize, lt.height as usize);
    let total = lt.l.len().max(1);
    let g = groups_of(&lt.l, &t);
    let means = group_means(&lt.l, &g, levels);
    let min = ((min_mass * total as f64).ceil() as usize).max(1);
    let patches = masses(&g, w, h, levels, min);
    let mut counts = vec![0usize; levels];
    for k in &g {
        if let Some(c) = counts.get_mut(usize::from(*k)) {
            *c += 1;
        }
    }
    let label = names(levels);
    let groups: Vec<Value> = (0..levels)
        .map(|i| {
            let n = counts.get(i).copied().unwrap_or(0);
            let (m, largest) = patches.get(i).copied().unwrap_or((0, 0));
            json!({
                "level": i,
                "name": label.get(i).copied().unwrap_or(""),
                "share": r3(n as f64 / total as f64),
                "meanL": if n > 0 { json!(r3(f64::from(means.get(i).copied().unwrap_or(0.0)))) } else { Value::Null },
                "masses": m,
                "largestShare": if n > 0 { r3(largest as f64 / n as f64) } else { 0.0 },
            })
        })
        .collect();
    let present: Vec<f64> =
        (0..levels).filter(|i| counts.get(*i).is_some_and(|n| *n > 0)).filter_map(|i| means.get(i).map(|m| f64::from(*m))).collect();
    let contrast = match (present.first(), present.last()) {
        (Some(d), Some(l)) => l - d,
        _ => 0.0,
    };
    let ex = extremes(&lt.l);
    let mut out = json!({
        "width": lt.width,
        "height": lt.height,
        "region": region_json(lt.region),
        "thresholds": t,
        "extremes": ex,
        "groups": groups,
        "contrast": r3(contrast),
        "method": "rendered as a PNG export on white; each pixel's CIE L* (sRGB, D65) put in a band between the thresholds; masses are 8-connected patches of one band of at least minMass of the pixels. A measure of the picture's tones, not of what the artist meant by them.",
    });
    if image {
        let px = g.iter().flat_map(|k| {
            let v = grey_of(means.get(usize::from(*k)).copied().unwrap_or(50.0));
            [v, v, v, 255]
        });
        out["pngBase64"] = json!(png_base64(lt.width, lt.height, px.collect())?);
    }
    Ok(out)
}

/// The ends of the tone scale in use: the 1st and 99th percentile of L*, how far their middle sits
/// from L* 50, and the share of pure black or white (L* below 2 or above 98).
fn extremes(l: &[f32]) -> Value {
    let mut hist = [0usize; 101];
    for v in l {
        if let Some(h) = hist.get_mut(v.round().clamp(0.0, 100.0) as usize) {
            *h += 1;
        }
    }
    let n = l.len().max(1);
    let pct = |q: f64| {
        let want = (q * n as f64).ceil() as usize;
        let mut seen = 0;
        hist.iter()
            .position(|h| {
                seen += h;
                seen >= want.max(1)
            })
            .unwrap_or(0) as f64
    };
    let (dark, light) = (pct(0.01), pct(0.99));
    let pure = l.iter().filter(|v| **v < 2.0 || **v > 98.0).count();
    json!({"darkL": dark, "lightL": light, "midOffset": r3((dark + light) / 2.0 - 50.0), "pureShare": r3(pure as f64 / n as f64)})
}

/// The median L* of the region (to the nearest unit).
fn median(l: &[f32]) -> f32 {
    let mut hist = [0usize; 101];
    for v in l {
        if let Some(h) = hist.get_mut(v.round().clamp(0.0, 100.0) as usize) {
            *h += 1;
        }
    }
    let half = l.len().div_ceil(2);
    let mut seen = 0;
    hist.iter()
        .position(|h| {
            seen += h;
            seen >= half.max(1)
        })
        .unwrap_or(100) as f32
}

/// `study.balance`.
pub(super) fn balance(s: &mut Session, p: &Value) -> Result<Value> {
    const C: &str = "study.balance";
    // darkness: dark on white weighs; contrast: what differs from the ground (the median tone), as on
    // a toned ground, where darkness alone would weigh the ground itself.
    let contrast = match p.get("weigh").filter(|v| !v.is_null()) {
        None => false,
        Some(Value::String(w)) if w == "darkness" => false,
        Some(Value::String(w)) if w == "contrast" => true,
        Some(_) => return Err(bad(C, "`weigh` must be \"darkness\" or \"contrast\"")),
    };
    let lt = lightness(s, p, C, 512, 2048)?;
    let ground = median(&lt.l);
    let (w, h) = (lt.width as usize, lt.height as usize);
    let (mut sum, mut sx, mut sy) = (0.0f64, 0.0f64, 0.0f64);
    let mut quad = [0.0f64; 4];
    let mut empty = 0usize;
    let mut on_ground = 0usize;
    for (i, l) in lt.l.iter().enumerate() {
        if (*l - ground).abs() <= 5.0 {
            on_ground += 1;
        }
        let weight = if contrast { f64::from((*l - ground).abs()) / 100.0 } else { f64::from(100.0 - *l) / 100.0 };
        if *l >= 90.0 {
            empty += 1;
        }
        if weight <= 0.0 {
            continue;
        }
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        sum += weight;
        sx += weight * x;
        sy += weight * y;
        let q = usize::from(x >= w as f64 / 2.0) + 2 * usize::from(y >= h as f64 / 2.0);
        if let Some(e) = quad.get_mut(q) {
            *e += weight;
        }
    }
    let r = lt.region;
    let total = lt.l.len().max(1) as f64;
    let (centroid, offset) = if sum > 0.0 {
        let (cx, cy) = (r.x0 + sx / sum / lt.scale, r.y0 + sy / sum / lt.scale);
        let c = r.center();
        (json!({"x": r3(cx), "y": r3(cy)}), json!({"x": r3((cx - c.x) / (r.width() / 2.0)), "y": r3((cy - c.y) / (r.height() / 2.0))}))
    } else {
        (Value::Null, Value::Null)
    };
    let share = |q: usize| if sum > 0.0 { r3(quad.get(q).copied().unwrap_or(0.0) / sum) } else { 0.0 };
    Ok(json!({
        "region": region_json(r),
        "centroid": centroid,
        "offset": offset,
        "quadrants": {"topLeft": share(0), "topRight": share(1), "bottomLeft": share(2), "bottomRight": share(3)},
        "emptyShare": r3(empty as f64 / total),
        "weigh": if contrast { "contrast" } else { "darkness" },
        "ground": {"L": ground, "share": r3(on_ground as f64 / total)},
        "method": "each pixel of the region, rendered on white, weighs its darkness (100 − L*) / 100, or with weigh contrast its difference from the median tone |L* − ground| / 100 (use that on a toned ground); colour and subject are not weighed. Where a viewer looks first depends on more than this: read it as one measure of how the tones are spread, not as the picture's balance.",
    }))
}

#[cfg(test)]
pub(super) fn lstar_of_rgb(r: u8, g: u8, b: u8) -> f32 {
    lstar(&linear_lut(), &[r, g, b, 255])
}

#[cfg(test)]
pub(super) fn grey_for(l: f32) -> u8 {
    grey_of(l)
}
