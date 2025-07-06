use crate::ascii_chars::AsciiChars;

pub struct SvgKeyboard {}

impl SvgKeyboard {
    pub fn new() -> Self {
        SvgKeyboard {}
    }

    pub const fn render() -> &'static str {
        // This generates an image of a standard QWERTY keyboard layout
        // with the specified key(s) highlighted - either key, or
        // SHIFT + key.
        //let shift_needed = shift_key_needed(&key_char);

        &SVG_KEYBOARD
    }

    pub fn get_css_ids(key_char: &AsciiChars) -> (String, Option<&str>) {
        // This returns the CSS IDs for the key(s) to highlight.
        // If shift is needed, it returns both the key and the shifted key.
        let shift_needed = shift_key_needed(key_char);
        let css_id = map_ascii_to_key_id(key_char);
        if shift_needed {
            (css_id, Some("key-shift-left"))
        } else {
            (css_id, None)
        }
    }
}

fn shift_key_needed(key_char: &AsciiChars) -> bool {
    let c = key_char.as_char();
    match c {
        'A'..='Z' => true,
        '<' | '>' | '?' | '~' | '!' | '@' | '#' | '$' | '%' | '^' | '&' | '*' | '(' | ')' | '_'
        | '+' => true,
        _ => false,
    }
}

fn map_ascii_to_key_id(key_char: &AsciiChars) -> String {
    // This maps the ASCII character to the corresponding key ID in the SVG.
    // For example, 'A' maps to 'key-a', '1' maps to 'key-1', etc.
    let c = key_char.as_char();
    match c {
        'A'..='Z' => format!("key-{}", c.to_ascii_lowercase()),
        'a'..='z' => format!("key-{}", c),
        '0'..='9' => format!("key-{}", c),
        '-' => "key-minus".to_string(),
        '=' => "key-equals".to_string(),
        '[' | '{' => "key-l-bracket".to_string(),
        ']' | '}' => "key-r-bracket".to_string(),
        '\\' | '|' => "key-backslash".to_string(),
        ';' | ':' => "key-semicolon".to_string(),
        '\'' | '"' => "key-s-quote".to_string(),
        ',' | '<' => "key-comma".to_string(),
        '.' | '>' => "key-period".to_string(),
        '/' | '?' => "key-slash".to_string(),
        '`' | '~' => "key-tilde".to_string(),
        '!' => "key-1".to_string(),
        '@' => "key-2".to_string(),
        '#' => "key-3".to_string(),
        '$' => "key-4".to_string(),
        '%' => "key-5".to_string(),
        '^' => "key-6".to_string(),
        '&' => "key-7".to_string(),
        '*' => "key-8".to_string(),
        '(' => "key-9".to_string(),
        ')' => "key-0".to_string(),
        '_' => "key-minus".to_string(),
        '+' => "key-equals".to_string(),

        _ => String::new(), // Return an empty string for unsupported characters
    }
}

const SVG_KEYBOARD: &'static str = r###"
<svg viewBox="0 0 710 240" xmlns="http://www.w3.org/2000/svg" width="100%" height="auto" preserveAspectRatio="xMidYMid meet">
  <defs>
    <linearGradient id="keyGrad" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0%" stop-color="#e0e0e0" />
      <stop offset="100%" stop-color="#a0a0a0" />
    </linearGradient>
  </defs>
<g id="key-tilde">
  <rect x="10" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="32.5" y="22" font-size="10" text-anchor="middle">~</text>
  <text x="32.5" y="42" font-size="14" text-anchor="middle">`</text>
</g>
<g id="key-1">
  <rect x="57" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="79.5" y="22" font-size="10" text-anchor="middle">!</text>
  <text x="79.5" y="42" font-size="14" text-anchor="middle">1</text>
</g>
<g id="key-2">
  <rect x="104" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="126.5" y="22" font-size="10" text-anchor="middle">@</text>
  <text x="126.5" y="42" font-size="14" text-anchor="middle">2</text>
</g>
<g id="key-3">
  <rect x="151" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="173.5" y="22" font-size="10" text-anchor="middle">#</text>
  <text x="173.5" y="42" font-size="14" text-anchor="middle">3</text>
</g>
<g id="key-4">
  <rect x="198" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="220.5" y="22" font-size="10" text-anchor="middle">$</text>
  <text x="220.5" y="42" font-size="14" text-anchor="middle">4</text>
</g>
<g id="key-5">
  <rect x="245" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="267.5" y="22" font-size="10" text-anchor="middle">%</text>
  <text x="267.5" y="42" font-size="14" text-anchor="middle">5</text>
</g>
<g id="key-6">
  <rect x="292" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="314.5" y="22" font-size="10" text-anchor="middle">^</text>
  <text x="314.5" y="42" font-size="14" text-anchor="middle">6</text>
</g>
<g id="key-7">
  <rect x="339" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="361.5" y="22" font-size="10" text-anchor="middle">&amp;</text>
  <text x="361.5" y="42" font-size="14" text-anchor="middle">7</text>
</g>
<g id="key-8">
  <rect x="386" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="408.5" y="22" font-size="10" text-anchor="middle">*</text>
  <text x="408.5" y="42" font-size="14" text-anchor="middle">8</text>
</g>
<g id="key-9">
  <rect x="433" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="455.5" y="22" font-size="10" text-anchor="middle">(</text>
  <text x="455.5" y="42" font-size="14" text-anchor="middle">9</text>
</g>
<g id="key-0">
  <rect x="480" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="502.5" y="22" font-size="10" text-anchor="middle">)</text>
  <text x="502.5" y="42" font-size="14" text-anchor="middle">0</text>
</g>
<g id="key-minus">
  <rect x="527" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="549.5" y="22" font-size="10" text-anchor="middle">_</text>
  <text x="549.5" y="42" font-size="14" text-anchor="middle">-</text>
</g>
<g id="key-equals">
  <rect x="574" y="10" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="596.5" y="22" font-size="10" text-anchor="middle">+</text>
  <text x="596.5" y="42" font-size="14" text-anchor="middle">=</text>
</g>
<g id="key-back">
  <rect x="621" y="10" width="80" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="661.0" y="35.0" font-size="14" text-anchor="middle">Back</text>
</g>
<g id="key-tab">
  <rect x="10" y="55" width="65" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="42.5" y="80.0" font-size="14" text-anchor="middle">Tab</text>
</g>
<g id="key-q">
  <rect x="77" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="99.5" y="80.0" font-size="14" text-anchor="middle">Q</text>
</g>
<g id="key-w">
  <rect x="124" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="146.5" y="80.0" font-size="14" text-anchor="middle">W</text>
</g>
<g id="key-e">
  <rect x="171" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="193.5" y="80.0" font-size="14" text-anchor="middle">E</text>
</g>
<g id="key-r">
  <rect x="218" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="240.5" y="80.0" font-size="14" text-anchor="middle">R</text>
</g>
<g id="key-t">
  <rect x="265" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="287.5" y="80.0" font-size="14" text-anchor="middle">T</text>
</g>
<g id="key-y">
  <rect x="312" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="334.5" y="80.0" font-size="14" text-anchor="middle">Y</text>
</g>
<g id="key-u">
<rect x="359" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
<text x="381.5" y="80.0" font-size="14" text-anchor="middle">U</text>
</g>
<g id="key-i">
  <rect x="406" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="428.5" y="80.0" font-size="14" text-anchor="middle">I</text>
</g>
<g id="key-o">
  <rect x="453" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="475.5" y="80.0" font-size="14" text-anchor="middle">O</text>
</g>
<g id="key-p">
  <rect x="500" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="522.5" y="80.0" font-size="14" text-anchor="middle">P</text>
</g>
<g id="key-l-bracket">
  <rect x="547" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="569.5" y="67" font-size="10" text-anchor="middle">{</text>
  <text x="569.5" y="87" font-size="14" text-anchor="middle">[</text>
</g>
<g id="key-r-bracket">
  <rect x="594" y="55" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="616.5" y="67" font-size="10" text-anchor="middle">}</text>
  <text x="616.5" y="87" font-size="14" text-anchor="middle">]</text>
</g>
<g id="key-backslash">
  <rect x="641" y="55" width="59" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="670.5" y="67" font-size="10" text-anchor="middle">|</text>
  <text x="670.5" y="87" font-size="14" text-anchor="middle">\</text>
</g>
<g id="key-caps">
  <rect x="10" y="100" width="75" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="47.5" y="125.0" font-size="14" text-anchor="middle">Caps</text>
</g>
<g id="key-a">
  <rect x="87" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="109.5" y="125.0" font-size="14" text-anchor="middle">A</text>
</g>
<g id="key-s">
  <rect x="134" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="156.5" y="125.0" font-size="14" text-anchor="middle">S</text>
</g>
<g id="key-d">
  <rect x="181" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="203.5" y="125.0" font-size="14" text-anchor="middle">D</text>
</g>
<g id="key-f">
  <rect x="228" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="250.5" y="125.0" font-size="14" text-anchor="middle">F</text>
</g>
<g id="key-g">
  <rect x="275" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="297.5" y="125.0" font-size="14" text-anchor="middle">G</text>
</g>
<g id="key-h">
  <rect x="322" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="344.5" y="125.0" font-size="14" text-anchor="middle">H</text>
</g>
<g id="key-j">
  <rect x="369" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="391.5" y="125.0" font-size="14" text-anchor="middle">J</text>
</g>
<g id="key-k">
  <rect x="416" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="438.5" y="125.0" font-size="14" text-anchor="middle">K</text>
</g>
<g id="key-l">
  <rect x="463" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="485.5" y="125.0" font-size="14" text-anchor="middle">L</text>
</g>
<g id="key-semicolon">
  <rect x="510" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="532.5" y="112" font-size="10" text-anchor="middle">:</text>
  <text x="532.5" y="132" font-size="14" text-anchor="middle">;</text>
</g>
<g id="key-s-quote">
  <rect x="557" y="100" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="579.5" y="112" font-size="10" text-anchor="middle">&quot;</text>
  <text x="579.5" y="132" font-size="14" text-anchor="middle">'</text>
</g>
<g id="key-enter">
  <rect x="604" y="100" width="96" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="652.0" y="125.0" font-size="14" text-anchor="middle">Enter</text>
</g>
<g id="key-shift-left">
  <rect x="10" y="145" width="90" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="55.0" y="170.0" font-size="14" text-anchor="middle">Shift</text>
</g>
<g id="key-z">
  <rect x="102" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="124.5" y="170.0" font-size="14" text-anchor="middle">Z</text>
</g>
<g id="key-x">
  <rect x="149" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="171.5" y="170.0" font-size="14" text-anchor="middle">X</text>
</g>
<g id="key-c">
  <rect x="196" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="218.5" y="170.0" font-size="14" text-anchor="middle">C</text>
</g>
<g id="key-v">
  <rect x="243" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="265.5" y="170.0" font-size="14" text-anchor="middle">V</text>
</g>
<g id="key-b">
  <rect x="290" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="312.5" y="170.0" font-size="14" text-anchor="middle">B</text>
</g>
<g id="key-n">
  <rect x="337" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="359.5" y="170.0" font-size="14" text-anchor="middle">N</text>
</g>
<g id="key-m">
  <rect x="384" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="406.5" y="170.0" font-size="14" text-anchor="middle">M</text>
</g>
<g id="key-comma">
  <rect x="431" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="453.5" y="157" font-size="10" text-anchor="middle">&lt;</text>
  <text x="453.5" y="177" font-size="14" text-anchor="middle">,</text>
</g>
<g id="key-period">
  <rect x="478" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="500.5" y="157" font-size="10" text-anchor="middle">&gt;</text>
  <text x="500.5" y="177" font-size="14" text-anchor="middle">.</text>
</g>
<g id="key-slash">
  <rect x="525" y="145" width="45" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="547.5" y="157" font-size="10" text-anchor="middle">?</text>
  <text x="547.5" y="177" font-size="14" text-anchor="middle">/</text>
</g>
<g id="key-shift-right">
  <rect x="572" y="145" width="128" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="636.0" y="170.0" font-size="14" text-anchor="middle">Shift</text>
</g>
<g id="key-ctrl-left">
  <rect x="10" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="40.0" y="215.0" font-size="14" text-anchor="middle">Ctrl</text>
</g>
<g id="key-win-left">
  <rect x="72" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="102.0" y="215.0" font-size="14" text-anchor="middle">Win</text>
</g>
<g id="key-alt-left">
  <rect x="134" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="164.0" y="215.0" font-size="14" text-anchor="middle">Alt</text>
</g>
<g id="key-space">
  <rect x="196" y="190" width="256" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="324.0" y="215.0" font-size="14" text-anchor="middle">Space</text>
</g>
<g id="key-alt-right">
  <rect x="454" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="484.0" y="215.0" font-size="14" text-anchor="middle">Alt</text>
</g>
<g id="key-fn">
  <rect x="516" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="546.0" y="215.0" font-size="14" text-anchor="middle">FN</text>
</g>
<g id="key-menu">
  <rect x="578" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="608.0" y="215.0" font-size="14" text-anchor="middle">Menu</text>
</g>
<g id="key-ctrl-right">
  <rect x="640" y="190" width="60" height="40" rx="6" fill="url(#keyGrad)" stroke="black" />
  <text x="670.0" y="215.0" font-size="14" text-anchor="middle">Ctrl</text>
</g>
</svg>
"###;
