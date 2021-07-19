<table class="wikitable">
<tbody><tr>
<th></th>
<th>Name</th>
<th>Note
</th></tr>
<tr>
<td style="width:15%"><span class="monospaced">0</span>
</td>
<td>Reset <i>or</i> normal
</td>
<td>All attributes off
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">1</span>
</td>
<td>Bold or increased intensity
</td>
<td>As with faint, the color change is a PC (SCO / <a href="/wiki/Color_Graphics_Adapter" title="Color Graphics Adapter">CGA</a>) invention.<sup id="cite_ref-SCO_31-0" class="reference"><a href="#cite_note-SCO-31"></a></sup><sup class="noprint Inline-Template noprint noexcerpt Template-Fact" style="white-space:nowrap;"></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">2</span>
</td>
<td>Faint, decreased intensity, <i>or</i> dim
</td>
<td>May be implemented as a light <a href="/wiki/Font_weight" class="mw-redirect" title="Font weight">font weight</a> like bold.<sup id="cite_ref-32" class="reference"><a href="#cite_note-32"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">3</span>
</td>
<td>Italic
</td>
<td>Not widely supported. Sometimes treated as inverse or blink.<sup id="cite_ref-SCO_31-1" class="reference"><a href="#cite_note-SCO-31"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">4</span>
</td>
<td>Underline
</td>
<td>Style extensions exist for Kitty, VTE, mintty and iTerm2.<sup id="cite_ref-color-u_33-0" class="reference"><a href="#cite_note-color-u-33"></a></sup><sup id="cite_ref-color-u-kitty-spec_34-0" class="reference"><a href="#cite_note-color-u-kitty-spec-34"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">5</span>
</td>
<td>Slow blink
</td>
<td>Less than 150 per minute
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">6</span>
</td>
<td>Rapid blink
</td>
<td>MS-DOS ANSI.SYS, 150+ per minute; not widely supported
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">7</span>
</td>
<td><a href="/wiki/Reverse_video" title="Reverse video">Reverse video</a> <i>or</i> invert
</td>
<td>Swap foreground and background colors; inconsistent emulation<sup id="cite_ref-console-termio-realize_35-0" class="reference"><a href="#cite_note-console-termio-realize-35"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">8</span>
</td>
<td>Conceal <i>or</i> hide
</td>
<td>Not widely supported.
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">9</span>
</td>
<td><a href="/wiki/Strikethrough" title="Strikethrough">Crossed-out</a>, <i>or</i> strike
</td>
<td>Characters legible but marked as if for deletion.
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">10</span>
</td>
<td>Primary (default) font
</td>
<td>
</td></tr>
<tr>
<td><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886049734"><span class="monospaced">11–19</span>
</td>
<td>Alternative font
</td>
<td>Select alternative font <span class="texhtml"><i>n</i> − 10</span>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">20</span>
</td>
<td><a href="/wiki/Blackletter" title="Blackletter">Blackletter</a> font
</td>
<td>Rarely supported
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">21</span>
</td>
<td>Doubly underlined; or: not bold
</td>
<td>Double-underline per ECMA-48, but instead disables bold intensity on several terminals, including in the <a href="/wiki/Linux_kernel" title="Linux kernel">Linux kernel</a>'s <a href="/wiki/Linux_console" title="Linux console">console</a> before version 4.17.<sup id="cite_ref-36" class="reference"><a href="#cite_note-36"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">22</span>
</td>
<td>Normal intensity
</td>
<td>Neither bold nor faint; color changes where intensity is implemented as such.
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">23</span>
</td>
<td>Neither italic, nor blackletter
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">24</span>
</td>
<td>Not underlined
</td>
<td>Neither singly nor doubly underlined
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">25</span>
</td>
<td>Not blinking
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">26</span>
</td>
<td>Proportional spacing
</td>
<td><a href="/wiki/ITU_T.61" title="ITU T.61">ITU T.61</a> and T.416, not known to be used on terminals
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">27</span>
</td>
<td>Not reversed
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">28</span>
</td>
<td>Reveal
</td>
<td>Not concealed
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">29</span>
</td>
<td>Not crossed out
</td>
<td>
</td></tr>
<tr>
<td><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886049734"><span class="monospaced">30–37</span>
</td>
<td>Set foreground <a href="#3-bit_and_4-bit">color</a>
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">38</span>
</td>
<td>Set foreground <a href="#8-bit">color</a>
</td>
<td><a href="#8-bit">Next arguments are <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">5;n</code> or <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">2;r;g;b</code></a>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">39</span>
</td>
<td>Default foreground color
</td>
<td>Implementation defined (according to standard)
</td></tr>
<tr>
<td><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886049734"><span class="monospaced">40–47</span>
</td>
<td>Set background <a href="#3-bit_and_4-bit">color</a>
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">48</span>
</td>
<td>Set background <a href="#8-bit">color</a>
</td>
<td><a href="#8-bit">Next arguments are <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">5;n</code> or <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">2;r;g;b</code></a>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">49</span>
</td>
<td>Default background color
</td>
<td>Implementation defined (according to standard)
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">50</span>
</td>
<td>Disable proportional spacing
</td>
<td>T.61 and T.416
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">51</span>
</td>
<td>Framed
</td>
<td rowspan="2">Implemented as "<a href="/wiki/Variation_Selectors_(Unicode_block)" title="Variation Selectors (Unicode block)">emoji variation selector</a>" in mintty.<sup id="cite_ref-mintty_37-0" class="reference"><a href="#cite_note-mintty-37"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">52</span>
</td>
<td>Encircled
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">53</span>
</td>
<td>Overlined
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">54</span>
</td>
<td>Neither framed nor encircled
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">55</span>
</td>
<td>Not overlined
</td>
<td>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">58</span>
</td>
<td>Set underline <a href="#8-bit">color</a>
</td>
<td>Not in standard; implemented in Kitty, VTE, mintty, and iTerm2.<sup id="cite_ref-color-u_33-1" class="reference"><a href="#cite_note-color-u-33"></a></sup><sup id="cite_ref-color-u-kitty-spec_34-1" class="reference"><a href="#cite_note-color-u-kitty-spec-34"></a></sup> <a href="#8-bit">Next arguments are <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">5;n</code> or <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">2;r;g;b</code></a>.
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">59</span>
</td>
<td>Default underline color
</td>
<td>Not in standard; implemented in Kitty, VTE, mintty, and iTerm2.<sup id="cite_ref-color-u_33-2" class="reference"><a href="#cite_note-color-u-33"></a></sup><sup id="cite_ref-color-u-kitty-spec_34-2" class="reference"><a href="#cite_note-color-u-kitty-spec-34"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">60</span>
</td>
<td>Ideogram underline or right side line
</td>
<td rowspan="5">Rarely supported
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">61</span>
</td>
<td>Ideogram double underline, <i>or</i> double line on the right side
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">62</span>
</td>
<td>Ideogram overline or left side line
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">63</span>
</td>
<td>Ideogram double overline, <i>or</i> double line on the left side
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">64</span>
</td>
<td>Ideogram stress marking
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">65</span>
</td>
<td>No ideogram attributes
</td>
<td>Reset the effects of all of <code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">60</code>–<code class="mw-highlight mw-highlight-lang-text mw-content-ltr" id="" style="" dir="ltr">64</code>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">73</span>
</td>
<td>Superscript
</td>
<td rowspan="3">Implemented only in mintty<sup id="cite_ref-mintty_37-1" class="reference"><a href="#cite_note-mintty-37"></a></sup>
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">74</span>
</td>
<td>Subscript
</td></tr>
<tr>
<td style="width:15%"><span class="monospaced">75</span>
</td>
<td>Neither superscript nor subscript
</td></tr>
<tr>
<td><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886049734"><span class="monospaced">90–97</span>
</td>
<td>Set bright foreground color
</td>
<td rowspan="2">Not in standard; originally implemented by aixterm<sup id="cite_ref-xtc_23-3" class="reference"><a href="#cite_note-xtc-23"></a></sup>
</td></tr>
<tr>
<td><link rel="mw-deduplicated-inline-style" href="mw-data:TemplateStyles:r886049734"><span class="monospaced"><span class="nowrap">100–107</span></span>
</td>
<td>Set bright background color
</td></tr></tbody></table>