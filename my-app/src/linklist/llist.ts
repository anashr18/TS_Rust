class Node<T> {
    private _value: T;
    private _next: Node<T> | null;

    constructor(value: T) {
        this._value = value;
        this._next = null;
    }

    get value(): T {
        return this._value;
    }
    set value(value: T) {
        this._value = value;
    }
    get next(): Node<T> | null {
        return this._next;
    }
    set next(next: Node<T> | null) {
        this._next = next;
    }
}

class LinkedList<T> {
    private head: Node<T> | null;
    private tail: Node<T> | null;
    private length: number;

    constructor(value: T) {
        const node = new Node<T>(value);
        this.head = node;
        this.tail = node;
        this.length = 1;
    }

    printList(): void {
        let current = this.head;
        const values: T[] = [];
        while (current) {
            values.push(current.value);
            current = current.next;
        }
        console.log(values.join('->'));
    }

    append(value: T): void {
        let new_node = new Node(value);
        if (!this.head) {
            this.head = new_node;
            this.tail = new_node;
        }
        else if (this.tail) {
            this.tail.next = new_node;
            this.tail = new_node;
        }
        this.length += 1;
    }

    prepend(value: T): void {
        let new_node = new Node(value);
        if (!this.head) {
            this.head = new_node;
            this.tail = new_node;
        }
        else {
            new_node.next = this.head;
            this.head = new_node;
        }
        this.length += 1;
    }
    delete_last(): void {
        if (!this.head) {
            return;
        }
        if (!this.head.next) {
            this.head = null;
            this.tail = null;
        }
        else {
            let current = this.head;
            while (current.next && current.next.next) {
                current = current.next;
            }
            current.next = null;
            this.tail = current;
        }
        this.length -= 1;
    }
    get(idx: number): void {
        if (idx < 0 || idx >= this.length) {
            console.log("wrong index")
            return;
        }
        let current = this.head;
        for (let i = 0; i < idx && current; i++) {
            current = current.next;
        }
        console.log(current ? current.value : null)
    }
    set(idx: number, value: T): void {
        if (idx < 0 || idx >= this.length) {
            console.log("wrong index");
            return;
        }
        let current = this.head;
        for (let i = 0; i < idx && current; i++) {
            current = current.next;
        }
        if (current) {
            current.value = value;
        }
        else {
            return;
        }
        this.printList();
    }
}

const ll = new LinkedList<number>(4);
ll.append(7);
ll.append(10);
ll.prepend(123)
ll.printList(); // Output: 4 -> 7 -> 10
ll.get(0);
ll.get(2);
ll.get(20);
ll.set(1, 999);
ll.set(0, 10000);
ll.set(3, 888);
ll.set(2, 77888);
