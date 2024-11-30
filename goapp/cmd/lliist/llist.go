package main

import "fmt"

type Node struct {
	value int
	next  *Node
}

func newNode(value int) *Node {
	return &Node{value: value, next: nil}
}

type LinkedList struct {
	head   *Node
	tail   *Node
	length int
}

func newLinkedList(value int) *LinkedList {
	node := newNode(value)
	return &LinkedList{head: node, tail: node, length: 1}
}

func main() {
	ll := newLinkedList(4) // Initialize the linked list with a value of 4

	// Print details of the linked list
	fmt.Println("Head Value:", ll.head.value)
	fmt.Println("Tail Value:", ll.tail.value)
	fmt.Println("Length:", ll.length)
}
