#Go
## By default, Go passes variables by value. However, you can pass a pointer to a value to avoid making a copy and to enable modifications to the original value.

### Struct init
`node := &Node{value: 10, next: nil}`
### constructor/fucntion call
Go does not have constructor thing
`node := newNode(10)`