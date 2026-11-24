# Regenerates src/app/view/icons.rs:  pwsh tools/fluent-icons.ps1 -Out src/app/view/icons.rs
param([string]$Out)
# Downloads Fluent UI System Icons (20px, regular) and rewrites their SVG path data for XAML's
# path markup: every segment gets its own command letter (XAML does not repeat arc commands
# implicitly) and numbers are space separated.
$icons = [ordered]@{
  UNDO='arrow_undo'; REDO='arrow_redo'; BOLD='text_bold'; ITALIC='text_italic';
  STRIKETHROUGH='text_strikethrough'; HIGHLIGHT='highlight'; INLINE_CODE='braces';
  MATH='math_formula'; BULLETS='text_bullet_list_ltr'; NUMBERED='text_number_list_ltr';
  TASKS='task_list_ltr'; OUTDENT='text_indent_decrease_ltr'; INDENT='text_indent_increase_ltr';
  MARK_DONE='checkmark'; LINK='link'; PAGE_LINK='document_endnote'; TABLE='table_add';
  PICTURE='image'; VIDEO='video'; CODE_BLOCK='code_block'; DIVIDER='line_horizontal_1';
  QUOTE='text_quote'; VISUAL='eye'; SOURCE='code'; NAVIGATION='navigation'; FIND='search';
  REPLACE='text_grammar_arrow_right'; PROPERTIES='tag'; ZOOM_IN='zoom_in'; ZOOM_OUT='zoom_out';
  MORE='more_horizontal'
}
$args_per = @{ M=2; L=2; H=1; V=1; C=6; S=4; Q=4; T=2; A=7; Z=0 }

function Convert-Path([string]$d) {
  $inv = [System.Globalization.CultureInfo]::InvariantCulture
  $out = New-Object System.Text.StringBuilder
  $n = $d.Length
  function Skip { while ($script:i -lt $n -and ($d[$script:i] -eq ' ' -or $d[$script:i] -eq ',' -or $d[$script:i] -eq "`n")) { $script:i++ } }
  function ReadNumber {
    Skip
    $m = [regex]::Match($d.Substring($script:i), '^[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?')
    if (-not $m.Success) { throw "bad number at $($script:i) in $d" }
    $script:i += $m.Length
    return [double]::Parse($m.Value, $inv)
  }
  function ReadFlag { Skip; $c = $d[$script:i]; $script:i++; return [string]$c }
  function F([double]$v) { return $v.ToString('0.###', $inv) }
  $script:i = 0
  $cmd = $null; $x = 0.0; $y = 0.0; $sx = 0.0; $sy = 0.0; $open = $false
  while ($true) {
    Skip
    if ($script:i -ge $n) { break }
    $ch = $d[$script:i]
    if ($ch -match '[A-Za-z]') { $cmd = [string]$ch; $script:i++ }
    $rel = $cmd -cmatch '[a-z]'
    $up = $cmd.ToUpper()
    $ox = if ($rel) { $x } else { 0.0 }; $oy = if ($rel) { $y } else { 0.0 }
    switch ($up) {
      'Z' { [void]$out.Append('Z '); $x = $sx; $y = $sy; $open = $false; continue }
      'M' {
        if ($open) { [void]$out.Append('Z ') }
        $x = $ox + (ReadNumber); $y = $oy + (ReadNumber); $sx = $x; $sy = $y; $open = $true
        [void]$out.Append("M $(F $x) $(F $y) ")
        $cmd = if ($rel) { 'l' } else { 'L' }
      }
      'L' { $x = $ox + (ReadNumber); $y = $oy + (ReadNumber); [void]$out.Append("L $(F $x) $(F $y) ") }
      'H' { $x = $ox + (ReadNumber); [void]$out.Append("L $(F $x) $(F $y) ") }
      'V' { $y = $oy + (ReadNumber); [void]$out.Append("L $(F $x) $(F $y) ") }
      'C' {
        $a = @(); for ($k = 0; $k -lt 3; $k++) { $a += ($ox + (ReadNumber)); $a += ($oy + (ReadNumber)) }
        $x = $a[4]; $y = $a[5]
        [void]$out.Append("C " + (($a | ForEach-Object { F $_ }) -join ' ') + ' ')
      }
      'S' {
        $a = @(); for ($k = 0; $k -lt 2; $k++) { $a += ($ox + (ReadNumber)); $a += ($oy + (ReadNumber)) }
        $x = $a[2]; $y = $a[3]
        [void]$out.Append("S " + (($a | ForEach-Object { F $_ }) -join ' ') + ' ')
      }
      'Q' {
        $a = @(); for ($k = 0; $k -lt 2; $k++) { $a += ($ox + (ReadNumber)); $a += ($oy + (ReadNumber)) }
        $x = $a[2]; $y = $a[3]
        [void]$out.Append("Q " + (($a | ForEach-Object { F $_ }) -join ' ') + ' ')
      }
      'T' { $x = $ox + (ReadNumber); $y = $oy + (ReadNumber); [void]$out.Append("T $(F $x) $(F $y) ") }
      'A' {
        $rx = ReadNumber; $ry = ReadNumber; $rot = ReadNumber; $large = ReadFlag; $sweep = ReadFlag
        $x = $ox + (ReadNumber); $y = $oy + (ReadNumber)
        [void]$out.Append("A $(F $rx) $(F $ry) $(F $rot) $large $sweep $(F $x) $(F $y) ")
      }
    }
  }
  if ($open) { [void]$out.Append('Z') }
  return $out.ToString().Trim()
}

$lines = @('//! Fluent UI System Icons (20px, regular), as XAML path markup for `PathIcon`.',
  '//! Generated from @fluentui/svg-icons 1.1.341, rewritten in absolute coordinates with every',
  '//! figure closed: WinUI misdraws some relative and implicitly repeated commands. "F1" is the',
  '//! nonzero fill rule SVG uses. Regenerate with tools/fluent-icons.ps1.',
  '')
foreach ($name in $icons.Keys) {
  $slug = $icons[$name]
  $svg = (Invoke-WebRequest "https://cdn.jsdelivr.net/npm/@fluentui/svg-icons@1.1.341/icons/${slug}_20_regular.svg" -UseBasicParsing).Content
  $paths = [regex]::Matches($svg, ' d="([^"]+)"') | ForEach-Object { Convert-Path $_.Groups[1].Value }
  $data = 'F1 ' + ($paths -join ' ')
  $lines += "/// Fluent ``$slug``."
  $lines += "pub const ${name}: &str = `"$data`";"
}
$lines -join "`n" | Set-Content -Path $Out -Encoding utf8NoBOM
"wrote $($icons.Count) icons"
