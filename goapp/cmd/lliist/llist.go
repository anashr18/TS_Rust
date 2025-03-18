package main

import "fmt"

// Node represents a single element in the linked list
type Node[T any] struct {
	value T
	next  *Node[T]
}

// LinkedList represents the linked list structure
type LinkedList[T any] struct {
	head   *Node[T]
	tail   *Node[T]
	length int
}

// Create a new node with the given value
func newNode[T any](value T) *Node[T] {
	return &Node[T]{value: value, next: nil}
}

// Create a new linked list with an initial value
func newLinkedList[T any](value T) *LinkedList[T] {
	node := newNode(value)
	return &LinkedList[T]{head: node, tail: node, length: 1}
}

// Print the linked list
func (ll *LinkedList[T]) print_list() {
	if ll.head == nil {
		fmt.Println("The list is empty.")
		return
	}
	current := ll.head
	for current != nil {
		fmt.Print(current.value, " -> ")
		current = current.next
	}
	fmt.Println("nil")
}

// Append a value to the end of the linked list
func (ll *LinkedList[T]) append(value T) {
	newNode := newNode(value)
	if ll.head == nil {
		ll.head = newNode
		ll.tail = newNode
		ll.length++
		return
	}
	ll.tail.next = newNode
	ll.tail = newNode
	ll.length++
}

// Prepend a value to the start of the linked list
func (ll *LinkedList[T]) prepend(value T) {
	newNode := newNode(value)
	if ll.head == nil {
		ll.head = newNode
		ll.tail = newNode
		ll.length++
		return
	}
	newNode.next = ll.head
	ll.head = newNode
	ll.length++
}

// Delete the last node in the linked list
func (ll *LinkedList[T]) delete_last() {
	if ll.head == nil {
		return
	}
	if ll.head.next == nil { // Single-node list
		ll.head = nil
		ll.tail = nil
		ll.length--
		return
	}
	current := ll.head
	for current.next != ll.tail {
		current = current.next
	}
	current.next = nil
	ll.tail = current
	ll.length--
}

// Delete the first node in the linked list
func (ll *LinkedList[T]) delete_first() {
	if ll.head == nil {
		return
	}
	ll.head = ll.head.next
	if ll.head == nil {
		ll.tail = nil
	}
	ll.length--
}

func (ll *LinkedList[T]) get(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= ll.length {
		return zero, false
	}
	current := ll.head
	for i := 0; i < idx; i++ {
		current = current.next
	}
	return current.value, true

}
func (ll *LinkedList[T]) set(idx int, value T) error {
	// list check
	if ll.head == nil {
		// fmt.Println("list is empty")
		return fmt.Errorf("list is empty")
	}
	// index check
	if idx < 0 || idx >= ll.length {
		// fmt.Println("wrong index")
		return fmt.Errorf("wrong index")
		// return
	}
	current := ll.head
	for i := 0; i < idx; i++ {
		current = current.next
	}
	current.value = value
	// ll.print_list()
	return nil
}
func main() {
	// Example: LinkedList of integers
	intList := newLinkedList(10)
	intList.append(20)
	intList.append(30)
	intList.print_list() // Output: 10 -> 20 -> 30 -> nil

	intList.delete_first()
	intList.print_list() // Output: 20 -> 30 -> nil

	intList.delete_last()
	intList.print_list() // Output: 20 -> nil

	// Example: LinkedList of strings
	stringList := newLinkedList("hello")
	stringList.append("world")
	stringList.prepend("greetings")
	stringList.print_list() // Output: greetings -> hello -> world -> nil

	idx := 0
	value, ok := stringList.get(idx)
	if ok {
		fmt.Printf("value at index %d: %v\n ", idx, value)
	}
	stringList.print_list()
	if err := stringList.set(5, "Testing"); err != nil {
		fmt.Println("Error:: ", err)
	} else {
		stringList.print_list()
	}
	// stringList.print_list()
}
