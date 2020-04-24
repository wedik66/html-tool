# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `es2018 > async-generators > for-await-async-context`

```javascript
Program {
  comments: Array []
  corrupt: true
  directives: Array []
  filename: 'input.js'
  hasHoistedVars: false
  interpreter: undefined
  mtime: undefined
  sourceType: 'script'
  syntax: Array []
  loc: Object {
    filename: 'input.js'
    end: Object {
      column: 0
      index: 43
      line: 4
    }
    start: Object {
      column: 0
      index: 0
      line: 1
    }
  }
  diagnostics: Array [
    Object {
      origins: Array [Object {category: 'js-parser'}]
      description: Object {
        advice: Array []
        category: 'parse/js'
        message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: 'Unexpected token, expected ('}
      }
      location: Object {
        filename: 'input.js'
        mtime: undefined
        sourceType: 'script'
        end: Object {
          column: 11
          index: 26
          line: 2
        }
        start: Object {
          column: 6
          index: 21
          line: 2
        }
      }
    }
  ]
  body: Array [
    FunctionDeclaration {
      id: BindingIdentifier {
        name: 'f'
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 10
            index: 10
            line: 1
          }
          start: Object {
            column: 9
            index: 9
            line: 1
          }
        }
      }
      loc: Object {
        filename: 'input.js'
        end: Object {
          column: 1
          index: 42
          line: 3
        }
        start: Object {
          column: 0
          index: 0
          line: 1
        }
      }
      head: FunctionHead {
        async: false
        generator: false
        hasHoistedVars: false
        params: Array []
        predicate: undefined
        rest: undefined
        returnType: undefined
        thisType: undefined
        typeParameters: undefined
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 13
            index: 13
            line: 1
          }
          start: Object {
            column: 10
            index: 10
            line: 1
          }
        }
      }
      body: BlockStatement {
        directives: Array []
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 1
            index: 42
            line: 3
          }
          start: Object {
            column: 13
            index: 13
            line: 1
          }
        }
        body: Array [
          ForStatement {
            loc: Object {
              filename: 'input.js'
              end: Object {
                column: 23
                index: 38
                line: 2
              }
              start: Object {
                column: 2
                index: 17
                line: 2
              }
            }
            test: ReferenceIdentifier {
              name: 'x'
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 18
                  index: 33
                  line: 2
                }
                start: Object {
                  column: 17
                  index: 32
                  line: 2
                }
              }
            }
            update: ReferenceIdentifier {
              name: 'of'
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 21
                  index: 36
                  line: 2
                }
                start: Object {
                  column: 19
                  index: 34
                  line: 2
                }
              }
            }
            body: ExpressionStatement {
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 23
                  index: 38
                  line: 2
                }
                start: Object {
                  column: 22
                  index: 37
                  line: 2
                }
              }
              expression: ReferenceIdentifier {
                name: 'y'
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 23
                    index: 38
                    line: 2
                  }
                  start: Object {
                    column: 22
                    index: 37
                    line: 2
                  }
                }
              }
            }
            init: CallExpression {
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 16
                  index: 31
                  line: 2
                }
                start: Object {
                  column: 6
                  index: 21
                  line: 2
                }
              }
              callee: ReferenceIdentifier {
                name: 'await'
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 11
                    index: 26
                    line: 2
                  }
                  start: Object {
                    column: 6
                    index: 21
                    line: 2
                  }
                }
              }
              arguments: Array [
                ReferenceIdentifier {
                  name: 'let'
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 16
                      index: 31
                      line: 2
                    }
                    start: Object {
                      column: 13
                      index: 28
                      line: 2
                    }
                  }
                }
              ]
            }
          }
          ExpressionStatement {
            loc: Object {
              filename: 'input.js'
              end: Object {
                column: 25
                index: 40
                line: 2
              }
              start: Object {
                column: 23
                index: 38
                line: 2
              }
            }
            expression: ReferenceIdentifier {
              name: 'INVALID_PLACEHOLDER'
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 24
                  index: 39
                  line: 2
                }
                start: Object {
                  column: 23
                  index: 38
                  line: 2
                }
              }
            }
          }
        ]
      }
    }
  ]
}
```