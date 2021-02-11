# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/css-parser/index.test.ts --update-snapshots` to update.

## `custom-properties`

```javascript
CSSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	integrity: undefined
	loc: SourceLocation custom-properties/input.css 1:0-6:1
	path: RelativeFilePath<custom-properties/input.css>
	body: Array [
		CSSRule {
			loc: SourceLocation custom-properties/input.css 1:0-6:1
			prelude: Array [
				CSSSelector {
					loc: SourceLocation custom-properties/input.css 1:0-1:7
					patterns: Array [
						CSSClassSelector {
							value: "style"
							loc: SourceLocation custom-properties/input.css 1:0-1:6
						}
					]
				}
			]
			block: CSSBlock {
				value: Array [
					CSSDeclaration {
						name: CSSCustomProperty {
							value: "--foo"
							loc: SourceLocation custom-properties/input.css 2:1-2:1
						}
						value: Array [
							CSSString {
								value: "bar"
								loc: SourceLocation custom-properties/input.css 2:8-2:13
							}
						]
						important: false
						loc: SourceLocation custom-properties/input.css 2:1-2:13
					}
					CSSDeclaration {
						name: CSSCustomProperty {
							value: "--lore-ipsum"
							loc: SourceLocation custom-properties/input.css 3:1-3:1
						}
						value: Array [
							CSSString {
								value: "foo"
								loc: SourceLocation custom-properties/input.css 3:15-3:20
							}
						]
						important: false
						loc: SourceLocation custom-properties/input.css 3:1-3:20
					}
					CSSDeclaration {
						name: CSSCustomProperty {
							value: "--FANCY"
							loc: SourceLocation custom-properties/input.css 4:1-4:1
						}
						value: Array [
							CSSString {
								value: "abort"
								loc: SourceLocation custom-properties/input.css 4:10-4:17
							}
						]
						important: false
						loc: SourceLocation custom-properties/input.css 4:1-4:17
					}
					CSSDeclaration {
						name: CSSCustomProperty {
							value: "--test"
							loc: SourceLocation custom-properties/input.css 5:1-5:1
						}
						value: Array [
							CSSNumber {
								value: 1_987
								raw: "1987"
								loc: SourceLocation custom-properties/input.css 5:9-5:13
							}
						]
						important: false
						loc: SourceLocation custom-properties/input.css 5:1-5:13
					}
				]
				startingTokenValue: "{"
				loc: SourceLocation custom-properties/input.css 1:7-6:1
			}
		}
	]
}
```