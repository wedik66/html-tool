---
title: Lint Rule html/useClosingNonVoid
layout: layouts/rule.liquid
showHero: false
description: MISSING DOCUMENTATION
eleventyNavigation:
  key: lint-rules/html/useClosingNonVoid
  parent: lint-rules
  title: html/useClosingNonVoid
---

# html/useClosingNonVoid

Close empty HTML elements with an XHTML closing tag.

<!-- GENERATED:START(hash:da39a3ee5e6b4b0d3255bfef95601890afd80709,id:description) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. -->

<!-- GENERATED:END(id:description) -->

<!-- GENERATED:START(hash:7bf2739b78a01a5c9fab186913f449c27a30a1f3,id:examples) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules-docs` to update. -->
## Examples


### Valid

{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span><span class="token punctuation">&gt;</span><span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">div</span><span class="token punctuation">&gt;</span>child<span class="token punctuation">&lt;/</span><span class="token attr-name">div</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span><span class="token punctuation">&gt;</span></code></pre>{% endraw %}
{% raw %}<pre class="language-html"><code class="language-html"><span class="token punctuation">&lt;</span><span class="token tag">input</span> <span class="token punctuation">/&gt;</span></code></pre>{% endraw %}
<!-- GENERATED:END(id:examples) -->
