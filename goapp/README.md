#Go
## By default, Go passes variables by value. However, you can pass a pointer to a value to avoid making a copy and to enable modifications to the original value.

### Struct init
`node := &Node{value: 10, next: nil}`
### constructor/fucntion call
Go does not have constructor thing
`node := newNode(10)`

### Types of Receivers:
- Pointer Receiver (*Type): Used when the method needs to modify the receiver or when you want to avoid copying large structs.
- Value Receiver (Type): Used when you don't need to modify the receiver, and it can work with a copy of the value.
#### Example Comparison:
``` go
type LinkedList struct {
    size int
}

// Method with a pointer receiver
func (ll *LinkedList) Increment() {
    ll.size++ // Modifies the original LinkedList instance
}

// Method with a value receiver
func (ll LinkedList) Display() {
    fmt.Println(ll.size) // Works with a copy, original is not modified
}
// Why Use Pointer Receivers?
//     To allow modification of the original struct.
//     To avoid copying large structs when calling methods.
//     The method receiver is what makes Go's structs similar to classes in object-oriented programming, as it associates methods with data types.