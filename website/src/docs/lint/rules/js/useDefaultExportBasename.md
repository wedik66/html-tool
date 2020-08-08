<<<<<<< HEAD:website/src/docs/lint/rules/js/defaultExportSameBasename.md
---
title: Lint Rule js/defaultExportSameBasename
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/defaultExportSameBasename
	parent: lint-rules
	title: js/defaultExportSameBasename
---

# js/defaultExportSameBasename

=======
---
title: Lint Rule js/useDefaultExportBasename
layout: layouts/rule.liquid
description: MISSING DOCUMENTATION
eleventyNavigation:
	key: lint-rules/js/useDefaultExportBasename
	parent: lint-rules
	title: js/useDefaultExportBasename
---

# js/useDefaultExportBasename

>>>>>>> feat: consolidate lint rule naming:website/src/docs/lint/rules/js/useDefaultExportBasename.md
MISSING DOCUMENTATION

<!-- GENERATED:START(hash:551d2aaf8732003d89dcb8332511c9d941e3e9d3,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples
### Invalid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token variable">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">foo.ts:1:24</span> <strong>lint/js/useDefaultExportBasename</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The filename and the name of a default function should match.</span>

    <span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token variable">test</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
                            <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The filename should be </span><span style="color: DodgerBlue;"><strong>test.ts</strong></span><span style="color: DodgerBlue;"> or the function name should be </span><span style="color: DodgerBlue;"><strong>foo</strong></span><span style="color: DodgerBlue;">.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

</code></pre>{% endraw %}

---------------

{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text">
 <span style="text-decoration-style: dashed; text-decoration-line: underline;">foo.ts:1:21</span> <strong>lint/js/useDefaultExportBasename</strong> <span style="color: #000; background-color: #ddd;"> FIXABLE </span> ━━━━━━━━━━━━━━━━━━━

  <strong><span style="color: Tomato;">✖ </span></strong><span style="color: Tomato;">The filename and the name of a default class should match.</span>

    <span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Test</span> <span class="token punctuation">{</span><span class="token punctuation">}</span>
                         <span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span><span style="color: Tomato;"><strong>^</strong></span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">The filename should be </span><span style="color: DodgerBlue;"><strong>Test.ts</strong></span><span style="color: DodgerBlue;"> or the class name should be </span><span style="color: DodgerBlue;"><strong>Foo</strong></span><span style="color: DodgerBlue;">.</span>

  <strong><span style="color: DodgerBlue;">ℹ </span></strong><span style="color: DodgerBlue;">Safe fix</span>

</code></pre>{% endraw %}
### Valid
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">function</span> <span class="token variable">foo</span><span class="token punctuation">(</span><span class="token punctuation">)</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token keyword">class</span> <span class="token variable">Foo</span> <span class="token punctuation">{</span><span class="token punctuation">}</span></code></pre>{% endraw %}
{% raw %}<pre class="language-text"><code class="language-text"><span class="token keyword">export</span> <span class="token keyword">default</span> <span class="token string">&apos;rome&apos;</span><span class="token punctuation">;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:main) -->