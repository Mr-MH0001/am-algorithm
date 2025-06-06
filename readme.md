# 🎌 Anime Title Matcher

A blazing-fast, ultra-accurate Rust utility for matching anime metadata — including English, Romaji, Native, and alternative titles — against messy or ambiguous inputs.

Built for anime apps, scrapers, and libraries that demand **precision** and **flexibility** when identifying shows.

---

## 🚀 Features

- 🧠 **Multiple Matching Strategies** – exact match, normalized match, fuzzy match, and year/episode tolerance.
- 🌍 **Multilingual Support** – English, Romaji, and Japanese title support out of the box.
- 🧬 **Heuristics + Fuzziness** – adaptive scoring ensures the most accurate result wins.
- 🧼 **Normalization Magic** – intelligent preprocessing cleans up bad formatting, version suffixes, and extra junk.
- 🦀 **Blazing Fast** – written in Rust for speed and safety.

---

## 🧪 Why It Works So Well

The matcher follows a layered, fault-tolerant strategy:

1. **Exact Match**
   - Direct string equality on preferred title fields.
   - Ultra fast and precise when inputs are clean.

2. **Normalized Title Match**
   - Cleans up titles: lowercased, punctuation/season tags removed, simplified spacing.
   - Matches even when formats differ (e.g., "Attack on Titan S1" → "Attack on Titan").

3. **Year/Episode Tolerance**
   - Smart fuzzy logic around expected `year` and `episode` counts to differentiate between remakes, seasons, or reboots.

4. **Fallback Fuzzy Matching**
   - Uses string similarity scoring to rank candidates when nothing else hits.
   - Prioritizes semantically closer titles over raw score alone.

These steps cascade, ensuring high precision with minimal false positives 💡

---

## 📦 Usage

```rust
let search = ExpectAnime::from_string_title("Attack on Titan", Some(2013), Some(25));
let result = find_best_match(&search, &anime_list, |anime| anime);

match result {
    Some(m) => println!("🎯 Found match: {:?}", m.title),
    None => println!("💔 No match found."),
}
````

You can inject a list of `ExpectAnime` objects using your preferred metadata source (Anilist, MAL, etc.).

---

## 📚 Example: Matching "Bleach TYBW"

**Input:**

```json
{ "title": "Bleach TYBW", "year": 2022, "episodes": 13 }
```

**Matched:**

```json
{ 
  "english": "Bleach: Thousand-Year Blood War",
  "romaji": "Bleach: Sennen Kessen-hen",
  "native": "BLEACH 千年血戦篇"
}
```

✔ Matched via `userPreferred` + normalized title
✔ Year and episode count used to disambiguate from other Bleach entries

---

## 🧠 Extensible Design

The matcher is designed to be plug-and-play with any data model. Just provide a closure to map your data into the matcher’s `ExpectAnime` structure. Easy\~ 💋

---

## 🛡 Accuracy by Design

The engine is:

* 🩷 **Strict where it matters** (title/season boundaries)
* 💜 **Flexible when needed** (typos, aliases, subs/dubs)
* 💙 **Tested across thousands of common edge cases** (especially Anilist/MAL discrepancies)

---

## 📄 Licensing & Porting

You are *totally welcome* to port or adapt this algorithm into another language like JavaScript, Python, Go, etc. I believe in open sharing! 💞

> 📝 **Please give credit** if you do —
> Drop a link back to my GitHub profile:
> [https://github.com/shimizudev](https://github.com/shimizudev)

Licensed under the MIT license.

---

## ✨ Credits

Built with love, logic, and way too many anime nights under the stars\~ 🌌